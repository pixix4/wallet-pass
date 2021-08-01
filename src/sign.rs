use fs_extra::dir::CopyOptions;
use openssl::sha::sha1;
use openssl::stack::Stack;
use openssl::x509::X509;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tempfile::tempdir;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

struct Sign {
    pass_path: PathBuf,
    certificate_path: PathBuf,
    certificate_password: String,
    wwdr_intermediate_certificate_path: PathBuf,
    output_path: PathBuf,
    temporary_path: PathBuf,
    manifest_path: PathBuf,
    signature_path: PathBuf,
}

pub fn sign_to_file(
    pass_path: &Path,
    certificate_path: &Path,
    certificate_password: &str,
    wwdr_intermediate_certificate_path: &Path,
    output_path: &Path,
    force_pass_signing: bool,
) -> io::Result<()> {
    let mut sign = Sign::create(
        pass_path,
        certificate_path,
        certificate_password,
        wwdr_intermediate_certificate_path,
        output_path,
    );
    sign.sign_pass(force_pass_signing)
}

impl Sign {
    pub fn create(
        pass_path: &Path,
        certificate_path: &Path,
        certificate_password: &str,
        wwdr_intermediate_certificate_path: &Path,
        output_path: &Path,
    ) -> Self {
        Self {
            pass_path: pass_path.to_path_buf(),
            certificate_path: certificate_path.to_path_buf(),
            certificate_password: certificate_password.to_string(),
            wwdr_intermediate_certificate_path: wwdr_intermediate_certificate_path.to_path_buf(),
            output_path: output_path.to_path_buf(),
            temporary_path: PathBuf::new(),
            manifest_path: PathBuf::new(),
            signature_path: PathBuf::new(),
        }
    }

    pub fn sign_pass(&mut self, force_pass_signing: bool) -> io::Result<()> {
        if force_pass_signing {
            self.force_clean_raw_pass()?;
        }

        // Validate that requested contents are not a signed and expanded pass archive.
        self.validate_directory_as_unsigned_raw_pass()?;

        // Get a temporary place to stash the pass contents
        self.create_temporary_directory()?;

        // Make a copy of the pass contents to the temporary folder
        self.copy_pass_to_temporary_location()?;

        // Clean out the unneeded .DS_Store files
        self.clean_ds_store_files()?;

        // Build the json manifest
        self.generate_json_manifest()?;

        // Sign the manifest
        self.sign_manifest()?;

        // Package pass
        self.compress_pass_file()?;

        // Clean up the temp directory
        self.delete_temp_dir()?;

        Ok(())
    }

    fn validate_directory_as_unsigned_raw_pass(&self) -> io::Result<()> {
        let has_manifest_file = Path::new(&self.pass_path).join("manifest.json").exists();
        let has_signature_file = Path::new(&self.pass_path).join("signature").exists();

        if has_manifest_file || has_signature_file {
            eprintln!(
                "{:?} contains pass signing artificats that need to be removed before signing.",
                &self.pass_path
            );
            return Err(io::ErrorKind::AlreadyExists.into());
        }

        Ok(())
    }

    fn force_clean_raw_pass(&self) -> io::Result<()> {
        let manifest_file = Path::new(&self.pass_path).join("manifest.json");
        if manifest_file.exists() {
            fs::remove_file(manifest_file)?;
        }

        let signature_file = Path::new(&self.pass_path).join("signature");
        if signature_file.exists() {
            fs::remove_file(signature_file)?;
        }

        Ok(())
    }

    fn create_temporary_directory(&mut self) -> io::Result<()> {
        self.temporary_path = tempdir()?.into_path();

        Ok(())
    }

    fn copy_pass_to_temporary_location(&self) -> io::Result<()> {
        let mut options = CopyOptions::new();
        options.content_only = true;

        if fs_extra::dir::copy(&self.pass_path, &self.temporary_path, &options).is_err() {
            return Err(io::ErrorKind::Other.into());
        }

        Ok(())
    }

    fn clean_ds_store_files(&self) -> io::Result<()> {
        for entry in WalkDir::new(&self.temporary_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == ".DS_Store" {
                fs::remove_file(entry.path())?;
            }
        }

        Ok(())
    }

    fn generate_json_manifest(&mut self) -> io::Result<()> {
        let mut manifest = HashMap::<String, String>::new();

        for entry in WalkDir::new(&self.temporary_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            let file = File::open(entry.path())?;
            let mut file_reader = BufReader::new(file);
            let mut file_buffer = Vec::new();
            file_reader.read_to_end(&mut file_buffer)?;

            let digest = sha1(&file_buffer);

            manifest.insert(
                entry.file_name().to_str().unwrap().to_owned(),
                hex::encode(digest),
            );
        }

        self.manifest_path = self.temporary_path.clone();
        self.manifest_path.push("manifest.json");

        let mut manifest_file = File::create(&self.manifest_path)?;
        manifest_file.write_all(&serde_json::to_string_pretty(&manifest)?.as_bytes())?;

        Ok(())
    }

    fn sign_manifest(&mut self) -> io::Result<()> {
        let pkcs12_file = fs::File::open(&self.certificate_path)?;
        let mut pkcs12_reader = BufReader::new(pkcs12_file);
        let mut pkcs12_buffer = Vec::new();
        pkcs12_reader.read_to_end(&mut pkcs12_buffer)?;
        let pkcs12_certificate =
            openssl::pkcs12::Pkcs12::from_der(&pkcs12_buffer)?.parse(&self.certificate_password)?;

        let x509_file = fs::File::open(&self.wwdr_intermediate_certificate_path)?;
        let mut x509_reader = BufReader::new(x509_file);
        let mut x509_buffer = Vec::new();
        x509_reader.read_to_end(&mut x509_buffer)?;
        let x509_certificate = openssl::x509::X509::from_pem(&x509_buffer)?;

        let flags = openssl::pkcs7::Pkcs7Flags::BINARY | openssl::pkcs7::Pkcs7Flags::DETACHED;

        let manifest_file = fs::File::open(&self.manifest_path)?;
        let mut manifest_reader = BufReader::new(manifest_file);
        let mut manifest_buffer = Vec::new();
        manifest_reader.read_to_end(&mut manifest_buffer)?;

        let mut certs = Stack::<X509>::new()?;
        certs.push(x509_certificate)?;

        let signed = openssl::pkcs7::Pkcs7::sign(
            &pkcs12_certificate.cert,
            &pkcs12_certificate.pkey,
            &certs,
            &manifest_buffer,
            flags,
        )?;

        self.signature_path = self.temporary_path.clone();
        self.signature_path.push("signature");

        let mut signature_file = File::create(&self.signature_path)?;
        signature_file.write_all(&signed.to_der()?)?;

        Ok(())
    }

    fn compress_pass_file(&self) -> io::Result<()> {
        let src_dir = &self.temporary_path;
        let dst_file = &self.output_path;

        if !Path::new(src_dir).is_dir() {
            return Err(ZipError::FileNotFound.into());
        }

        let path = Path::new(dst_file);
        let file = File::create(&path).unwrap();

        let walkdir = WalkDir::new(src_dir);
        let it = walkdir.into_iter();

        zip_dir(
            &mut it.filter_map(|e| e.ok()),
            src_dir,
            file,
            zip::CompressionMethod::Deflated,
        )?;

        Ok(())
    }

    fn delete_temp_dir(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.temporary_path)
    }
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

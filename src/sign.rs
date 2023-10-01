use fs_extra::dir::CopyOptions;
use openssl::sha::sha1;
use openssl::stack::Stack;
use openssl::x509::X509;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tempfile::tempdir;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use crate::template::Template;

/// Sign pass with certificates
pub fn sign_path<T, P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
    pass_path: P1,
    template: Option<&Template>,
    certificate_path: P2,
    certificate_password: &str,
    wwdr_intermediate_certificate_path: P3,
    writer: T,
    force_pass_signing: bool,
) -> io::Result<T>
where
    T: Write + Seek,
{
    if force_pass_signing {
        force_clean_raw_pass(&pass_path)?;
    }

    // Validate that requested contents are not a signed and expanded pass archive.
    validate_directory_as_unsigned_raw_pass(&pass_path)?;

    // Get a temporary place to stash the pass contents
    let temporary_path = create_temporary_directory()?;

    // Make a copy of the pass contents to the temporary folder
    copy_pass_to_temporary_location(&pass_path, &temporary_path)?;

    if let Some(template) = template {
        save_pass_file(template, &temporary_path)?;
    }

    // Clean out the unneeded .DS_Store files
    clean_ds_store_files(&temporary_path)?;

    // Build the json manifest
    let manifest_path = generate_json_manifest(&temporary_path)?;

    // Sign the manifest
    sign_manifest(
        certificate_path,
        certificate_password,
        wwdr_intermediate_certificate_path,
        &temporary_path,
        &manifest_path,
    )?;

    // Package pass
    let writer = compress_pass(&temporary_path, writer)?;

    // Clean up the temp directory
    delete_temp_dir(&temporary_path)?;

    Ok(writer)
}

/// Validate that requested contents are not a signed and expanded pass archive.
fn validate_directory_as_unsigned_raw_pass<P: AsRef<Path>>(pass_path: P) -> io::Result<()> {
    let has_manifest_file = pass_path.as_ref().join("manifest.json").exists();
    let has_signature_file = pass_path.as_ref().join("signature").exists();

    if has_manifest_file || has_signature_file {
        eprintln!(
            "{:?} contains pass signing artificats that need to be removed before signing.",
            pass_path.as_ref()
        );
        return Err(io::ErrorKind::AlreadyExists.into());
    }

    Ok(())
}

/// Remove `manifest.json` and `signature` if they exist
fn force_clean_raw_pass<P: AsRef<Path>>(pass_path: P) -> io::Result<()> {
    let manifest_file = pass_path.as_ref().join("manifest.json");
    if manifest_file.exists() {
        fs::remove_file(manifest_file)?;
    }

    let signature_file = pass_path.as_ref().join("signature");
    if signature_file.exists() {
        fs::remove_file(signature_file)?;
    }

    Ok(())
}

/// Get a temporary place to stash the pass contents
fn create_temporary_directory() -> io::Result<PathBuf> {
    Ok(tempdir()?.into_path())
}

/// Make a copy of the pass contents to the temporary folder
fn copy_pass_to_temporary_location<P1: AsRef<Path>, P2: AsRef<Path>>(
    pass_path: P1,
    temporary_path: P2,
) -> io::Result<()> {
    let mut options = CopyOptions::new();
    options.content_only = true;

    if fs_extra::dir::copy(pass_path, temporary_path, &options).is_err() {
        return Err(io::ErrorKind::Other.into());
    }

    Ok(())
}

/// Load given `Template` and write content to `pass.json`
fn save_pass_file<P: AsRef<Path>>(template: &Template, temporary_path: P) -> io::Result<()> {
    let pass_path = temporary_path.as_ref().join("pass.json");

    let mut pass_file = File::create(&pass_path)?;
    pass_file.write_all(&serde_json::to_vec_pretty(template)?)?;

    Ok(())
}

/// Clean out the unneeded .DS_Store files
fn clean_ds_store_files<P: AsRef<Path>>(temporary_path: P) -> io::Result<()> {
    for entry in WalkDir::new(temporary_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == ".DS_Store" {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

/// Build the json manifest
fn generate_json_manifest<P: AsRef<Path>>(temporary_path: P) -> io::Result<PathBuf> {
    let mut manifest = HashMap::<String, String>::new();

    for entry in WalkDir::new(&temporary_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let file = File::open(entry.path())?;
        let mut file_reader = BufReader::new(file);
        let mut file_buffer = Vec::new();
        file_reader.read_to_end(&mut file_buffer)?;

        let digest = sha1(&file_buffer);

        let name = entry
            .path()
            .strip_prefix(&temporary_path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .to_str()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "Could not convert path to string!".to_string(),
                )
            })?
            .to_owned();
        manifest.insert(name, hex::encode(digest));
    }

    let manifest_path = temporary_path.as_ref().join("manifest.json");

    let mut manifest_file = File::create(&manifest_path)?;
    manifest_file.write_all(serde_json::to_string_pretty(&manifest)?.as_bytes())?;

    Ok(manifest_path)
}

/// Sign the manifest
fn sign_manifest<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>, P4: AsRef<Path>>(
    certificate_path: P1,
    certificate_password: &str,
    wwdr_intermediate_certificate_path: P2,
    temporary_path: P3,
    manifest_path: P4,
) -> io::Result<PathBuf> {
    let pkcs12_file = fs::File::open(certificate_path)?;
    let mut pkcs12_reader = BufReader::new(pkcs12_file);
    let mut pkcs12_buffer = Vec::new();
    pkcs12_reader.read_to_end(&mut pkcs12_buffer)?;
    let pkcs12_certificate =
        openssl::pkcs12::Pkcs12::from_der(&pkcs12_buffer)?.parse2(certificate_password)?;

    let x509_file = fs::File::open(wwdr_intermediate_certificate_path)?;
    let mut x509_reader = BufReader::new(x509_file);
    let mut x509_buffer = Vec::new();
    x509_reader.read_to_end(&mut x509_buffer)?;
    let x509_certificate = openssl::x509::X509::from_pem(&x509_buffer)?;

    let flags = openssl::pkcs7::Pkcs7Flags::BINARY | openssl::pkcs7::Pkcs7Flags::DETACHED;

    let manifest_file = fs::File::open(manifest_path)?;
    let mut manifest_reader = BufReader::new(manifest_file);
    let mut manifest_buffer = Vec::new();
    manifest_reader.read_to_end(&mut manifest_buffer)?;

    let mut certs = Stack::<X509>::new()?;
    certs.push(x509_certificate)?;

    let signed = openssl::pkcs7::Pkcs7::sign(
        &pkcs12_certificate.cert.as_ref().unwrap(),
        &pkcs12_certificate.pkey.as_ref().unwrap(),
        &certs,
        &manifest_buffer,
        flags,
    )?;

    let signature_path = temporary_path.as_ref().join("signature");

    let mut signature_file = File::create(&signature_path)?;
    signature_file.write_all(&signed.to_der()?)?;

    Ok(signature_path)
}

/// Package pass
fn compress_pass<T, P: AsRef<Path>>(temporary_path: P, writer: T) -> io::Result<T>
where
    T: Write + Seek,
{
    if !temporary_path.as_ref().is_dir() {
        return Err(ZipError::FileNotFound.into());
    }

    let walkdir = WalkDir::new(&temporary_path);
    let it = walkdir.into_iter();

    let writer = zip_dir(
        &mut it.filter_map(|e| e.ok()),
        &temporary_path,
        writer,
        zip::CompressionMethod::Deflated,
    )?;

    Ok(writer)
}

/// Clean up the temp directory
fn delete_temp_dir<P: AsRef<Path>>(temporary_path: P) -> io::Result<()> {
    fs::remove_dir_all(temporary_path)
}

/// Utility function for `compress_pass_file`
fn zip_dir<T, P: AsRef<Path>>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: P,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<T>
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
        let name = path
            .strip_prefix(&prefix)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

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
    let writer = zip.finish()?;
    Ok(writer)
}

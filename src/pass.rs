use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, Write},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::{sign, template::Template};

/// Represents an complete pass with reference to a directory with image and resource files
#[derive(Debug, Clone)]
pub struct Pass {
    /// Reference to a directory with image and resource files
    pass_path: PathBuf,
    /// Reference to the `Template` instance
    pub template: Template,
}

impl Pass {
    /// Parse a `Pass` instance from the given directory
    pub fn from_path(pass_path: &Path) -> io::Result<Self> {
        let mut path_buf = pass_path.to_path_buf();
        path_buf.push("pass.json");

        let file = File::open(&path_buf).unwrap();
        let mut file_reader = BufReader::new(file);
        let mut file_buffer = Vec::new();
        file_reader.read_to_end(&mut file_buffer).unwrap();
        let template: crate::template::Template = serde_json::from_slice(&file_buffer)?;

        Ok(Self {
            pass_path: pass_path.to_path_buf(),
            template,
        })
    }

    /// Create a `Pass` instance from the given `Template` and an reference to a directory with image and resource files
    pub fn from_template(template: &Template, pass_path: &Path) -> Self {
        Self {
            pass_path: pass_path.to_path_buf(),
            template: template.clone(),
        }
    }

    /// Sign, package and save this `Pass` to writer
    pub fn export<T>(
        &self,
        certificate_path: &Path,
        certificate_password: &str,
        wwdr_intermediate_certificate_path: &Path,
        writer: T,
    ) -> io::Result<T>
    where
        T: Write + Seek,
    {
        sign::sign_path(
            &self.pass_path,
            Some(&self.template),
            certificate_path,
            certificate_password,
            wwdr_intermediate_certificate_path,
            writer,
            false,
        )
    }

    /// Sign, package and save this `Pass` to a file
    pub fn export_to_file(
        &self,
        certificate_path: &Path,
        certificate_password: &str,
        wwdr_intermediate_certificate_path: &Path,
        output_path: &Path,
    ) -> io::Result<()> {
        let path = Path::new(output_path);
        let file = File::create(&path).unwrap();
        self.export(
            certificate_path,
            certificate_password,
            wwdr_intermediate_certificate_path,
            file,
        )?;

        Ok(())
    }
}

impl Deref for Pass {
    type Target = Template;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

impl DerefMut for Pass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.template
    }
}

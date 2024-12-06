use std::ffi::OsStr;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod file_dialog;
pub mod image;
pub mod raw;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OutputParams {
    pub path: String,
    pub file_name: String,
    pub format: FileFormat,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum FileFormat {
    JPEG,
    #[default]
    TIFF,
    OpenEXR,
}

impl FileFormat {
    pub fn to_extension(&self) -> &str {
        match self {
            FileFormat::JPEG => "jpg",
            FileFormat::TIFF => "tif",
            FileFormat::OpenEXR => "exr",
        }
    }
}

impl From<&OsStr> for FileFormat {
    fn from(value: &OsStr) -> Self {
        match value.to_str().unwrap() {
            "jpg" | "jpeg" => Self::JPEG,
            "tif" | "tiff" => Self::TIFF,
            "exr" => Self::OpenEXR,
            _ => panic!("Unsupported file format"),
        }
    }
}

impl From<PathBuf> for OutputParams {
    fn from(value: PathBuf) -> Self {
        let file_name = value.file_name().unwrap_or("Stacked".as_ref());
        let extension = value.extension().unwrap_or("tif".as_ref());
        let directory = if value.is_dir() {
            value.as_os_str()
        } else {
            value
                .parent()
                .map(|value| value.as_os_str())
                .unwrap_or("$HOME".as_ref())
        };

        let suffix = format!(".{}", extension.to_str().unwrap());

        Self {
            path: directory.to_str().unwrap().to_string(),
            file_name: file_name
                .to_str()
                .unwrap()
                .to_string()
                .strip_suffix(&suffix)
                .unwrap()
                .to_string(),
            format: extension.into(),
        }
    }
}

impl From<String> for OutputParams {
    fn from(value: String) -> Self {
        PathBuf::from(value).into()
    }
}

impl From<&OutputParams> for PathBuf {
    fn from(value: &OutputParams) -> PathBuf {
        format!(
            "{}/{}.{}",
            value.path,
            value.file_name,
            value.format.to_extension()
        )
        .parse()
        .unwrap()
    }
}

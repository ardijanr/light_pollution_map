use serde::{Deserialize, Serialize};
pub mod dir_download;
pub mod hdf5_download;

pub use hdf5_download::download;

#[derive(Serialize, Deserialize, Debug)]
struct Placeholder {
    content: Vec<FileEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    name: String,
    downloadsLink: String,
}

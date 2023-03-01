use serde::{Deserialize, Serialize};
pub mod dir_download;
pub mod hdf5_download;

pub use hdf5_download::download;

#[derive(Serialize, Deserialize, Debug)]
struct Placeholder {
    content: Vec<RemoteFileEntry>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteFileEntry {
    name: String,
    downloadsLink: String,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    local_path: String,
    name: String,
    pub download_link: String,
}

impl FileEntry {
    pub fn tif_name(&self) -> String {
        format!("{}.tif", self.name)
    }

    pub fn hdf5_name(&self) -> String {
        format!("{}.h5", self.name)
    }

    pub fn tif_path(&self) -> String {
        format!("{}/{}.tif", self.local_path, self.name)
    }

    pub fn hdf5_path(&self) -> String {
        format!("{}/{}.h5", self.local_path, self.name)
    }
}

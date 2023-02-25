use reqwest::Client;
use tokio::task::JoinSet;

use super::{hdf5_download, FileEntry, Placeholder};

//TODO remove this!!!
use crate::{convert::hdf5_file_to_geotiff, DOWNLOAD_DIR, TOKEN};

//Number of parallel downloads
const PD: usize = 20;

pub async fn list_dir_content(client: Client) -> Result<Vec<FileEntry>, reqwest::Error> {
    let resp = client.get("https://ladsweb.modaps.eosdis.nasa.gov/api/v2/content/details/allData/5000/VNP46A1/2012/19")
    .header("Authorization", format!("Bearer {TOKEN}"))
    .header("X-Requested-With","XMLHttpRequest")
    .send()
    .await?
    .json::<Placeholder>()
    .await?;

    // Rename file since gdal will crash if multiple "." exist in the filename
    let files = resp
        .content
        .iter()
        .map(|f| -> Option<FileEntry> {
            let s = f.name.split(".").collect::<Vec<&str>>();

            // Can't declare last before this since it may fail if len = 0
            if s.len() < 2 || s[s.len()-1]!="h5"{
                return None;
            }

            let last = s.len() - 1;

            let mut name = s[..last].join("_");
            name.push_str(s[last]);

            Some(FileEntry {
                name: name,
                downloadsLink: f.downloadsLink.clone(),
            })
        })
        .collect::<Vec<Option<FileEntry>>>();

    Ok(files.into_iter().filter_map(|x| x ).collect::<Vec<FileEntry>>())
}

// If some files failed the download by this point there is nothing we can do
//
pub async fn files_download_convert_delete(files: Vec<FileEntry>, client: Client) {
    for file in files {
        if let Ok(downloaded) = hdf5_download::download(file, DOWNLOAD_DIR, client.clone()).await {
            if hdf5_file_to_geotiff(&downloaded).await.is_ok() {
                let _ = std::fs::remove_file(&downloaded);
            }
        }
    }
}

pub async fn download_dir_content(client: Client) {
    let files = list_dir_content(client.clone()).await.unwrap();

    let mut set = JoinSet::new();

    let a: Vec<_> = files
        .chunks(files.len() / PD)
        .map(|chunk| {
            set.spawn(files_download_convert_delete(
                chunk.to_owned(),
                client.to_owned(),
            ))
        })
        .collect();

    //Wait for downloads and geotiff generation to complete
    while let Some(_) = set.join_next().await {}


    // Validate that all files are downloaded if not display whats missing
    let failed = files.iter().filter(|f| {
        f.name
    });


    // Generate one geotiff from the resulting geotiffs

}

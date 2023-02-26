use chrono::NaiveDate;
use reqwest::Client;
use tokio::task::JoinSet;

use super::{
    hdf5_download::files_download_convert_delete, FileEntry, Placeholder, RemoteFileEntry,
};

//TODO remove TOKEN!!!
use crate::{DOWNLOAD_DIR, PRODUCT_URL, TOKEN, convert::gdal_hdf5_to_geotiff::merge_geotiff};

//Number of parallel downloads
const PD: usize = 20;

//Fetches directory
pub async fn get_dir_content(
    url: String,
    client: Client,
) -> Result<Vec<RemoteFileEntry>, reqwest::Error> {
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .header("X-Requested-With", "XMLHttpRequest")
        .send()
        .await?
        .json::<Placeholder>()
        .await?;

    Ok(resp.content)
}

//Downloads a certain date with data and deletes it afterwards.
pub async fn dl_date_and_convert(year:u32 , day:u32, client: Client) -> Result<(), reqwest::Error> {
    let date_path = format!("{year}/{day}");
    let download_dir = format!("{}/{}", DOWNLOAD_DIR, date_path);
    let url = format!("{}/{}", PRODUCT_URL, date_path);
    // Get the remote directory content and create file entries from them.
    let files = get_dir_content(url, client.clone())
        .await?
        .iter()
        .filter_map(|f| -> Option<FileEntry> {
            if !f.name.ends_with(".h5"){
                return None;
            }

            Some(FileEntry {
                local_path: download_dir.clone(),
                name: f.name.trim_end_matches(".h5").split(".").collect::<Vec<&str>>().join("_"),
                download_link: f.downloadsLink.clone(),
            })
        })
        .collect::<Vec<FileEntry>>();


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
    let _ = files.iter().inspect(|f| {
        if !std::path::Path::new(&f.tif_path()).exists() {
            println!("{} is missing!", f.tif_path());
        }
    });

    // Merge geotiff into one file

    merge_geotiff(download_dir, format!("merged_data_{year}_{day}"));

    Ok(())
}

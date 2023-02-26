use std::{io::Write, path::Path, thread, time::Duration};

use reqwest::Client;

use super::FileEntry;

use crate::{convert::hdf5_file_to_geotif, DOWNLOAD_DIR, TOKEN};

// This function checks if file exists and verifies that the file is a valid hdf5 file.
// If its not valid or the file does not exist it will download the data and create the
// file. Then checks if the downloaded file is a valid hdf5 file, if not it wil reattempt
// the download 3 times with, each time it will pause execution so as to not overload the
// data provider.
//
pub async fn download(
    file_info: FileEntry,
    client: Client,
) -> Result<FileEntry, Box<dyn std::error::Error + Send + Sync>> {
    // let file_path = format!("{}/{}", dl_dir, file_info.name);

    //If the path exists and can be read by hdf5 library.
    if Path::new(&file_info.tif_path()).exists() {
        println!("{} exits, skipping download!", file_info.name);
        return Ok(file_info);
    }

    //4 attempts at downloading the same file
    for i in 0..4 {
        let resp = client
            .get(&file_info.download_link)
            .header("Authorization", format!("Bearer {TOKEN}"))
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await.unwrap()
            .bytes()
            .await.unwrap();

        let p = &file_info.hdf5_path();
        let path = std::path::Path::new(p);
        let _ = std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = std::fs::File::create(path).unwrap();

        file.write_all(resp.into_iter().collect::<Vec<u8>>().as_slice()).unwrap();
        file.flush().unwrap();

        if hdf5::File::open(&file_info.hdf5_path()).is_ok() {
            return Ok(file_info);
        }

        println!("{} download failed! Retrying #{}", file_info.hdf5_name(), i);
        //Retry sequence 0, 1, 4, 8
        thread::sleep(Duration::from_secs(i * i));
    }

    Err("Download failed".into())
}

// If some files failed the download by this point there is nothing we can do
//
pub async fn files_download_convert_delete(files: Vec<FileEntry>, client: Client) {
    for file in files {
        if let Ok(downloaded) = download(file, client.clone()).await {
            if let Ok(converted) = hdf5_file_to_geotif(downloaded).await {
                let _ = std::fs::remove_file(converted.hdf5_path());
            }
        }
    }
}

use std::{io::Write, path::Path, thread, time::Duration};

use reqwest::Client;

use super::FileEntry;

use crate::TOKEN;

// This function checks if file exists and verifies that the file is a valid hdf5 file.
// If its not valid or the file does not exist it will download the data and create the
// file. Then checks if the downloaded file is a valid hdf5 file, if not it wil reattempt
// the download 3 times with, each time it will pause execution so as to not overload the
// data provider.
//
pub async fn download(
    file_info: FileEntry,
    dl_dir: &str,
    client: Client,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    let file_path = format!("{}/{}", dl_dir, file_info.name);

    //If the path exists and can be read by hdf5 library.
    if Path::new(&file_path).exists() && hdf5::File::open(&file_path).is_ok() {
        println!("{} is ok, skipping download!", file_info.name);
        return Ok(file_info.name);
    }

    //4 attempts at downloading the same file
    for i in 0..4 {
        let resp = client
            .get(&file_info.downloadsLink)
            .header("Authorization", format!("Bearer {TOKEN}"))
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await?
            .bytes()
            .await?;

        let mut file = std::fs::File::create(&file_path)?;
        file.write_all(resp.into_iter().collect::<Vec<u8>>().as_slice())?;
        file.flush()?;

        if hdf5::File::open(&file_path).is_ok() {
            return Ok(file_info.name);
        }

        println!("{} download failed! Retrying #{}", file_info.name, i);
        //Retry sequence 0, 1, 4, 8
        thread::sleep(Duration::from_secs(i * i));
    }

    Err("Download failed".into())
}

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tokio::task::JoinSet;

use crate::download::*;

pub mod convert;
pub mod download;

const TOKEN : &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJBUFMgT0F1dGgyIEF1dGhlbnRpY2F0b3IiLCJpYXQiOjE2NzY5MjkyMTMsIm5iZiI6MTY3NjkyOTIxMywiZXhwIjoxNjkyNDgxMjEzLCJ1aWQiOiJwb2x5Z29uMzUwNyIsImVtYWlsX2FkZHJlc3MiOiJhLnJleGhhakBzdHVkLnVpcy5ubyIsInRva2VuQ3JlYXRvciI6InBvbHlnb24zNTA3In0.p8HpTKWCOv1GmjXvyZZlm4O7yQEBAn_95JPIRONzHaM";
const DOWNLOAD_DIR: &str = "./archive";

const HDF5_DATA_PATH: &str = "//HDFEOS/GRIDS/VNP_Grid_DNB/Data_Fields/BrightnessTemperature_M12";

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // let client = reqwest::Client::new();

    // let files = dir_download::dir_content(client.clone()).await.unwrap();
    // dbg!(&files[0]);

    // let mut set = JoinSet::new();

    // let a: Vec<_> = files
    //     .chunks(files.len() / 20)
    //     .map(|file_chunks| {
    //         let _chunks = file_chunks.to_owned();
    //         let _client = client.clone();
    //         set.spawn(async move {
    //             let c = _client.to_owned();
    //             for file in _chunks {
    //                 if let Ok(downloaded) = hdf5_download::download(file, DOWNLOAD_DIR, c.clone()).await
    //                 {
    //                     if gdal_convert_to_geotiff(downloaded).await.is_ok(){
    //                         // delete hdf5 file
    //                     }
    //                 }
    //             }
    //         })
    //     })
    //     .collect();

    // while let Some(res) = set.join_next().await {
    //     // if let Ok(re) = res{
    //     //     if let Some(r) = re{
    //     //     gdal_convert_to_geotiff(&r);
    //     //     }
    //     // }
    // }
}

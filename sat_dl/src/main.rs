use std::error::Error;
// use core::slice::SlicePattern;
use std::io::{Write, Cursor};
use std::{collections::HashMap, fs::File};
use std::process::Command;
use std::fmt::format;
use std::fs;
use serde::{Deserialize, Serialize};


const TOKEN : &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJBUFMgT0F1dGgyIEF1dGhlbnRpY2F0b3IiLCJpYXQiOjE2NzY5MjkyMTMsIm5iZiI6MTY3NjkyOTIxMywiZXhwIjoxNjkyNDgxMjEzLCJ1aWQiOiJwb2x5Z29uMzUwNyIsImVtYWlsX2FkZHJlc3MiOiJhLnJleGhhakBzdHVkLnVpcy5ubyIsInRva2VuQ3JlYXRvciI6InBvbHlnb24zNTA3In0.p8HpTKWCOv1GmjXvyZZlm4O7yQEBAn_95JPIRONzHaM";





#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    println!("Hello, World!");
    // download_folder("19")



    let files = request_dir_content().await.unwrap();
    dbg!(&files[0]);

    let downloaded = download_file(files[0].clone(),".").await.unwrap();

    dbg!(downloaded);


}

// fn download_folder(foldername:&str){

    // let dir_listing_file = "./folder_contents.json";
    // let files = request_dir_content();
    // dbg!();


    // wget -e robots=off -m -np -R .html,.tmp -nH --cut-dirs=3 "https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/5000/VNP46A1/2012/" --header "Authorization: Bearer INSERT_DOWNLOAD_TOKEN_HERE" -P .
    // let command_str: String = format!("-e robots=off -m -np -R .html,.tmp -nH --cut-dirs=3 'https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/5000/VNP46A1/2012/{}' --header 'Authorization: Bearer INSERT_DOWNLOAD_TOKEN_HERE' -P .",foldername);
    // dbg!(&command_str);

    // // The pi will return a json file which contains the contents of the directory
    // wget("https://ladsweb.modaps.eosdis.nasa.gov/api/v2/content/details/allData/5000/VNP46A1/2012/19", "./folder_contents.json");



    // let file = fs::File::open("text.json").expect("file should open read only");

    // let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");

    // // json



// }

//home/ardijan/repos/bachelor_thesis/light_pollution_map/sat_dl/archive/allData/5000/VNP46A1/2012/19/h04v00.001.2019081214225.h5


// fn list_directory(){
    
// }
// // let tmp_dir = Builder::new().prefix("example").tempdir()?;
// let content =  response.text().await?;
// copy(&mut content.as_bytes(), &mut dest)?;
// Ok(())




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    name : String,
    downloadsLink: String,
}

async fn request_dir_content()-> Result<Vec<FileEntry>, reqwest::Error> {


    #[derive(Serialize, Deserialize, Debug)]
    struct Placeholder {
        content: Vec<FileEntry>
    }

    let client = reqwest::Client::new();

    let resp = client.get("https://ladsweb.modaps.eosdis.nasa.gov/api/v2/content/details/allData/5000/VNP46A1/2012/19")
    .header("Authorization", format!("Bearer {TOKEN}"))
    .header("X-Requested-With","XMLHttpRequest")
    .send()
    .await?
    .json::<Placeholder>()
    .await?;

    Ok(resp.content)
}





async fn download_file(file_info: FileEntry, location : &str) -> Result<String, Box<dyn std::error::Error>> {

    // Rename file since gdal will crash if multiple "." exist in the file
    let file_name : String = file_info.name.split(".").enumerate().map(|(i,x)| -> String {
        if i==0 {
            return x.to_string();
        }

        if x=="h5" {
            return ".h5".to_string()
        }

        format!("_{x}")

    }).collect::<Vec<String>>().join("");

    dbg!(&file_name);

    let client = reqwest::Client::new();

    let resp = client.get(file_info.downloadsLink)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .header("X-Requested-With","XMLHttpRequest")
        .send()
        .await?;

    let mut file = File::create(format!("{}/{}",location,file_name))?;
    let mut content =  Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(file_name)

}

use std::process::Command;
use std::fmt::format;

fn main() {
    println!("Hello, World!");
    download_folder("19")
}

fn download_folder(foldername:&str){
    // wget -e robots=off -m -np -R .html,.tmp -nH --cut-dirs=3 "https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/5000/VNP46A1/2012/" --header "Authorization: Bearer INSERT_DOWNLOAD_TOKEN_HERE" -P .
    // let command_str: String = format!("-e robots=off -m -np -R .html,.tmp -nH --cut-dirs=3 'https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/5000/VNP46A1/2012/{}' --header 'Authorization: Bearer INSERT_DOWNLOAD_TOKEN_HERE' -P .",foldername);
    // dbg!(&command_str);
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJBUFMgT0F1dGgyIEF1dGhlbnRpY2F0b3IiLCJpYXQiOjE2NzY5MjkyMTMsIm5iZiI6MTY3NjkyOTIxMywiZXhwIjoxNjkyNDgxMjEzLCJ1aWQiOiJwb2x5Z29uMzUwNyIsImVtYWlsX2FkZHJlc3MiOiJhLnJleGhhakBzdHVkLnVpcy5ubyIsInRva2VuQ3JlYXRvciI6InBvbHlnb24zNTA3In0.p8HpTKWCOv1GmjXvyZZlm4O7yQEBAn_95JPIRONzHaM";
    let out = Command::new("wget")
                .args([
                    "-e",
                    "robots=off",
                    "-m",
                    "-np",
                    "-R",
                    ".html,.tmp",
                    "-nH",
                    // "--cut-dirs=3",
                    &format!("https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/5000/VNP46A1/2012/{}/",foldername),
                    "--header",
                    &format!("Authorization: Bearer {}",token),
                    "-P",
                    "."
                ])
                .output()
                .expect("failed to execute process");

    dbg!(out.stdout);
    println!("{}",String::from_utf8(out.stderr).unwrap());
}



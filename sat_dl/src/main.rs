use chrono::NaiveDate;
use reqwest::Client;

pub mod convert;
pub mod download;


const DOWNLOAD_DIR: &str = "./archive/VNP46A1";
const PRODUCT_URL: &str =
    "https://ladsweb.modaps.eosdis.nasa.gov/api/v2/content/details/allData/5000/VNP46A1";
const HDF5_INTERNAL_DATA_PATH: &str =
    "//HDFEOS/GRIDS/VNP_Grid_DNB/Data_Fields/BrightnessTemperature_M12";

use download::dir_download::dl_date_and_convert;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let client = Client::new();

    // let date_from =  NaiveDate::from_ymd_opt(2012, 01, 19).unwrap();
    let year = 2012;
    let day = 19;
    dl_date_and_convert(year,day,client).await.unwrap();
}

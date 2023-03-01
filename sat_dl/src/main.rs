use std::env;

use chrono::Datelike;
use reqwest::Client;

pub mod convert;
pub mod download;

const DOWNLOAD_DIR_PREFIX: &str = "./archive/VNP46A2";
const PRODUCT_URL: &str =
    "https://ladsweb.modaps.eosdis.nasa.gov/api/v2/content/details/allData/5000/VNP46A2";
const DATASET_PATH: &str = "//HDFEOS/GRIDS/VNP_Grid_DNB/Data_Fields";
// DATASET OPTIONS
// DNB_BRDF-Corrected_NTL
// DNB_Lunar_Irradiance
// Gap_Filled_DNB_BRDF-Corrected_NTL
// Latest_High_Quality_Retrieval
// Mandatory_Quality_Flag
// QF_Cloud_Mask
// Snow_Flag

const DATASET: &str = "Gap_Filled_DNB_BRDF-Corrected_NTL";

pub fn hdf5_internal_data_path() -> String {
    format!("{}/{}", DATASET_PATH, DATASET)
}

pub fn download_dir() -> String {
    format!("{}/{}", DOWNLOAD_DIR_PREFIX, DATASET)
}

use download::dir_download::dl_date_and_convert;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv::dotenv().expect("Missing .env file!");
    env::var("TOKEN").expect("Missing environment variable TOKEN");

    let client = Client::new();

    //TODO make this an input argument
    let from_year = 2012;

    let year_now = chrono::offset::Utc::now().date_naive().year() as u32;
    for year in from_year..year_now + 1 {
        for day in 1..367 {
            println!("Downloading day {}",day);
            let _ = dl_date_and_convert(year, day, client.clone()).await;

        }
    }
}

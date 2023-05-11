use std::{env, process::exit};

use chrono::{Datelike, Duration, NaiveDate};
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

    //Parse command line arguments to get the start and end dates
    let (mut current_date, end_date) = parse_arguments(env::args().collect::<Vec<String>>());

    let client = Client::new();

    while current_date < end_date {
        println!("Downloading date: {current_date}");
        let _ = dl_date_and_convert(
            current_date.year() as u32,
            current_date.ordinal(),
            client.clone(),
        )
        .await;

        current_date += Duration::days(1);
    }
}

fn parse_arguments(arguments: Vec<String>) -> (NaiveDate, NaiveDate) {
    if arguments.len() < 2 {
        println!("Missing argument starting date d.m.y");
        exit(1)
    } else if arguments.len() > 3 {
        println!("Incorrect arguments, supply start and end date example: 1.1.2012 1.1.2020");
        exit(1);
    }

    let start_date =
        NaiveDate::parse_from_str(&arguments[1], "%d.%m.%Y").expect("Unable to parse start date");
    let end_date;

    if let Some(val) = arguments.get(2) {
        end_date = NaiveDate::parse_from_str(&val, "%d.%m.%Y").expect("Unable to parse start date");
    } else {
        end_date = chrono::offset::Utc::now().date_naive();
    }

    if start_date > end_date {
        println!("Incorrect arguments, start date is after end date");
        exit(1);
    }

    (start_date, end_date)
}

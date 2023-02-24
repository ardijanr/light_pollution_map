use std::{process::Command, ffi::OsStr};
use std::fmt::format;
use std::fs;
use std::path::Path;

use gdal::{Dataset, Metadata};


const SOURCE_DIR: &str = "../sat_dl/archive/allData/5000/VNP46A1/2012/19/";
const OUTPUT_DIRNAME: &str = "./test";


fn main(){
    convert_downloaded_data();
}


fn convert_downloaded_data()->Option<bool>{
    let dir_content = fs::read_dir(SOURCE_DIR).unwrap();
    let mut datasets = vec![];
    for file in dir_content{
        if file.is_err(){
            continue;
        }
        let file = file.unwrap();


        // dir_content.into_iter().map(|file|{
        let rasterFilePre = file.path();

        if rasterFilePre.extension() != Some(&OsStr::new("h5")){
            return None;
        }
        dbg!(&file);

        let hdf_layer = Dataset::open(rasterFilePre).unwrap();
        dbg!(&hdf_layer);
        let c = hdf_layer.metadata_domains();

        let domain = hdf_layer.metadata_domain("SUBDATASETS").unwrap();
        // dbg!(domain);
        let c = hdf_layer.metadata_item("SUBDATASET_1_NAME","SUBDATASETS")?;
        // dbg!(&c);

        let mut data = Dataset::open(c).ok()?;
        data.set_projection("EPSG:4326");

        // datasets.push(data.cl);
        // data.

        // let m = data.metadata_domains();
        let HorizontalTileNumber = data.metadata_item("HorizontalTileNumber","")?.parse::<i32>().ok()?;
        let VerticalTileNumber = data.metadata_item("VerticalTileNumber","")?.parse::<i32>().ok()?;

        let WestBoundCoord : i32 = (10*HorizontalTileNumber) - 180;
        let NorthBoundCoord : i32 = 90-(10*VerticalTileNumber);
        let EastBoundCoord : i32 = WestBoundCoord + 10;
        let SouthBoundCoord : i32 = NorthBoundCoord - 10;

        // let translateOptionText = format!("-a_srs EPSG:4326 -a_ullr {WestBoundCoord} {NorthBoundCoord} {EastBoundCoord} {SouthBoundCoord}");
        // dbg!(data.projection());

        let driver = gdal::DriverManager::get_driver_by_name("GTiff").unwrap();

        driver.create_with_band_type_with_options("./testing.tif", size_x, size_y, bands, options)



        // let filename = file.file_name().into_string().unwrap();
        // let filename = filename.as_str();
        // let fname = &filename[..filename.len()-3];

        // let opt = gdal::programs::raster::MultiDimTranslateOptions::new(translateOptionText.split(" ")).ok();
        // let path = gdal::programs::raster::MultiDimTranslateDestination::path(format!("{:}/{:}.tif",OUTPUT_DIRNAME,fname)).unwrap();


        // gdal::programs::raster::multi_dim_translate(&[&data], path, opt);


        // let out = Command::new("gdal_translate")
        //         .arg(data.into())
        //         .args([
        //             fname.to_string(),
        //             "-a_srs".to_string(),
        //             "EPSG:4326".to_string(),
        //             "-a_ullr".to_string(),
        //             WestBoundCoord.to_string(),
        //             NorthBoundCoord.to_string(),
        //             EastBoundCoord.to_string(),
        //             SouthBoundCoord.to_string()
        //             ]).output();

        // dbg!(out);

        // return None;
    // });

    };

    gdal::programs::raster::build_vrt(Some(std::path::Path::new("./test/virt.tif")), datasets.as_slice(), None);








    // gdal::programs::raster::build_vrt(dest, datasets, options)

    return Some(true);
}


// gdal_translate -a_srs "EPSG:4326" -a_ullr 0 0 0 0 HDF5:"testingfile.h5"://HDFEOS/GRIDS/VNP_Grid_DNB/Data_Fields/BrightnessTemperature_M12 test.tif

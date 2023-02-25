use std::process::Command;

use crate::DOWNLOAD_DIR;
use crate::HDF5_DATA_PATH;

//Converts hdf5 dataset to geotiff using gdal
pub async fn hdf5_file_to_geotiff(
    filename: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dl_file = format!("{}/{}", DOWNLOAD_DIR, filename);
    let data_path = format!(r#"HDF5:"{}":{}"#, dl_file, HDF5_DATA_PATH);
    let hdf_file = hdf5::File::open(&dl_file).expect(&format!("error opening {}", dl_file));

    let data = hdf_file.group("HDFEOS/GRIDS/VNP_Grid_DNB")?;

    let west = data.attr("WestBoundingCoord")?.read_1d::<i32>()?[0];
    let north = data.attr("NorthBoundingCoord")?.read_1d::<i32>()?[0];
    let east = data.attr("EastBoundingCoord")?.read_1d::<i32>()?[0];
    let south = data.attr("SouthBoundingCoord")?.read_1d::<i32>()?[0];

    let a = Command::new("gdal_translate")
        .args([
            "-a_srs",
            "EPSG:4326",
            "-a_ullr",
            &west.to_string(),
            &north.to_string(),
            &east.to_string(),
            &south.to_string(),
            &data_path,
            &format!("atiff_results/{}.tif", filename.trim_end_matches(".h5")),
        ])
        .output();

    Ok(())
}



pub async fn merge_geotiff(date:&str,dir:&str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>  {
    //gdalbuildvrt mosaic2.vrt ./atiff_results/*
    //gdal_translate -of GTiff -co "TILED=YES" mosaic2.vrt mosaic_virt.tif
    let fname_vrt = &format!("{}.vrt",date);
    let fname_tif = &format!("{}.tif",date);

    let build_virt = Command::new("gdalbuildvrt")
    .args([
        date,
        fname_vrt,
        dir,
    ])
    .output()?;


    if build_virt.status.code().ok_or("No error code")? != 0 {
        return Err("Unable build vrt file".into());
    }


    let build_single_geotiff = Command::new("gdal_translate")
    .args([
        "-of",
        "GTiff",
        "-co",
        "TILED=YES",
        fname_vrt,
        fname_tif,
    ])
    .output()?;

    if build_single_geotiff.status.code().ok_or("No error code")? != 0 {
        return Err("Unable build geotiff from vrt file".into());
    }

    Ok(fname_tif.to_string())
}
use std::process::Command;

use crate::{download::FileEntry, HDF5_INTERNAL_DATA_PATH};

//Converts hdf5 dataset to geotiff using gdal
pub async fn hdf5_file_to_geotif(
    file: FileEntry,
) -> Result<FileEntry, Box<dyn std::error::Error + Send + Sync>> {
    let hdf_file = hdf5::File::open(&file.hdf5_path())?;

    let data = hdf_file.group("HDFEOS/GRIDS/VNP_Grid_DNB")?;

    let west = data.attr("WestBoundingCoord")?.read_1d::<i32>()?[0];
    let north = data.attr("NorthBoundingCoord")?.read_1d::<i32>()?[0];
    let east = data.attr("EastBoundingCoord")?.read_1d::<i32>()?[0];
    let south = data.attr("SouthBoundingCoord")?.read_1d::<i32>()?[0];

    let data_path = format!(r#"HDF5:"{}":{}"#, file.hdf5_path(), HDF5_INTERNAL_DATA_PATH);

    let _ = Command::new("gdal_translate")
        .args([
            "-a_srs",
            "EPSG:4326",
            "-a_ullr",
            &west.to_string(),
            &north.to_string(),
            &east.to_string(),
            &south.to_string(),
            &data_path,
            &file.tif_path(),
        ])
        .output();

    Ok(file)
}

// This will merge all the files located in the directory
// It will fail if there are files other than .tif files in the directory.
// Commands being run
// gdalbuildvrt mosaic.vrt <dir>/*tif
// gdal_translate -of GTiff -co "TILED=YES" mosaic.vrt mosaic.tif
//
pub async fn merge_geotiff(
    dir: String,
    file_name: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let fname_vrt = &format!("{}/{}.vrt", dir, file_name);
    let fname_tif = &format!("{}/{}.tif", dir, file_name);

    let build_virt = Command::new("gdalbuildvrt")
        .args([fname_vrt, &format!("{}*tif", dir)])
        .output()?;

    if build_virt.status.code().ok_or("No error code")? != 0 {
        return Err("Unable build vrt file".into());
    }

    let build_single_geotiff = Command::new("gdal_translate")
        .args(["-of", "GTiff", "-co", "TILED=YES", fname_vrt, fname_tif])
        .output()?;

    if build_single_geotiff.status.code().ok_or("No error code")? != 0 {
        return Err("Unable build geotiff from vrt file".into());
    }

    Ok(fname_tif.to_string())
}

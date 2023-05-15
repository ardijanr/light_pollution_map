use std::{path::Path, process::Command};

use hdf5::types::FixedAscii;
use nom::bytes::complete::{tag, take_until};
use nom::number::complete::float;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

use crate::download::FileEntry;
use crate::hdf5_internal_data_path;

#[derive(Debug, Clone, Copy)]
struct TileBounds {
    west: i32,
    east: i32,
    north: i32,
    south: i32,
}

fn parse_metadata(input: &str) -> Option<TileBounds> {
    fn skip_and_find_values<'a>(
        input: &'a str,
        match_text: &'a str,
    ) -> IResult<&'a str, (f32, f32)> {
        preceded(
            tuple((take_until(match_text), tag(match_text))),
            parse_coord,
        )(input)
    }

    fn parse_coord(i: &str) -> IResult<&str, (f32, f32)> {
        let (out, (first, _, second)) =
            delimited(tag("("), tuple((float, tag(","), float)), tag(")"))(i)?;
        Ok((out, (first, second)))
    }

    let (_, (west, north)) = skip_and_find_values(input, "UpperLeftPointMtrs=").ok()?;
    let (_, (east, south)) = skip_and_find_values(input, "LowerRightMtrs=").ok()?;

    // Coordinates are in millions
    // Do not be fooled, this is not meters but actually degrees!
    // UpperLeftPointMtrs=(150000000.000000,-20000000.000000)
    // LowerRightMtrs=(160000000.000000,-30000000.000000)
    Some(TileBounds {
        north: north as i32 / 1_000_000,
        south: south as i32 / 1_000_000,
        west: west as i32 / 1_000_000,
        east: east as i32 / 1_000_000,
    })
}

//Converts hdf5 dataset to geotiff using gdal
pub async fn hdf5_file_to_geotif(
    file: FileEntry,
) -> Result<FileEntry, Box<dyn std::error::Error + Send + Sync>> {
    let hdf_file = hdf5::File::open(&file.hdf5_path())?;

    //Query the HDF file for information inside the group
    let data = hdf_file.group("HDFEOS INFORMATION")?;

    //Query the HDF file for information inside the group
    let data_string = data
        .dataset("/HDFEOS INFORMATION/StructMetadata.0")?
        .read_scalar::<FixedAscii<32000>>()?;

    let bounds = parse_metadata(&data_string).ok_or("Unable to parse text")?;

    let data_path = format!(
        r#"HDF5:"{}":{}"#,
        file.hdf5_path(),
        hdf5_internal_data_path()
    );

    let translate = Command::new("gdal_translate")
        .args([
            "-a_srs",
            "EPSG:4326",
            "-a_ullr",
            &bounds.west.to_string(),
            &bounds.north.to_string(),
            &bounds.east.to_string(),
            &bounds.south.to_string(),
            &data_path,
            &file.tif_path(),
        ])
        .output()?;

    if translate.status.code().ok_or("No error code")? != 0 {
        return Err("Unable build geotiff from h5 file".into());
    }

    Ok(file)
}

// This will merge all the files located in the directory
// It will fail if there are files other than .tif files in the directory.
// Commands being run
// gdalbuildvrt mosaic.vrt <dir>/*tif
// gdal_translate -of GTiff -co "TILED=YES" mosaic.vrt mosaic.tif
//
pub async fn merge_geotiffs(
    dir: String,
    file_name: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let fname_vrt = &format!("{}/{}.vrt", dir, file_name);
    let fname_tif = &format!("{}/{}.tif", dir, file_name);

    if !Path::new(&fname_tif).exists() {
        //Need to run as bash command here so that /*tiff works
        let merge_files = Command::new("bash")
            .arg("-c")
            .arg(format!("gdalbuildvrt {} {}/*.tif", fname_vrt, dir))
            .output()?;

        if merge_files.status.code().ok_or("No error code")? != 0 {
            return Err("Unable build vrt file".into());
        }

        let merged_to_geotiff = Command::new("gdal_translate")
            .args(["-of", "GTiff", "-co", "TILED=YES", fname_vrt, fname_tif])
            .output()?;

        if merged_to_geotiff.status.code().ok_or("No error code")? != 0 {
            return Err("Unable build geotiff from vrt file".into());
        }
    }

    // Delete intermediates
    let _ = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "find {dir}/ -type f ! -name '{file_name}.tif' -delete"
        ))
        .output()?;

    Ok(fname_tif.to_string())
}

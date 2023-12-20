#Light Pollution Map

This repository contains:

1. NASA VPN64A2 API data download service, which downloads data and converts it to a GeoTIFF
2. Light pollution model based on R.H. Garstang's 1989 model
3. Map generation service, which takes the GeoTIFF file and processes it by applying the light pollution model, and then applies a gradient and turns it into a PNG image. In future this will be a GeoTIFF file.

### Demonstration GIF of light pollution model.
This gif animation demonstrates what the model looks like from the perspective of a person moving away from a light source or city.
![](./garstang/video_frames/output.gif)

###  Light pollution map, based of satellite data.
This demonstrates the result from the map_generation software, which creates a map using NASA VIIRS sattellite data.
The data can be downloaded using the sat_dl module. 
This is currently work in progress and the output from the garstang model has not yet been calibrated or verified.

![](./assets/single_tile_test.tiff)

How to run: (PS: Date must be formatted as dd.mm.yyyy)
```
cargo run --bin map_generation --release <start date> <end date>
```

You must provide a starting date.
If no end date is provided it will download from start date to current date.




### Satellite download service design diagram
![](./assets/sat_dl.png)

Immediate goals:
- Calibration
- Deployment
# SATELLITE DATA DOWNLOADER AND PRE-PROCESSOR


### What does it do?
1. Downloads data from NASAS's V2 API.
2. Parses HDF5 metadata to find tile coordinate bounds.
3. Converts HDF5 dataset tiles into GeoTiff tiles and deletes the HDF5 file afterwards. Too big to keep.
4. Merges GeoTiff tiles into a singular GeoTiff. The result is around 6 GB for each day.
5. Deletes all intermediate files.


### How does it do it?
The code is written to hopefully be quite readable.
However a diagram is provided here.

INSERT DIAGRAM
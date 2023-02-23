
from osgeo import gdal
import os

## List input raster files
os.chdir('./archive/allData/5000/VNP46A2/2012/19/')
rasterFiles = os.listdir(os.getcwd())
#print(rasterFiles)


#Get File Name Prefix
for layer in range(21):
    outputFolder = f"./tiff/{layer}/"
    fileExtension = "_BBOX.tif"
    try:
        for raster_file in rasterFiles:
            rasterFilePre = raster_file[:-3]

            ## Open HDF file
            hdflayer = gdal.Open(raster_file, gdal.GA_ReadOnly)

            #print (hdflayer.GetSubDatasets())

            # Open raster layer
            #hdflayer.GetSubDatasets()[0][0] - for first layer
            #hdflayer.GetSubDatasets()[1][0] - for second layer ...etc
            subhdflayer = hdflayer.GetSubDatasets()[layer][0]
            rlayer = gdal.Open(subhdflayer, gdal.GA_ReadOnly)
            #outputName = rlayer.GetMetadata_Dict()['long_name']

            #Subset the Long Name
            outputName = subhdflayer[92:]

            outputNameNoSpace = outputName.strip().replace(" ","_").replace("/","_")
            outputNameFinal = outputNameNoSpace + rasterFilePre + fileExtension
            print(outputNameFinal)


            if os.path.exists(outputFolder) == False:
                os.makedirs(outputFolder)

            outputRaster = outputFolder + outputNameFinal

            #collect bounding box coordinates
            HorizontalTileNumber = int(rlayer.GetMetadata_Dict()["HorizontalTileNumber"])
            VerticalTileNumber = int(rlayer.GetMetadata_Dict()["VerticalTileNumber"])

            WestBoundCoord = (10*HorizontalTileNumber) - 180
            NorthBoundCoord = 90-(10*VerticalTileNumber)
            EastBoundCoord = WestBoundCoord + 10
            SouthBoundCoord = NorthBoundCoord - 10

            EPSG = "-a_srs EPSG:4326" #WGS84

            translateOptionText = EPSG+" -a_ullr " + str(WestBoundCoord) + " " + str(NorthBoundCoord) + " " + str(EastBoundCoord) + " " + str(SouthBoundCoord)

            translateoptions = gdal.TranslateOptions(gdal.ParseCommandLine(translateOptionText))
            gdal.Translate(outputRaster,rlayer, options=translateoptions)

            #Display image in QGIS (run it within QGIS python Console) - remove comment to display
            #iface.addRasterLayer(outputRaster, outputNameFinal)

    except:
        pass
            # kwargs_gdal = {'format': 'PNG',
            #                'xRes': xRes,
            #                'yRes': yRes,
            #                'outputType': output
            #                }
import { MapContainer } from 'react-leaflet/MapContainer'
import { TileLayer } from 'react-leaflet/TileLayer'
import {ImageOverlay} from 'react-leaflet'
import {LatLngBounds} from 'leaflet'
import 'leaflet/dist/leaflet.css';
import {CRS} from 'leaflet';

import './App.css';


function App() {
  const position = [55., -5.]

  const bounds = new LatLngBounds([50, 0.],[60, -10] );

  return (
    <MapContainer  crs={CRS.EPSG3395} center={position} zoom={6.3} scrollWheelZoom={false} className="map">
<ImageOverlay
  class="image_overlay"
  url="./uk_test.png"
  bounds={bounds}
  opacity={0.6}
  zIndex={10}
/>
    <TileLayer
      attribution= '&copy; <a href="https://stadiamaps.com/">Stadia Maps</a>, &copy; <a href="https://openmaptiles.org/">OpenMapTiles</a> &copy; <a href="http://openstreetmap.org">OpenStreetMap</a> contributors'
      url='https://tiles.stadiamaps.com/tiles/alidade_smooth_dark/{z}/{x}/{y}{r}.png'
      maxZoom={ 20}
    />
  </MapContainer>
  );
}

export default App;


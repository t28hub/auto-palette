import {useEffect, useRef, useState} from 'react'
import './App.css'
import init, {Palette, Swatch} from '../../wasm/pkg';


function App() {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [wasmInitialized, setWasmInitialized] = useState(false);

  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    init().then((instance) => {
      console.info("WASM initialized");
      console.info(instance);
      setWasmInitialized(true);
    }).catch((err) => {
      console.error("WASM failed to initialize");
      console.error(err);
      setWasmInitialized(false);
    });
  }, []);

  useEffect(() => {
    setImage(null);

    const image = new Image();
    image.src = "https://images.unsplash.com/photo-1682188299490-1e6e9c98bac8?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=bob-brewer-aD5axmPDbdE-unsplash.jpg&w=640";
    image.crossOrigin = "anonymous";
    image.onload = () => {
      setImage(image);
    };
  }, [wasmInitialized]);

  useEffect(() => {
    if (!image || !wasmInitialized) {
      return;
    }

    const context = canvasRef.current?.getContext("2d", {willReadFrequently: true});
    if (!context) {
      return;
    }

    context.drawImage(image, 0, 0, image.width, image.height);
    const imageData = context.getImageData(0, 0, image.width, image.height);

    console.time("palette");
    const palette = Palette.fromImageData(imageData);
    console.info({palette});
    console.timeEnd("palette");

    const swatches = palette.swatches(5);
    swatches.forEach((swatch: Swatch) => {
      console.info(swatch.color.toHexString());
      console.info(swatch.position.x);
      console.info(swatch.position.y);
      console.info(swatch.population);
    });
  }, [image, wasmInitialized]);

  return (
    <>
      <h1>Auto Palette Demo</h1>
      <canvas ref={canvasRef} width={image?.width} height={image?.height}/>
    </>
  )
}

export default App

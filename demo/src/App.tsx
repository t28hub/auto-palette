import { useEffect, useRef, useState } from 'react';
import './App.css';
import { Swatch } from 'auto-palette';
import { useAutoPalette } from './hooks/useAutoPalette.ts';

function App() {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [autoPalette] = useAutoPalette();

  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    setImage(null);

    const image = new Image();
    image.src =
      'https://images.unsplash.com/photo-1682188299490-1e6e9c98bac8?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=bob-brewer-aD5axmPDbdE-unsplash.jpg&w=640';
    image.crossOrigin = 'anonymous';
    image.onload = () => {
      setImage(image);
    };
  }, []);

  useEffect(() => {
    if (!image || !autoPalette) {
      return;
    }

    const context = canvasRef.current?.getContext('2d');
    if (context) {
      context.drawImage(image, 0, 0);
    }

    console.time('palette');
    const palette = autoPalette.extract(image);
    console.info({ palette });
    console.timeEnd('palette');

    const swatches = palette.findSwatches(5);
    swatches.forEach((swatch: Swatch) => {
      console.info(swatch.color.toString());
      console.info(swatch.position.x);
      console.info(swatch.position.y);
      console.info(swatch.population);
    });
  }, [image, autoPalette]);

  return (
    <>
      <h1>Auto Palette Demo</h1>
      <canvas ref={canvasRef} width={image?.naturalWidth} height={image?.naturalHeight} />
    </>
  );
}

export default App;

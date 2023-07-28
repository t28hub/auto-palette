import { useEffect, useRef, useState } from 'react';
import './App.css';
import { WorkerWrapper } from './worker';
import { LoadEvent } from './worker/message.ts';
import { uuid } from './utils/uuid.ts';

function App() {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const worker = new WorkerWrapper();

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
    if (!image) {
      return;
    }

    const context = canvasRef.current?.getContext('2d');
    if (!context) {
      return;
    }

    context.drawImage(image, 0, 0);

    const imageData = context.getImageData(0, 0, image.naturalWidth, image.naturalHeight);
    const event: LoadEvent = {
      id: uuid(),
      type: 'load',
      payload: {
        width: image.naturalWidth,
        height: image.naturalHeight,
        buffer: imageData.data.buffer,
        channels: 4,
      },
    };
    console.time('palette');
    worker
      .postMessage(event, [imageData.data.buffer])
      .then((result) => {
        console.timeEnd('palette');
        console.info(result);
      })
      .catch((error) => {
        console.warn(error);
      });
  }, [image, worker]);

  return (
    <>
      <h1>Auto Palette Demo</h1>
      <canvas ref={canvasRef} width={image?.naturalWidth} height={image?.naturalHeight} />
    </>
  );
}

export default App;

import { useEffect, useRef, useState } from 'react';

import { uuid } from './utils/uuid.ts';
import { WorkerWrapper } from './worker';
import type { LoadEvent } from './worker/message.ts';

function App() {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [colors, setColors] = useState<string[]>([]);

  const wrapperRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const worker = new WorkerWrapper();

  useEffect(() => {
    setImage(null);

    const image = new Image();
    image.src =
      'https://images.unsplash.com/photo-1682188299490-1e6e9c98bac8?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=bob-brewer-aD5axmPDbdE-unsplash.jpg&w=640';
    image.crossOrigin = 'anonymous';
    image.onload = () => {
      setImage(image);
    };

    return () => {
      worker.terminate();
    };
  }, []);

  useEffect(() => {
    if (image === null) {
      return;
    }

    const wrapper = wrapperRef.current;
    if (wrapper === null) {
      return;
    }

    const context = canvasRef.current?.getContext('2d');
    if (context == null) {
      return;
    }

    const width = wrapper.clientWidth;
    const height = wrapper.clientHeight;

    const hRatio = width / image.naturalWidth;
    const vRatio = height / image.naturalHeight;
    const ratio = Math.min(hRatio, vRatio);

    const canvasWidth = Math.round(image.naturalWidth * ratio);
    const canvasHeight = Math.round(image.naturalHeight * ratio);
    if (canvasRef.current !== null) {
      canvasRef.current.width = canvasWidth;
      canvasRef.current.height = canvasHeight;
    }

    context.drawImage(image, 0, 0, canvasWidth, canvasHeight);

    const imageData = context.getImageData(0, 0, canvasWidth, canvasHeight);
    const event: LoadEvent = {
      id: uuid(),
      type: 'load',
      payload: {
        width: canvasWidth,
        height: canvasHeight,
        buffer: imageData.data.buffer,
        channels: 4,
      },
    };

    console.time('palette');
    worker
      .postMessage(event, [imageData.data.buffer])
      .then((result) => {
        console.timeEnd('palette');
        switch (result.type) {
          case 'complete': {
            setColors(result.payload.colors);
            break;
          }
          case 'error': {
            console.warn('Failed to extract colors from image');
            break;
          }
        }
      })
      .catch((error) => {
        console.warn(error);
      });
  }, [image, wrapperRef]);

  return (
    <div className="flex flex-row justify-center items-center w-screen h-screen bg-slate-950">
      <div ref={wrapperRef} className="flex flex-auto justify-center h-full p-4 overscroll-none">
        <canvas ref={canvasRef} />
      </div>
      <div className="flex flex-col flex-none h-full w-48">
        {colors.map((color: string) => {
          const style = {
            backgroundColor: color,
          };
          return (
            <div key={color} className="flex flex-1 items-center justify-center p-4" style={style}>
              {color.toUpperCase()}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default App;

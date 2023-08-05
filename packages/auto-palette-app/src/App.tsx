import { useEffect, useLayoutEffect, useRef, useState } from 'react';

import { uuid } from './utils/uuid.ts';
import { WorkerWrapper } from './worker';
import type { LoadEvent } from './worker/message.ts';

function App() {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [width, setWidth] = useState<number>(0);
  const [height, setHeight] = useState<number>(0);

  const worker = new WorkerWrapper();

  const wrapperRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useLayoutEffect(() => {
    const wrapper = wrapperRef.current;
    if (wrapper == null) {
      return;
    }

    setWidth(wrapper.clientWidth);
    setHeight(wrapper.clientHeight);
  }, []);

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

    const context = canvasRef.current?.getContext('2d');
    if (context == null) {
      return;
    }

    const hRatio = width / image.naturalWidth;
    const vRatio = height / image.naturalHeight;
    const ratio = Math.min(hRatio, vRatio);

    const dw = Math.round(image.naturalWidth * ratio);
    const dh = Math.round(image.naturalHeight * ratio);
    const dx = Math.round((width - dw) / 2);
    const dy = Math.round((height - dh) / 2);
    context.drawImage(image, dx, dy, dw, dh);

    const imageData = context.getImageData(dx, dy, dw, dh);
    const event: LoadEvent = {
      id: uuid(),
      type: 'load',
      payload: {
        width: dw,
        height: dh,
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
    <div className="flex flex-col justify-center items-center w-screen h-screen bg-slate-950">
      <div ref={wrapperRef} className="flex-auto w-full h-full m-4 overscroll-none">
        <canvas ref={canvasRef} width={width} height={height} />
      </div>
    </div>
  );
}

export default App;

import { useEffect, useRef, useState } from 'react';

import { useImageData, usePalette, Options as ImageDataOptions, Swatch } from './hooks';

function App() {
  const [swatches, setSwatches] = useState<Swatch[]>([]);
  const [options, setOptions] = useState<ImageDataOptions | null>(null);
  const imageData = useImageData(
    'https://images.unsplash.com/photo-1682188299490-1e6e9c98bac8?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=bob-brewer-aD5axmPDbdE-unsplash.jpg&w=640',
    options,
  );
  const state = usePalette(imageData);

  const wrapperRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const wrapper = wrapperRef.current;
    if (wrapper === null) {
      return;
    }

    const width = wrapper.clientWidth;
    const height = wrapper.clientHeight;
    setOptions({ width, height, scaleType: 'fit' });
  }, [wrapperRef.current]);

  useEffect(() => {
    const { result, error } = state;
    if (error) {
      console.warn(error);
    }

    setSwatches(result || []);
  }, [state]);

  return (
    <div className="flex flex-row justify-center items-center w-screen h-screen bg-slate-950">
      <div ref={wrapperRef} className="flex flex-auto justify-center h-full p-4 overscroll-none">
        <canvas ref={canvasRef} />
      </div>
      <div className="flex flex-col flex-none h-full w-48">
        {swatches.map(({ color, isLight }) => {
          const style = {
            backgroundColor: color,
          };
          return (
            <div key={color} className="flex flex-1 items-center justify-center p-4" style={style}>
              <span className={`text-opacity-90 ${isLight ? 'text-slate-800' : 'text-slate-100'}`}>
                {color.toUpperCase()}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default App;

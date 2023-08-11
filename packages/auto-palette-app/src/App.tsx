import { useCallback, useEffect, useRef, useState } from 'react';

import FileInput from './components/FileInput.tsx';
import { useImageData, useAutoPalette, Options as ImageDataOptions, Swatch } from './hooks';

function App() {
  const wrapperRef = useRef<HTMLDivElement>(null);

  const [swatches, setSwatches] = useState<Swatch[]>([]);
  const [options, setOptions] = useState<ImageDataOptions | null>(null);
  const imageData = useImageData(
    'https://images.unsplash.com/photo-1682188299490-1e6e9c98bac8?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&dl=bob-brewer-aD5axmPDbdE-unsplash.jpg&w=640',
    options,
  );
  const state = useAutoPalette(imageData);

  const onFileSelect = useCallback((file: File | File[]) => {
    console.log(file);
  }, []);

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
    <div className="flex flex-row justify-center items-center w-screen h-screen bg-white">
      <div ref={wrapperRef} className="flex flex-auto justify-center h-full p-4 overscroll-none">
        <FileInput
          name="image-file"
          types={['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/bmp', 'image/tiff']}
          required={true}
          multiple={false}
          onSelect={onFileSelect}
        >
          <div>aaaa</div>
          {/*<canvas ref={canvasRef} />*/}
        </FileInput>
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

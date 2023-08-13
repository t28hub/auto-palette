import { useCallback, useEffect, useRef, useState } from 'react';

import { FileInput, Swatch } from './components';
import { useImageData, useAutoPalette, Options as ImageDataOptions } from './hooks';

const DEFAULT_OPTIONS: ImageDataOptions = {
  width: 256,
  height: 256,
  scaleType: 'fit',
};

function App() {
  const wrapperRef = useRef<HTMLDivElement>(null);
  const imageRef = useRef<HTMLImageElement>(null);
  const [imageFile, setImageFile] = useState<File>();
  const [scale, setScale] = useState<number>(1);

  const { imageURL, imageData } = useImageData(imageFile, DEFAULT_OPTIONS);
  const { colors } = useAutoPalette(imageData || undefined);

  const onFileSelect = useCallback((file: File | File[]) => {
    if (Array.isArray(file)) {
      setImageFile(file[0]);
    } else {
      setImageFile(file);
    }
  }, []);

  useEffect(() => {
    const image = imageRef.current;
    if (image === null) {
      return;
    }

    if (!imageData) {
      return;
    }

    const { width, height } = imageData;
    const scale = Math.min(width / image.clientWidth, height / image.clientHeight);
    console.info({ width, height, scale, image });
    setScale(scale);
  }, [imageRef.current, imageData]);

  return (
    <div className="flex flex-row justify-center items-center w-screen h-screen bg-white">
      <div ref={wrapperRef} className="flex justify-center items-center w-full h-full p-4 overscroll-none">
        <FileInput
          name="image-file"
          types={['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/bmp', 'image/tiff']}
          required={true}
          multiple={false}
          onSelect={onFileSelect}
          onError={(error) => console.warn(error)}
        >
          <>
            {imageURL && (
              <img ref={imageRef} className="h-full p-2 rounded object-scale-down" alt="Image preview" src={imageURL} />
            )}
            {!imageURL && <span>Select or drop an image file</span>}
            {colors &&
              colors.map((color) => {
                const position = {
                  x: color.position.x / scale,
                  y: color.position.y / scale,
                };
                return <Swatch key={color.hex} color={color.hex} position={position} />;
              })}
          </>
        </FileInput>
      </div>
      <div className="flex flex-col flex-none h-full w-48">
        {colors &&
          colors.map(({ hex: color, isLight }) => {
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

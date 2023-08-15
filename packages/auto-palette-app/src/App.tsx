import { useCallback, useRef, useState } from 'react';

import { FileInput, PreviewImage } from './components';
import { useImageData, useAutoPalette, Options as ImageDataOptions } from './hooks';

const DEFAULT_OPTIONS: ImageDataOptions = {
  // width: 256,
  // height: 256,
  scaleType: 'fit',
};

function App() {
  const wrapperRef = useRef<HTMLDivElement>(null);
  const [imageFile, setImageFile] = useState<File>();

  const { imageURL, imageData } = useImageData(imageFile, DEFAULT_OPTIONS);
  const { colors } = useAutoPalette(imageData || undefined);

  const onFileSelect = useCallback((file: File | File[]) => {
    if (Array.isArray(file)) {
      setImageFile(file[0]);
    } else {
      setImageFile(file);
    }
  }, []);

  return (
    <div className="flex flex-row justify-center items-center w-screen h-screen bg-white">
      <div ref={wrapperRef} className="flex justify-center items-center w-full h-full p-4 overscroll-none">
        <FileInput
          name="image-file"
          className="p-4"
          types={['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/bmp', 'image/tiff']}
          required={true}
          multiple={false}
          onSelect={onFileSelect}
          onError={(error) => console.warn(error)}
        >
          <>
            {imageURL && <PreviewImage src={imageURL} colors={colors || undefined} />}
            {!imageURL && <span>Select or drop an image file</span>}
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

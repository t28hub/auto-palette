import { useCallback, useRef, useState } from 'react';

import FileInput from './components/FileInput.tsx';
import { useImageData, useAutoPalette, Options as ImageDataOptions } from './hooks';

const DEFAULT_OPTIONS: ImageDataOptions = {
  width: 256,
  height: 256,
  scaleType: 'fit',
};

function App() {
  const wrapperRef = useRef<HTMLDivElement>(null);
  const [imageFile, setImageFile] = useState<File>();

  const { imageURL, imageData } = useImageData(imageFile, DEFAULT_OPTIONS);
  const { swatches } = useAutoPalette(imageData || undefined);

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
          types={['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/bmp', 'image/tiff']}
          required={true}
          multiple={false}
          onSelect={onFileSelect}
          onError={(error) => console.warn(error)}
        >
          <>
            {imageURL && <img className="h-full p-2 rounded object-cover" alt="Image preview" src={imageURL} />}
            {!imageURL && <span>Select or drop an image file</span>}
          </>
          {/*</div>*/}
        </FileInput>
      </div>
      <div className="flex flex-col flex-none h-full w-48">
        {swatches &&
          swatches.map(({ color, isLight }) => {
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

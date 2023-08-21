import { ChangeEvent, useCallback, useRef, useState } from 'react';

import { FileInput, PreviewImage } from './components';
import { useImageData, useAutoPalette, AutoPaletteOptions, ImageDataOptions } from './hooks';

const DEFAULT_IMAGE_DATA_OPTIONS: ImageDataOptions = {
  scaleType: 'fit',
};

const DEFAULT_AUTO_PALETTE_OPTIONS: Required<AutoPaletteOptions> = {
  method: 'dbscan',
  colorCount: 5,
  signal: new AbortSignal(),
};

function App() {
  const wrapperRef = useRef<HTMLDivElement>(null);
  const [imageFile, setImageFile] = useState<File>();
  const [autoPaletteOptions, setAutoPaletteOptions] =
    useState<Required<AutoPaletteOptions>>(DEFAULT_AUTO_PALETTE_OPTIONS);

  const { imageURL, imageData } = useImageData(imageFile, DEFAULT_IMAGE_DATA_OPTIONS);
  const { colors } = useAutoPalette(imageData || undefined, autoPaletteOptions);

  const onFileSelect = useCallback((file: File | File[]) => {
    if (Array.isArray(file)) {
      setImageFile(file[0]);
    } else {
      setImageFile(file);
    }
  }, []);

  const onInputChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    const element = event.target;
    const value = parseInt(element.value, 10);
    if (value < 2 || value > 32) {
      return;
    }

    setAutoPaletteOptions((options) => ({
      ...options,
      colorCount: value,
    }));
  }, []);

  const onIncrement = useCallback(() => {
    setAutoPaletteOptions((options) => {
      if (options.colorCount >= 32) {
        return options;
      }
      return { ...options, colorCount: options.colorCount + 1 };
    });
  }, []);

  const onDecrement = useCallback(() => {
    setAutoPaletteOptions((options) => {
      if (options.colorCount <= 2) {
        return options;
      }
      return { ...options, colorCount: options.colorCount - 1 };
    });
  }, []);

  return (
    <div className="w-screen h-screen absolute top-0 left-0 overflow-hidden overscroll-none">
      <div ref={wrapperRef} className="w-full h-full p-4">
        <>{imageURL && <PreviewImage className="-z-10" src={imageURL} colors={colors || undefined} />}</>
        <FileInput
          name="image-file"
          className="p-4"
          types={['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/bmp', 'image/tiff']}
          required={true}
          multiple={false}
          onSelect={onFileSelect}
          onError={(error) => console.warn(error)}
        >
          <>{!imageURL && <span>Select or drop an image file</span>}</>
        </FileInput>
        <div className="fixed inset-x-0 bottom-4 flex items-center justify-center z-50">
          <label className="flex flex-row justify-center items-center flex-wrap gap-4 p-1 border-none rounded bg-slate-950 opacity-90 shadow-xl">
            <button
              className="w-12 h-12 text-center rounded border-none text-gray-50 font-semibold cursor-pointer select-none hover:opacity-60 transition-opacity"
              onClick={onIncrement}
            >
              +
            </button>
            <input
              className="w-12 bg-transparent border-b border-dashed decoration-transparent leading-normal text-gray-50 text-center outline-0"
              type="number"
              inputMode="numeric"
              pattern="[0-9]*"
              min="2"
              max="32"
              step="1"
              required={true}
              value={autoPaletteOptions.colorCount}
              onChange={onInputChange}
            />
            <button
              className="w-12 h-12 text-center rounded border-none text-gray-50 font-semibold cursor-pointer select-none hover:opacity-60 transition-opacity"
              onClick={onDecrement}
            >
              -
            </button>
          </label>
        </div>
        <div className="fixed inset-y-0 right-4 flex flex-col h-full p-4 z-50 drop-shadow-xl">
          {colors &&
            colors.map(({ hex: color, isLight }) => {
              const style = {
                backgroundColor: color,
              };
              return (
                <div key={color} className="flex items-center justify-center flex-1 w-24" style={style}>
                  <span
                    className={`leading-tight font-bold select-none ${isLight ? 'text-slate-900' : 'text-slate-50'}`}
                  >
                    {color.toUpperCase()}
                  </span>
                </div>
              );
            })}
        </div>
      </div>
    </div>
  );
}

export default App;

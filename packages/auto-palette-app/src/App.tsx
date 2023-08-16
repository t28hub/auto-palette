import { ChangeEvent, useCallback, useRef, useState } from 'react';

import { FileInput, PreviewImage } from './components';
import { useImageData, useAutoPalette, AutoPaletteOptions, ImageDataOptions } from './hooks';

const DEFAULT_OPTIONS: ImageDataOptions = {
  // width: 256,
  // height: 256,
  scaleType: 'fit',
};

const DEFAULT_AUTO_PALETTE_OPTIONS: Required<AutoPaletteOptions> = {
  method: 'dbscan',
  colorCount: 5,
};

function App() {
  const wrapperRef = useRef<HTMLDivElement>(null);
  const [imageFile, setImageFile] = useState<File>();
  const [autoPaletteOptions, setAutoPaletteOptions] =
    useState<Required<AutoPaletteOptions>>(DEFAULT_AUTO_PALETTE_OPTIONS);

  const { imageURL, imageData } = useImageData(imageFile, DEFAULT_OPTIONS);
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
    console.log(value);
    if (value < 2 || value > 32) {
      return;
    }

    setAutoPaletteOptions((options) => ({
      ...options,
      colorCount: value,
    }));
  }, []);

  const onPlusClick = useCallback(() => {
    setAutoPaletteOptions((options) => {
      if (options.colorCount >= 32) {
        return options;
      }
      return { ...options, colorCount: options.colorCount + 1 };
    });
  }, []);

  const onMinusClick = useCallback(() => {
    setAutoPaletteOptions((options) => {
      if (options.colorCount <= 2) {
        return options;
      }
      return { ...options, colorCount: options.colorCount - 1 };
    });
  }, []);

  return (
    <div className="flex flex-col justify-center items-center w-screen h-screen bg-white">
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
      <div className="flex flex-row w-full h-24 p-4">
        <label className="flex flex-row">
          <span className="p-4 text-opacity-80">Colors</span>
          <input
            className="p-4 bg-transparent border-none decoration-transparent text-right outline-0"
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
          <span
            className="w-12 p-4 leading-tight text-center rounded opacity-80 border border-solid font-semibold cursor-pointer select-none"
            onClick={onPlusClick}
          >
            +
          </span>
          <span
            className="w-12 p-4 leading-tight text-center rounded opaciy-80 border border-solid font-semibold cursor-pointer select-none"
            onClick={onMinusClick}
          >
            -
          </span>
        </label>
      </div>
      <div className="flex flex-row w-full h-36 p-4">
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

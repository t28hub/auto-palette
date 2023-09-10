import clsx from 'clsx';
import { ReactElement, useCallback, useEffect, useRef, useState } from 'react';
import { BlurhashCanvas } from 'react-blurhash';

import { BlurHashOptions, useAppSelector, useBlurHash, useResizeObserver } from '../hooks';
import { Size } from '../types.ts';

import { Swatch } from './index.ts';

/**
 * Component properties for ImageViewer.
 */
interface Props {
  readonly className?: string;
  readonly imageData?: ImageData;
}

const initialContainerSize: Size = { width: 0, height: 0 };

const blurHashOptions: BlurHashOptions = {
  componentX: 6,
  componentY: 4,
};

/**
 * Image preview component.
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function ImageViewer({ className = '', imageData }: Props): ReactElement {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [containerSize, setContainerSize] = useState<Size>(initialContainerSize);
  const [imageScale, setImageScale] = useState<number>(1.0);
  const paletteState = useAppSelector((state) => state.palette);
  const { hash } = useBlurHash(imageData, blurHashOptions);

  const onResize = useCallback((entry: ResizeObserverEntry): void => {
    const { width, height } = entry.target.getBoundingClientRect();
    setContainerSize({ width, height });
  }, []);
  const { ref: wrapperRef } = useResizeObserver<HTMLDivElement>(onResize);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    if (!imageData) {
      setImageScale(1.0);
      return;
    }

    const scale = Math.min(containerSize.width / imageData.width, containerSize.height / imageData.height);
    if (scale >= 1.0) {
      canvas.width = imageData.width;
      canvas.height = imageData.height;
      setImageScale(1.0);
    } else {
      canvas.width = imageData.width * scale;
      canvas.height = imageData.height * scale;
      setImageScale(scale);
    }

    createImageBitmap(imageData)
      .then((bitmap) => {
        const context = canvas.getContext('2d', { alpha: true });
        if (context === null) {
          return;
        }
        context.clearRect(0, 0, canvas.width, canvas.height);
        context.drawImage(bitmap, 0, 0, canvas.width, canvas.height);
      })
      .catch((error) => {
        console.warn(error);
      });
  }, [imageData, containerSize]);

  return (
    <div ref={wrapperRef} className={clsx('flex', 'justify-center', 'items-center', className)}>
      {hash && (
        <BlurhashCanvas
          className={clsx('flex-shrink-0', 'absolute', 'top-0', 'left-0', '-z-10', 'opacity-60')}
          hash={hash}
          width={containerSize.width}
          height={containerSize.height}
        />
      )}
      <div className="flex-shrink-0 relative shadow-2xl">
        <canvas ref={canvasRef} />

        {paletteState.status === 'succeeded' &&
          paletteState.result.map(({ hex: color, position }) => {
            const x = Math.ceil(position.x * imageScale);
            const y = Math.ceil(position.y * imageScale);
            return <Swatch key={color} color={color} size={32} x={x} y={y} />;
          })}
      </div>
    </div>
  );
}

export default ImageViewer;

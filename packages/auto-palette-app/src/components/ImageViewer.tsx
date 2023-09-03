import { ReactElement, useCallback, useEffect, useRef, useState } from 'react';

import { ImageDataOptions, useAppSelector, useImageData, useResizeObserver } from '../hooks';
import { Size } from '../types.ts';

/**
 * Component properties for ImageViewer.
 */
interface Props {
  readonly className?: string;
}

const defaultSize: Size = { width: 0, height: 0 };

/**
 * Default component properties for useImageData.
 */
const DEFAULT_OPTIONS: ImageDataOptions = {
  scaleType: 'fit',
};

/**
 * Image preview component.
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function ImageViewer(props: Props): ReactElement {
  const { className } = props;
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [size, setSize] = useState<Size>(defaultSize);

  const imageUrl = useAppSelector((state) => state.image.url);

  const { imageData } = useImageData(imageUrl, DEFAULT_OPTIONS);

  const onResize = useCallback((entry: ResizeObserverEntry): void => {
    const { width, height } = entry.target.getBoundingClientRect();
    setSize({ width, height });
  }, []);
  const { ref: wrapperRef } = useResizeObserver<HTMLDivElement>(onResize);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    if (!imageData) {
      return;
    }

    const scale = Math.min(size.width / imageData.width, size.height / imageData.height);
    if (scale >= 1.0) {
      canvas.width = imageData.width;
      canvas.height = imageData.height;
    } else {
      canvas.width = imageData.width * scale;
      canvas.height = imageData.height * scale;
    }

    const context = canvas.getContext('2d', { alpha: true });
    if (context === null) {
      return;
    }
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.putImageData(imageData, 0, 0);
  }, [imageData, size]);

  return (
    <div ref={wrapperRef} className={`flex justify-center items-center relative ${className || ''}`}>
      <canvas ref={canvasRef} className="flex-shrink-0 drop-shadow" />
    </div>
  );
}

export default ImageViewer;

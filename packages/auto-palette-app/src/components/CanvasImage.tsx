import { ReactElement, RefAttributes, useEffect, useRef, useState } from 'react';

import { useImageData, Options as ImageDataOptions } from '../hooks';

/**
 * Component properties for CanvasImage.
 */
type Props = {
  readonly className?: string;
  readonly src: string;
  readonly width?: number;
  readonly height?: number;
  readonly onLoad?: () => void;
  readonly onError?: (error: Error) => void;
} & RefAttributes<ReactElement>;

/**
 * Default component properties for useImageData.
 */
const DEFAULT_OPTIONS: ImageDataOptions = {
  scaleType: 'fit',
};

/**
 * Canvas image component.
 *
 * @constructor
 * @param props - Component properties
 * @return {ReactElement}
 */
function CanvasImage(props: Props): ReactElement {
  const { className, src, width, height, onLoad, onError } = props;
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [imageDataOptions, setImageDataOptions] = useState<ImageDataOptions>(DEFAULT_OPTIONS);
  const { imageData, error: imageDataError } = useImageData(src, imageDataOptions);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    if (width !== undefined) {
      canvas.width = width;
    }
    if (height !== undefined) {
      canvas.height = height;
    }
    setImageDataOptions({ width, height, scaleType: 'fit' });
  }, [width, height, canvasRef.current]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    const context = canvas.getContext('2d');
    if (context === null) {
      return;
    }
    context.clearRect(0, 0, canvas.width, canvas.height);

    if (imageData === null) {
      return;
    }
    const dx = (canvas.width - imageData.width) / 2;
    const dy = (canvas.height - imageData.height) / 2;
    context.putImageData(imageData, dx, dy);

    if (onLoad) {
      onLoad();
    }
  }, [imageData]);

  useEffect(() => {
    if (imageDataError === null) {
      return;
    }

    if (onError) {
      onError(imageDataError);
    }

    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    const context = canvas.getContext('2d');
    if (context === null) {
      return;
    }
    context.fillRect(0, 0, canvas.width, canvas.height);
  }, [imageDataError]);

  return <canvas className={className} ref={canvasRef} width={width} height={height} />;
}

export default CanvasImage;

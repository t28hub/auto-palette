import { ReactElement, useEffect, useRef, useState } from 'react';

import { ImageDataOptions, useImageData, useResizeObserver } from '../hooks';
import { Color } from '../types.ts';

import { Swatch } from './index.ts';

/**
 * Component properties.
 */
type Props = {
  readonly className?: string;
  readonly src: string;
  readonly colors?: Color[];
};

type Size = {
  readonly width: number;
  readonly height: number;
};

type Offset = {
  readonly x: number;
  readonly y: number;
};

type Swatch = {
  readonly key: string;
  readonly color: string;
  readonly x: number;
  readonly y: number;
};

const DEFAULT_SIZE: Size = { width: 0, height: 0 };

const DEFAULT_OFFSET: Offset = { x: 0, y: 0 };

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
function PreviewImage(props: Props): ReactElement {
  const { className, src, colors } = props;

  const canvasRef = useRef<HTMLCanvasElement>(null);

  const { ref: wrapperRef } = useResizeObserver<HTMLDivElement>((entry) => {
    const { width, height } = entry.target.getBoundingClientRect();
    setSize({ width, height });
  });

  const { imageData } = useImageData(src, DEFAULT_OPTIONS);
  const [size, setSize] = useState<Size>(DEFAULT_SIZE);
  const [offset, setOffset] = useState<Offset>(DEFAULT_OFFSET);
  const [swatches, setSwatches] = useState<Swatch[]>([]);

  useEffect(() => {
    const wrapper = wrapperRef.current;
    if (wrapper === null) {
      return;
    }

    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    const wrapperRect = wrapper.getBoundingClientRect();
    const canvasRect = canvas.getBoundingClientRect();
    setOffset({ x: canvasRect.left - wrapperRect.left, y: canvasRect.top - wrapperRect.top });
  }, [canvasRef.current, size]);

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

  useEffect(() => {
    if (!colors) {
      setSwatches([]);
      return;
    }

    setSwatches(
      colors.map((color, index) => {
        const key = `swatch-${index}-${color.hex}`;
        const x = color.position.x + offset.x;
        const y = color.position.y + offset.y;
        return { key, color: color.hex, x, y };
      }),
    );
  }, [colors, size]);

  return (
    <div ref={wrapperRef} className={`flex flex-row justify-center items-center w-full h-full ${className || ''}`}>
      <canvas className="shadow-xl" ref={canvasRef} />
      {swatches.map((swatch) => {
        const { key, color, x, y } = swatch;
        return <Swatch key={key} color={color} x={x} y={y} />;
      })}
    </div>
  );
}

export default PreviewImage;

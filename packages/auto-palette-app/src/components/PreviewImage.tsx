import { ReactElement, useEffect, useRef, useState } from 'react';

import { Options as ImageDataOptions, useImageData, useResizeObserver } from '../hooks';
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
    const { paddingLeft, paddingRight, paddingTop, paddingBottom } = getComputedStyle(entry.target);
    const widthPadding = parseInt(paddingLeft, 10) + parseInt(paddingRight, 10);
    const heightPadding = parseInt(paddingTop, 10) + parseInt(paddingBottom, 10);
    setWidth(width - widthPadding);
    setHeight(height - heightPadding);
  });

  const { imageData } = useImageData(src, DEFAULT_OPTIONS);
  const [width, setWidth] = useState<number>(0);
  const [height, setHeight] = useState<number>(0);
  const [offset, setOffset] = useState({ x: 0, y: 0 });

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas === null) {
      return;
    }

    if (!imageData) {
      return;
    }

    const { width: imageWidth, height: imageHeight } = imageData;
    const scale = Math.min(width / imageWidth, height / imageHeight);
    canvas.width = imageWidth * scale;
    canvas.height = imageHeight * scale;

    const context = canvas.getContext('2d', { alpha: true });
    if (context === null) {
      return;
    }
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.putImageData(imageData, 0, 0);
  }, [imageData, width, height]);

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
  }, []);

  return (
    <div ref={wrapperRef} className={`flex flex-row justify-center items-center w-full h-full ${className || ''}`}>
      <canvas ref={canvasRef} />
      {colors &&
        colors.map((color) => {
          const x = color.position.x + offset.x;
          const y = color.position.y + offset.y;
          return <Swatch key={color.hex} color={color.hex} x={x} y={y} />;
        })}
    </div>
  );
}

export default PreviewImage;

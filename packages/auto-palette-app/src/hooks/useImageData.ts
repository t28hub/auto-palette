import { useEffect, useMemo, useState } from 'react';

export type ScaleType = 'fill' | 'fit';

export type Options = {
  readonly width: number;
  readonly height: number;
  readonly scaleType: ScaleType;
};

export function useImageData(url: string, options: Options | null = null): ImageData | null {
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [imageData, setImageData] = useState<ImageData | null>(null);
  const canvas = useMemo(() => document.createElement('canvas'), []);

  useEffect(() => {
    if (url.length === 0) {
      setImage(null);
      return;
    }

    let isCancelled = false;
    const image = new Image();
    image.src = url;
    image.crossOrigin = 'anonymous';
    image.onload = () => {
      if (isCancelled) {
        return;
      }
      setImage(image);
    };
    image.onerror = () => {
      if (isCancelled) {
        return;
      }
      setImage(null);
    };

    return () => {
      isCancelled = true;
      image.onload = null;
      image.onerror = null;
    };
  }, [url]);

  useEffect(() => {
    if (image === null) {
      setImageData(null);
      return;
    }

    const context = canvas.getContext('2d');
    if (context === null) {
      setImageData(null);
      return;
    }

    const { width, height, scaleType } = options ?? {
      width: image.naturalWidth,
      height: image.naturalHeight,
      scaleType: 'fit',
    };
    canvas.width = width;
    canvas.height = height;

    if (scaleType === 'fill') {
      const scale = Math.max(width / image.naturalWidth, height / image.naturalHeight);
      context.drawImage(image, 0, 0, image.naturalWidth * scale, image.naturalHeight * scale);
    } else {
      const scale = Math.min(width / image.naturalWidth, height / image.naturalHeight);
      context.drawImage(image, 0, 0, image.naturalWidth * scale, image.naturalHeight * scale);
    }
    setImageData(context.getImageData(0, 0, width, height));
  }, [image, options]);

  return imageData;
}

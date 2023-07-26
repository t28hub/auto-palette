import { AutoPalette, AutoPaletteOptions } from 'auto-palette';
import { useEffect, useRef, useState } from 'react';

/**
 * Hook for using the AutoPalette.
 *
 * @param options - The options of the AutoPalette.
 * @returns The AutoPalette instance.
 */
export const useAutoPalette = (options?: AutoPaletteOptions) => {
  const isMounted = useRef(false);
  const [autoPalette, setAutoPalette] = useState<AutoPalette | null>(null);

  useEffect(() => {
    isMounted.current = true;

    AutoPalette.initialize(options)
      .then((autoPalette) => {
        if (!isMounted.current) {
          return;
        }
        setAutoPalette(autoPalette);
      })
      .catch(() => {
        if (!isMounted.current) {
          return;
        }
        setAutoPalette(null);
      });

    return () => {
      isMounted.current = false;
    };
  }, [options]);

  return [autoPalette];
};

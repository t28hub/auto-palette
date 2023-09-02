import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux';

import { AppDispatch, RootState } from '../store';

export const useAppDispatch: () => AppDispatch = useDispatch;
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;

export { useImageData, type Options as ImageDataOptions, type ScaleType } from './useImageData.ts';
export { useAutoPalette, type Options as AutoPaletteOptions } from './useAutoPalette.ts';
export { useResizeObserver } from './useResizeObserver.ts';

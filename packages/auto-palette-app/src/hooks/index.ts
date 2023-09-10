import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux';

import { AppDispatch, RootState } from '../store';

export const useAppDispatch: () => AppDispatch = useDispatch;
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;

export { useBlurHash, type Options as BlurHashOptions } from './useBlurHash.ts';
export { useImageData, type Options as ImageDataOptions, type ScaleType } from './useImageData.ts';
export { useResizeObserver } from './useResizeObserver.ts';

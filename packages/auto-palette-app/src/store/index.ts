import { configureStore } from '@reduxjs/toolkit';

import imageReducer from './image.ts';
import paletteReducer from './palette.ts';

export { setImageUrl } from './image.ts';
export { extractPalette } from './palette.ts';

export const imageSelector = (state: RootState) => state.image;
export const paletteSelector = (state: RootState) => state.palette;

/**
 * The store of the application.
 */
export const store = configureStore({
  reducer: {
    image: imageReducer,
    palette: paletteReducer,
  },
});

/**
 * The root state of the store.
 */
export type RootState = ReturnType<typeof store.getState>;

/**
 * The dispatch function of the store.
 */
export type AppDispatch = typeof store.dispatch;

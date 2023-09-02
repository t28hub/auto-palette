import { configureStore } from '@reduxjs/toolkit';

import imageReducer from './image.ts';

/**
 * The store of the application.
 */
export const store = configureStore({
  reducer: {
    image: imageReducer,
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

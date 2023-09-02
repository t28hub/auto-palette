import { createSlice, PayloadAction } from '@reduxjs/toolkit';

/**
 * Interface for the image state.
 */
interface ImageState {
  /**
   * The URL of the image.
   */
  readonly url: string;
}

/**
 * The initial state for the image.
 */
const initialState: ImageState = {
  url: '',
};

const imageSlice = createSlice({
  name: 'image',
  initialState,
  reducers: {
    setImageUrl(state, action: PayloadAction<ImageState>) {
      state.url = action.payload.url;
    },
  },
});

export const { setImageUrl } = imageSlice.actions;

export default imageSlice.reducer;

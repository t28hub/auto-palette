import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';

import { Color } from '../types.ts';
import { createClient } from '../worker';
import { Options } from '../worker/client.ts';

interface Args {
  imageData: ImageData;
  options?: Options;
}

interface PaletteState {
  status: 'idle' | 'loading';
  result: Color[];
  error: Error | null;
}

const initialState: PaletteState = {
  status: 'idle',
  result: [],
  error: null,
};

const worker = createClient();

/**
 * Async thunk that extracts a color palette from an image data.
 */
export const extractPalette = createAsyncThunk<Color[], Args>(
  'palette/extract',
  async ({ imageData, options }, thunkApi) => {
    try {
      const colors = await worker.extract(imageData, options);
      return colors.map((color): Color => {
        return {
          hex: color.hex,
          isLight: color.isLight,
          position: color.position,
        };
      });
    } catch (error) {
      return thunkApi.rejectWithValue(error);
    }
  },
);

const paletteSlice = createSlice({
  name: 'palette',
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder.addCase(extractPalette.pending, (state) => {
      state.status = 'loading';
      state.result = [];
      state.error = null;
    });

    builder.addCase(extractPalette.fulfilled, (state, action) => {
      state.status = 'idle';
      state.result = action.payload;
      state.error = null;
    });

    builder.addCase(extractPalette.rejected, (state, action) => {
      state.status = 'idle';
      state.result = [];
      if (action.payload instanceof Error) {
        state.error = action.payload;
      } else {
        state.error = new Error('Unknown error');
      }
    });
  },
});

export default paletteSlice.reducer;

import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';

import { Color } from '../types.ts';
import { createClient } from '../worker';
import { Options } from '../worker/client.ts';

/**
 * Interface for the arguments of the `extractPalette` async thunk.
 */
interface Args {
  readonly imageData: ImageData;
  readonly options?: Options;
}

/**
 * Interface for the palette state.
 */
interface PaletteState {
  readonly status: 'idle' | 'loading' | 'succeeded' | 'failed';
  readonly result: Color[];
  readonly error: Error | null;
}

const initialState: PaletteState = {
  status: 'idle',
  result: [],
  error: null,
};

// The shared worker instance.
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
      return thunkApi.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
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
      state.error = null;
    });

    builder.addCase(extractPalette.fulfilled, (state, action) => {
      state.status = 'succeeded';
      state.result = [...action.payload];
      state.error = null;
    });

    builder.addCase(extractPalette.rejected, (state, action) => {
      state.status = 'failed';
      state.result = [];
      state.error = action.payload instanceof Error ? action.payload : new Error('Unknown error');
    });
  },
});

export default paletteSlice.reducer;

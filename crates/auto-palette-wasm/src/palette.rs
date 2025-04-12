use auto_palette::{Algorithm, ImageData, Palette, Swatch, Theme};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::ImageData as ImageSource;

use crate::{
    swatch::JsSwatch,
    types::{JsAlgorithm, JsTheme},
};

/// The `Palette` class represents a color palette extracted from an image.
#[derive(Debug)]
#[wasm_bindgen(js_name = Palette)]
pub struct JsPalette(Palette<f64>);

#[wasm_bindgen(js_class = Palette)]
impl JsPalette {
    /// Creates a new `Palette` instance with the given swatches.
    ///
    /// @param swatches The swatches in the palette.
    /// @returns A new `Palette` instance.
    #[wasm_bindgen(constructor)]
    pub fn new(swatches: Vec<JsSwatch>) -> Self {
        let swatches = swatches
            .into_iter()
            .map(|swatch| {
                let color = swatch.color();
                let position = swatch.position();
                Swatch::new(
                    color.0,
                    (position.x, position.y),
                    swatch.population(),
                    swatch.ratio(),
                )
            })
            .collect();
        let palette = Palette::new(swatches);
        Self(palette)
    }

    /// Returns the number of swatches in this palette.
    ///
    /// @returns The number of swatches in this palette.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.0.len()
    }

    /// Returns whether this palette is empty.
    ///
    /// @returns `true` if this palette is empty, `false` otherwise.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Finds the best `n` swatches in this palette.
    ///
    /// @param n The number of swatches to find.
    /// @param theme The theme to use when finding the swatches.
    /// @returns The best swatches in this palette.
    #[wasm_bindgen(js_name = findSwatches)]
    pub fn find_swatches(
        &self,
        n: usize,
        theme: Option<JsTheme>,
    ) -> Result<Vec<JsSwatch>, JsError> {
        let found = match theme {
            Some(name) => {
                let theme = Theme::try_from(name)?;
                self.0.find_swatches_with_theme(n, theme).map_err(|e| {
                    JsError::new(&format!("Failed to find swatches with theme: {}", e))
                })?
            }
            None => self
                .0
                .find_swatches(n)
                .map_err(|e| JsError::new(&format!("Failed to find swatches: {}", e)))?,
        };

        let swatches = found.into_iter().map(JsSwatch::from).collect::<Vec<_>>();
        Ok(swatches)
    }

    /// Extracts a color palette from the given image.
    ///
    /// @param image The image to extract the palette from.
    /// @param algorithm The algorithm to use for palette extraction. Defaults to 'dbscan'.
    /// @returns A new `Palette` instance.
    /// @throws Error if the image is invalid or the palette extraction fails.
    #[wasm_bindgen]
    pub fn extract(image: &ImageSource, algorithm: Option<JsAlgorithm>) -> Result<Self, JsError> {
        let width = image.width();
        let height = image.height();
        let data = image.data();
        let image_data = ImageData::new(width, height, &data)
            .map_err(|e| JsError::new(&format!("Failed to create ImageData from image: {}", e)))?;

        let algorithm = algorithm
            .map(Algorithm::try_from)
            .unwrap_or(Ok(Algorithm::DBSCAN))?;

        let palette = Palette::extract_with_algorithm(&image_data, algorithm)
            .map_err(|e| JsError::new(&format!("Failed to extract palette from image: {}", e)))?;
        Ok(Self(palette))
    }
}

use rayon::prelude::*;

use crate::RgbExrImage;

pub fn mse(ref_image: &RgbExrImage, test_image: &RgbExrImage) -> f32 {
    let sum: f32 = ref_image
        .data
        .par_iter()
        .zip(test_image.data.par_iter())
        .map(|(&refi, &test)| (refi - test).powi(2))
        .sum();

    sum / (ref_image.size as f32)
}

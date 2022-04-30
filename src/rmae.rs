use rayon::prelude::*;

use crate::RgbExrImage;

const RMAE_EPS: f32 = 0.001;

pub fn rmae(ref_image: &RgbExrImage, test_image: &RgbExrImage) -> f32 {
    let sum: f32 = ref_image
        .data
        .par_iter()
        .zip(test_image.data.par_iter())
        .map(|(&refi, &test)| ((refi - test) / (refi + RMAE_EPS)).abs())
        .sum();

    sum / (ref_image.size as f32)
}

use rayon::prelude::*;

use crate::RgbExrImage;

pub fn mae(ref_image: &RgbExrImage, test_image: &RgbExrImage) -> f32 {
    let sum: f32 = ref_image
        .data
        .par_iter()
        .zip(test_image.data.par_iter())
        .map(|(&refi, &test)| (refi - test).abs())
        .sum();

    sum / (ref_image.size as f32)
}

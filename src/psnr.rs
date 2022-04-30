use crate::RgbExrImage;

const PSNR_EPS: f32 = 0.0001;

pub fn psnr(ref_image: &RgbExrImage, test_image: &RgbExrImage) -> f32 {
    let mse = crate::mse::mse(ref_image, test_image);
    10.0 * (1.0 / (mse + PSNR_EPS)).log10()
}
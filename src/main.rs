use structopt::StructOpt;

mod mse;
mod relmse;
mod psnr;
mod mae;
mod rmae;
mod smape;
mod flip;

pub struct RgbExrImage {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub size: usize,
}

impl RgbExrImage {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            data: vec![0.0; size * 3],
            width,
            height,
            size,
        }
    }

    fn from_file(path: &str) -> Self {
        exr::prelude::read_first_rgba_layer_from_file(
            path,
            |res: exr::math::Vec2<usize>, _| {
                Self::new(res.width(), res.height())
            },
            |image, pos, (r, g, b, _): (f32, f32, f32, f32)| {
                image.set(pos.x(), pos.y(), r, g, b)
            },
        )
            .unwrap()
            .layer_data
            .channel_data
            .pixels
    }

    fn set(&mut self, x: usize, y: usize, r: f32, g: f32, b: f32) {
        let index = 3 * (y * self.width + x);
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
    }
}

#[derive(StructOpt)]
#[structopt(name = "image-cmp-tool")]
struct Opt {
    /// path to the reference image (ground-truth image)
    #[structopt(short, long)]
    reference: String,
    /// path to the test image (rendered image)
    #[structopt(short, long)]
    test: String,
    /// MSE (mean squared error)
    #[structopt(long)]
    mse: bool,
    /// relMSE (relative mean squared error)
    #[structopt(long)]
    relmse: bool,
    /// PSNR (peak signal to noise ratio)
    #[structopt(long)]
    psnr: bool,
    /// MAE (mean absolute error)
    #[structopt(long)]
    mae: bool,
    /// RMAE (relative mean absolute error)
    #[structopt(long)]
    rmae: bool,
    /// SMAPE (symmetric mean absolute percentage error)
    #[structopt(long)]
    smape: bool,
    /// FLIP (FLIP: A Tool for Visualizing and Communicating Errors in Rendered Images)
    #[structopt(long)]
    flip: bool,
}

fn main() {
    let opt = Opt::from_args();

    let ref_image = RgbExrImage::from_file(&opt.reference);
    let test_image = RgbExrImage::from_file(&opt.test);

    if opt.mse {
        let error = mse::mse(&ref_image, &test_image);
        println!("MSE: {}", error)
    }

    if opt.relmse {
        let error = relmse::relmse(&ref_image, &test_image);
        println!("relMSE: {}", error)
    }

    if opt.psnr {
        let error = psnr::psnr(&ref_image, &test_image);
        println!("PSNR: {}", error)
    }

    if opt.mae {
        let error = mae::mae(&ref_image, &test_image);
        println!("MAE: {}", error)
    }

    if opt.rmae {
        let error = rmae::rmae(&ref_image, &test_image);
        println!("RMAE: {}", error)
    }

    if opt.smape {
        let error = smape::smape(&ref_image, &test_image);
        println!("SMAPE: {}", error)
    }

    if opt.flip {
        let error = flip::flip(&opt.reference, &opt.test);
        println!("FLIP: {}", error)
    }
}

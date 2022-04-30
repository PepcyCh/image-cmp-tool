use std::str::FromStr;

const FLIP_CMD: &str = "C:\\Users\\PepcyCh\\Documents\\PepcyCh\\Programs\\flip\\cpp\\x64\\Release\\flip-cuda.exe";

pub fn flip(ref_image: &str, test_image: &str) -> f32 {
    let flip_output = std::process::Command::new(FLIP_CMD)
        .args(["-r", ref_image, "-t", test_image, "-nerm", "-nexm"])
        .output()
        .unwrap()
        .stdout;
    let flip_output = String::from_utf8(flip_output).unwrap();

    let re = lazy_regex::regex!(r#"Mean: ([\d\.]+)"#);
    let flip_str = re.captures(&flip_output).unwrap().get(1).unwrap().as_str();
    let flip = f32::from_str(flip_str).unwrap();

    flip
}

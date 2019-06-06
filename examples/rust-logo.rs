use aa_rs::{utils, filter};
use std::fs::File;

fn main() {
    // Various optional filters, which can all be configured.
    // More information on what these filters do in the documentation.
    let mut hough_filter = filter::hough::default();
    let binary_filter = filter::binary::default();
    let shrink_filter = filter::shrink::default();

    let image_array = utils::read_image(File::open("./rust.png").unwrap()).unwrap();
    let grayscale_array = filter::grayscale::default().run(image_array);
    let gradient_array = filter::line::default().run(grayscale_array.clone());
    let line_array = shrink_filter.run(binary_filter.run(gradient_array)).mapv(|e| e as f32) * 250.;
    let hough_array = hough_filter.run(line_array);
    let aa = filter::ascii_art::default().run(hough_array);

    println!("{}", aa);
}
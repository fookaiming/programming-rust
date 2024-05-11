use std::env;
use std::fs::File;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXEL UPPER_LEFT LOWER_RIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.2,0.35 -1.0,0.20", args[0]);
        std::process::exit(1);
    }
    let bound = parse_pair(&args[2], 'x').expect("error when parsing image dimension");
    let upper_left = parse_complex_number(&args[3]).expect("error when parsing upper left corner");
    let lower_right = parse_complex_number(&args[4]).expect("error when parsing lower right corner");

    let mut pixels = vec![0 as u8; bound.0 * bound.1];
    dispatch_render(&mut pixels, bound, upper_left, lower_right);
    write_image(&args[1], &pixels, bound).expect("error when writing image file");
}


fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    // T cannot be inferred
    assert_eq!(parse_pair::<u8>("3:", ':'), None);
    assert_eq!(parse_pair::<u16>(":4", ':'), None);
    assert_eq!(parse_pair::<u32>(":", ':'), None);
    // T can be inferred from Some((3,4))
    assert_eq!(parse_pair("3,4", ','), Some((3, 4)));
    assert_eq!(parse_pair("3:4", ':'), Some((3, 4)));
}

fn parse_complex_number(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex_number() {
    assert_eq!(parse_complex_number("3:5"), None);
    assert_eq!(parse_complex_number("3:"), None);
    assert_eq!(parse_complex_number("35"), None);
    assert_eq!(parse_complex_number("3,5"), Some(Complex { re: 3.0, im: 5.0 }));
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + width * (pixel.0 as f64 / bounds.0 as f64),
        im: upper_left.im - height * (pixel.1 as f64 / bounds.1 as f64),
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), Complex { re: -0.5, im: -0.75 });
}

fn escape_time(c: Complex<f64>, limit: u8) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn dispatch_render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    let threads = num_cpus::get_physical();
    let row_per_threads = bounds.1 / threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(row_per_threads * bounds.0).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = row_per_threads * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    }).unwrap();
}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count
            }
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}
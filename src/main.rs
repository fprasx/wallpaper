use image::{Rgb, RgbImage, ImageBuffer};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::env;
use std::fs;
use std::error::Error;
use std::path::Path;

const SCREEN_HEIGHT: usize = 956;
const SCREEN_WIDTH: usize = 1470;

fn main() {
    let arg = if env::args().count() == 2 {
        env::args().nth(1).expect("ai yah")
    } else {
        panic!("Please enter a target file path")
    };

    let path = Path::new(&arg);

    let mut image = ImageBuffer::from_pixel(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, Rgb([53, 73, 109]));

    let font = Vec::from(include_bytes!("../iosevka-regular.ttc") as &[u8]);
    let font = Font::try_from_vec(font).expect("error handling is overrated");

    let text = fs::read_to_string("message.txt").expect("bruh");
    draw_string(&mut image, &text, &font).expect("no artistic ability");

    image.save(path).expect("os L");
}

fn draw_string(image: &mut RgbImage, text: &str, font: &Font) -> Result<(), Box<dyn Error>> {
    let height = text.lines().count();
    let width = text
        .lines()
        .map(|l| l.len())
        .max()
        .ok_or("all lines were empty")?;

    let size = (SCREEN_HEIGHT / height).min(SCREEN_WIDTH / width);
    let scale = Scale {
        x: size as f32,
        y: size as f32,
    };

    let mut drawn_height = 100; // Start a line down to avoid status bar
    for line in text.lines() {
        draw_text_mut(
            image,
            Rgb([61, 53, 109]),
            0,
            drawn_height,
            scale,
            font,
            line,
        );
        let (_, offset) = text_size(scale, font, text);
        drawn_height += offset;
    }
    Ok(())
}

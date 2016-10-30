extern crate image;
extern crate cgmath;
extern crate minifb;
extern crate nfd as file_dialog;

use image::{DynamicImage, GenericImage, Pixel};
use std::ops::{Index};
use cgmath::InnerSpace;
use minifb::{Key, Scale, WindowOptions, Window};
use std::u32;
use file_dialog::Response;


fn file_dialogs() -> String {
    let result = file_dialog::dialog().open().unwrap();

    match result {
        Response::Okay(file_path) => return file_path,
        _ => panic!("d"),
    }
}

fn main() {

    let path = file_dialogs();

    let mut img = image::open(path).unwrap().grayscale();
    //img = img.blur(2_f32);
    let mut normal_image = DynamicImage::new_rgb8(img.dimensions().0, img.dimensions().1);

    for y in 0..img.height() - 1 {
        for x in 0..img.width() - 1 {
            let vx;
            let vy;
            if x > 0 && x < img.width() {
                vx = cgmath::Vector3 { x: 1_f32, y: 0_f32, z: (img.get_pixel(x - 1, y)[0] as f32 - img.get_pixel(x + 1, y)[0] as f32) };
            } else {
                vx = cgmath::Vector3 { x: 1_f32, y: 0_f32, z: (img.get_pixel(x, y)[0] - img.get_pixel(x, y)[0]) as f32 };
            }

            if y > 0 && y < img.height() {
                vy = cgmath::Vector3 { x: 0_f32, y: 1_f32, z: (img.get_pixel(x, y - 1)[0] as f32 - img.get_pixel(x, y + 1)[0] as f32) };
            } else {
                vy = cgmath::Vector3 { x: 0_f32, y: 1_f32, z: (img.get_pixel(x, y)[0] - img.get_pixel(x, y)[0]) as f32 };
            }

            let mut normal = vx.cross(vy);
            normal = normal.normalize();

            let pixel = image::Rgba::from_channels((((normal.x + 1.0) / 2.0) * 255_f32) as u8,
                                       (((normal.y + 1.0) / 2.0) * 255_f32) as u8,
                                       (((normal.z + 1.0) / 2.0) * 255_f32) as u8, 255_u8);

            normal_image.put_pixel(x, y, pixel);
        }
    }

    normal_image.to_rgb().save(ADD_SAVE_PATH).unwrap();

    let mut buffer: Vec<u32> = vec![0; (img.width() * img.height()) as usize];

    let mut window = Window::new("Image To Normal",
                                 img.width() as usize,
                                 img.height() as usize,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let raw = normal_image.raw_pixels();
    let buffer_length = buffer.len();
    for i in buffer.iter_mut().enumerate() {
        let index = i.0;
        let offset = index * 3;
            let hex = format!("{:X}{:X}{:X}", raw[offset], raw[1 + offset], raw[2 + offset]);
            let rgb_hex = u32::from_str_radix(&hex, 16).unwrap();
            *i.1 = rgb_hex;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer);
    }
}
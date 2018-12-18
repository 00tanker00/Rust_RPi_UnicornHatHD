extern crate unicorn_hat_hd;
extern crate rgb;

use unicorn_hat_hd::UnicornHatHd;

const RED: rgb::RGB8 = rgb::RGB8 {r: 255, g: 0, b: 0};
const CLEAR: rgb::RGB8 = rgb::RGB8 {r: 0, g: 0, b: 0};
//const BLUE: rgb::RGB8 = rgb::RGB8 {r: 0, g: 0, b: 255};
const GREEN: rgb::RGB8 = rgb::RGB8 {r: 0, g: 255, b: 0};

pub fn main() {
    let mut hat_hd = UnicornHatHd::default();
    loop {
        for y in 0..16 {
            for x in 0..16 {
                hat_hd.set_pixel(x, y, RED);
                hat_hd.display().unwrap();
                //hat_hd.set_pixel(x, y, BLUE);
                //hat_hd.display().unwrap();
                hat_hd.set_pixel(x, y, GREEN);
                hat_hd.display().unwrap();
                hat_hd.set_pixel(x, y, CLEAR);
            }
        }
    }
}

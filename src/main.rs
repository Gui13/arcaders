extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;

mod events;

fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("ArcadeRS Shooter", 800, 600)
    	.position_centered().opengl().build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();

    renderer.set_draw_color(Color::RGB(0,0,0));
    renderer.clear();
    renderer.present();

    thread::sleep_ms(3000);

}

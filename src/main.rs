extern crate sdl2;

use sdl2::pixels::Color;

mod events;
use ::events::Events;

fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("ArcadeRS Shooter", 800, 600)
    	.position_centered().opengl().build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();


    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();
        renderer.present();

        thread::sleep_ms(20);
    }

}

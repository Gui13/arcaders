extern crate sdl2;

use sdl2::pixels::Color;


// #[macro_use] asks the compiler to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro
// expansion happens before the concept of namespace even starts to _exist_ in
// the compilation timeline.

#[macro_use]
mod events;

// list of the keyboard events we're interested in.
// See events.rs for the macro definition.
struct_events! {
    keyboard: {
        key_escape: Escape
    },
    else: {
        quit: Quit { .. }
    }
}

use sdl2::render::Renderer;

/// Bundles the Phi abstractions in a single structure which
/// can be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}


fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("ArcadeRS Shooter", 800, 600)
                      .position_centered()
                      .opengl()
                      .build()
                      .unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.now.key_escape == Some(true) || events.now.quit {
            break;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

    }

}

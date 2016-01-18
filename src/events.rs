
extern crate sdl2;
use self::sdl2::EventPump;


pub struct Events {
    pump: EventPump,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
        	pump: pump,

        	quit: false,
        	key_escape: false,
        }
    }
}
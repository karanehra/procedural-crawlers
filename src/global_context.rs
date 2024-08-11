use std::cell::RefCell;

use sdl2::{render::Canvas, video::Window};

use crate::constants;

pub struct GlobalContext {
    pub canvas: RefCell<Canvas<Window>>,
    pub sdl_context: RefCell<sdl2::Sdl>,
}

impl GlobalContext {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", constants::WIDTH, constants::HEIGHT)
            .position(0, 0)
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window.into_canvas().build().expect("could not make a canvas");

        GlobalContext {
            canvas: RefCell::new(canvas),
            sdl_context: RefCell::new(sdl_context),
        }
    }
}

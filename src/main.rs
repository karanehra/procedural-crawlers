mod constants;
mod global_context;
mod util;

use global_context::GlobalContext;
use lazy_static::lazy_static;
use log::info;
use sdl2::{gfx::primitives::DrawRenderer, sys::KeyCode};
use std::{cell::RefCell, collections::HashSet};

use sdl2::{pixels::Color, ttf::Sdl2TtfContext};
use util::FPSCounter;

lazy_static! {
    static ref ttf_context: Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
}

#[derive(Debug, Copy, Clone)]
struct BodyNode {
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    constraint_distance: f32,
}

struct BodyManager {
    nodes: Vec<BodyNode>,
}

impl BodyManager {
    pub fn new() -> Self {
        let first_node = BodyNode {
            x: 50.0,
            y: 50.0,
            size: 10.0,
            color: Color::RGB(255, 0, 0),
            constraint_distance: 12.0,
        };
        BodyManager {
            nodes: vec![first_node],
        }
    }

    pub fn add_node(&mut self) {
        let last_node = self.nodes.last().unwrap();
        let new_node = BodyNode {
            x: last_node.x + last_node.constraint_distance,
            y: last_node.y,
            size: 10.0,
            color: Color::RGB(255, 0, 0),
            constraint_distance: 12.0,
        };
        self.nodes.push(new_node);
    }

    pub fn get_circles(&self) -> Vec<((i16, i16), i16)> {
        let mut ans = Vec::new();

        for node in self.nodes.iter() {
            ans.push(((node.x as i16, node.y as i16), node.size as i16));
        }

        ans
    }

    pub fn move_body(&mut self, offset: (f32, f32)) {
        let first_node = &mut self.nodes[0];

        first_node.x += offset.0;
        first_node.y += offset.1;

        let mut new_nodes = Vec::new();

        new_nodes.push(first_node.clone());

        for i in 1..self.nodes.len() {
            let node = &self.nodes[i];
            let prev_node = &self.nodes[i - 1];

            let vect_x = node.x - prev_node.x;
            let vect_y = node.y - prev_node.y;

            let vect_len = (vect_x.powi(2) + vect_y.powi(2)).sqrt();

            let normalized: (f32, f32) = (vect_x / vect_len, vect_y / vect_len);

            let constrained_vect = (
                normalized.0 * node.constraint_distance,
                normalized.1 * node.constraint_distance,
            );

            let new_position = (
                prev_node.x + constrained_vect.0,
                prev_node.y + constrained_vect.1,
            );
            new_nodes.push(BodyNode {
                x: new_position.0,
                y: new_position.1,
                size: node.size,
                color: node.color,
                constraint_distance: node.constraint_distance,
            });
        }

        self.nodes = new_nodes;
    }
}

fn main() {
    let logger_env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");

    env_logger::init_from_env(logger_env);

    info!("Starting game");

    let global_context = GlobalContext::new();

    let mut canvas = global_context.canvas.borrow_mut();
    let sdl_context = global_context.sdl_context.borrow();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let font = ttf_context.load_font("./assets/roboto.ttf", 16).unwrap();

    let mut fps_counter = FPSCounter::new();

    let mut bm = BodyManager::new();

    let move_speed = 1.0;

    let mut current_speed = (0.0, 0.0);

    let mut pressed_keys: HashSet<sdl2::keyboard::Keycode> = HashSet::new();

    'running: loop {
        fps_counter.tick();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Space),
                    ..
                } => {
                    bm.add_node();
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    pressed_keys.insert(keycode);
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    pressed_keys.remove(&keycode);
                }
                _ => {}
            }
        }

        for key in pressed_keys.iter() {
            match *key {
                sdl2::keyboard::Keycode::W => {
                    current_speed.1 = -move_speed;
                }
                sdl2::keyboard::Keycode::A => {
                    current_speed.0 = -move_speed;
                }
                sdl2::keyboard::Keycode::S => {
                    current_speed.1 = move_speed;
                }
                sdl2::keyboard::Keycode::D => {
                    current_speed.0 = move_speed;
                }
                _ => {}
            }
        }

        bm.move_body(current_speed);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for (pos, size) in bm.get_circles() {
            canvas
                .circle(pos.0, pos.1, size, Color::RGB(255, 0, 0))
                .unwrap();
        }

        fps_counter.render_to_canvas(&font, &mut canvas);

        canvas.present();
    }
}

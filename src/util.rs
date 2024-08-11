use std::time::{Duration, Instant};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

pub struct FPSCounter {
    frame_count: u32,
    fps: f64,
    last_time: Instant,
}

impl FPSCounter {
    pub fn new() -> Self {
        FPSCounter {
            frame_count: 0,
            fps: 0.0,
            last_time: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        let current_time = Instant::now();
        let elapsed = current_time - self.last_time;

        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_time = current_time;
        }

        self.frame_count += 1;
    }

    pub fn render_to_canvas(&self, font: &Font, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let surface = font
            .render(&format!("FPS: {:.2}", self.fps))
            .blended(Color::BLACK)
            .unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let target = Rect::new(10, 10, surface.width(), surface.height());
        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(target).unwrap();
        canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

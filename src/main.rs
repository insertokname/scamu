use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let window_options = WindowOptions {
        resize: true,
        scale_mode: minifb::ScaleMode::Stretch,
        ..WindowOptions::default()
    };

    let mut window = Window::new("SCAM", WIDTH, HEIGHT, window_options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();

        let red = ((time.sin() * 0.5 + 0.5) * 255.0) as u32;
        let green = (((time + 2.094).sin() * 0.5 + 0.5) * 255.0) as u32;
        let blue = (((time + 4.188).sin() * 0.5 + 0.5) * 255.0) as u32;

        let color = (red << 16) | (green << 8) | blue;

        for i in 0..buffer.len() {
            buffer[i] = color;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

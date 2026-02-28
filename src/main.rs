use minifb;
use minifb::{Key, Window, WindowOptions};
use meval;
use std::io;

struct GraphPlotter {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    scale_x: f32,
    scale_y: f32,
    offset_x: f32,
    offset_y: f32
}

impl GraphPlotter {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0xFFFFFF; width * height],
            scale_x: 50.0,
            scale_y: 50.0,
            offset_x: width as f32 / 2.0,
            offset_y: height as f32 / 2.0 ,
        }
    }

    fn clear(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    fn draw_axis(&mut self) {
        let axis_color = 0x666666;

        //y axis
        let x_center = self.offset_x as usize;
        for y in 0..self.height {
            self.draw_pixel(x_center, y, axis_color);

            if x_center > 0 {
                self.draw_pixel(x_center - 1, y, axis_color);
            }
        }

        //x axis
        let y_center = self.offset_y as usize;
        for x in 0..self.width {
            self.draw_pixel(x, y_center, axis_color);

            if y_center > 0 {
                self.draw_pixel(x, y_center - 1, axis_color);
            }
        }
    }

    fn draw_grid(&mut self, color: u32, period_x: usize, period_y: usize) {
        let width = self.width;
        let height = self.height;

        for pixel_y in 0..height / period_y {
            for pixel_x in 0..width {
                self.draw_pixel(pixel_x, self.height / 2 - (pixel_y + period_y * pixel_y) / 2, color);
                self.draw_pixel(pixel_x, self.height / 2 + (pixel_y + period_y * pixel_y) / 2, color);
            }
        }

        for pixel_x in 0..width / period_x {
            for pixel_y in 0..height {
                self.draw_pixel(self.width / 2 - (pixel_x + period_x * pixel_x) / 2, pixel_y, color);
                self.draw_pixel(self.width / 2 + (pixel_x + period_x * pixel_x) / 2, pixel_y, color);
            }
        }
    }

    fn update_window(&self, window: &mut Window) {
        window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    fn draw_func(&mut self, expression: &String, color: u32) {
        let serialized_expression: meval::Expr = expression.parse().unwrap();
        let func = serialized_expression.bind("x").unwrap();

        let mut previous_pixel_y: Option<usize> = None;

        for pixel_x in 0..self.width {
            let x = (pixel_x as f32 - self.offset_x) / self.scale_x;
            let y = func(x as f64);
            let pixel_y = (self.offset_y - y as f32 * self.scale_y) as usize;

            self.draw_pixel(pixel_x, pixel_y, color);
            self.draw_pixel(pixel_x + 1, pixel_y + 1, color);

            //There we are connecting two pixels with one line

            if let Some(prev_y_val) = previous_pixel_y {
                let start_y = prev_y_val.min(pixel_y);
                let end_y = prev_y_val.max(pixel_y);

                for y_line in start_y..=end_y {
                    self.draw_pixel(pixel_x - 1, y_line, color);
                    self.draw_pixel(pixel_x, y_line + 1, color);
                }
            }

            previous_pixel_y = Some(pixel_y);
        }
    }
}

fn main() {
    let width = 1000; //px
    let height = 800; //px

    let mut period_for_grid_x = 150; //px
    let mut period_for_grid_y = 150; //px

    println!("Hello, this program draws a graphic of function, that you enter (e.g. sin(x) * cos(x)");
    println!("Please, enter your function expression: ");

    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Something went wrong");
    let expression = expression.trim().to_string();

    let color = 0;

    let mut plotter = GraphPlotter::new(width, height);

    let mut window = Window::new(
        "Graphics",
        width, height,
        WindowOptions::default(),
    ).expect("Something went wrong while opening the window!");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Up) {
            plotter.clear(0xFFFFFF);
            plotter.scale_y += 2.0;
            period_for_grid_y += 2;
        }
        if window.is_key_down(Key::Down) && plotter.scale_y > 10.0 {
            plotter.clear(0xFFFFFF);
            plotter.scale_y -= 2.0;
            if period_for_grid_y > 6 {
                period_for_grid_y -= 2;
            }
        }
        if window.is_key_down(Key::Right) {
            plotter.clear(0xFFFFFF);
            plotter.scale_x += 2.0;
            period_for_grid_x += 2;
        }
        if window.is_key_down(Key::Left) && plotter.scale_x > 10.0 {
            plotter.clear(0xFFFFFF);
            plotter.scale_x -= 2.0;
            if period_for_grid_x > 5 {
                period_for_grid_x -= 2;
            }
        }
        if window.is_key_down(Key::R) {
            plotter.clear(0xFFFFFF);
            plotter.scale_x = 50.0;
            plotter.scale_y = 50.0;
            period_for_grid_y = 100;
            period_for_grid_x = 156;
        }

        plotter.draw_axis();
        plotter.draw_grid(color, period_for_grid_x, period_for_grid_y);

        plotter.draw_func(&expression, color);

        plotter.update_window(&mut window);
    }
}
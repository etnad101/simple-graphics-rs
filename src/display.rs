use crate::errors::*;
use crate::fonts::Font;
use minifb::{Key, Window, WindowOptions};

pub type Color = u32;

pub const BLACK: Color = 0x00000000;
pub const WHITE: Color = 0x00FFFFFF;

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub struct Display {
    window: Window,
    buffer: Vec<Color>,
    width: usize,
    height: usize,
    background: Color,
    font: Option<Font>,
}

#[allow(dead_code)]
impl Display {
    pub fn new(
        title: &str,
        width: usize,
        height: usize,
        topmost: bool,
    ) -> Result<Self, minifb::Error> {
        let buffer: Vec<Color> = vec![0; width * height];

        let window_options = WindowOptions {
            borderless: true,
            title: true,
            resize: false,
            scale: minifb::Scale::X2,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost,
            transparency: false,
            none: false,
        };

        let window = Window::new(title, width, height, window_options)?;

        Ok(Display {
            window,
            buffer,
            width,
            height,
            background: BLACK,
            font: None,
        })
    }

    pub fn render(&mut self) -> Result<(), minifb::Error> {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)?;

        Ok(())
    }

    pub fn limit_frame_rate(&mut self, rate: Option<std::time::Duration>) {
        self.window.limit_update_rate(rate);
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn draw_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Color,
    ) -> Result<(), DrawOutOfBoundsError> {
        if x > self.width {
            return Err(DrawOutOfBoundsError::X(x));
        }

        if y > self.height {
            return Err(DrawOutOfBoundsError::Y(y));
        }

        let index = (y * self.width) + x;
        self.buffer[index] = color;

        Ok(())
    }

    pub fn render_text(
        &mut self,
        text: &str,
        start_x: usize,
        start_y: usize,
    ) -> Result<(), &'static str> {
        let font = self.font.clone().unwrap();

        let mut char_x_offset = 0;
        for c in text.chars() {
            let character = font.get_character(c);

            for (pixel_y_offset, byte) in character.get_bytes().iter().enumerate() {
                for (pixel_x_offset, bit_offset) in (0..7).enumerate() {
                    let bit_set = (byte & (1 << (7 - bit_offset))) > 0;
                    let x = start_x + char_x_offset + pixel_x_offset;
                    let y = start_y + pixel_y_offset;
                    if bit_set {
                        self.draw_pixel(x, y, BLACK).unwrap();
                    }
                }
            }
            char_x_offset += 8;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background;
        }
    }

    pub fn set_buffer(&mut self, buff: Vec<Color>) {
        if self.buffer.len() != buff.len() {
            panic!("Buffers must be same size");
        }
        self.buffer = buff;
    }

    pub fn set_background(&mut self, bg: Color) {
        self.background = bg;
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = Some(font);
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

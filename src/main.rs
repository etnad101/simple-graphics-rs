use simple_graphics::{
    display::{Display, WHITE},
    fonts::Font,
};

fn main() {
    let mut display = Display::new("test window", 500, 500, true).unwrap();

    let font = Font::new("./retro-pixel-cute-mono.bdf").unwrap();

    display.set_background(WHITE);
    display.set_font(font);

    let mut x: i32 = 0;
    while display.is_open() {
        display.clear();
        let msg = format!("Test Value: {}", x);
        x = x.wrapping_add(1);
        display.render_text(&msg, 0, 0).unwrap();
        display.render().unwrap();
    }
}

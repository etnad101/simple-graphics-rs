use simple_graphics::{
    display::{Display, WHITE},
    fonts::Font,
};

fn main() {
    let mut display = Display::new("test window", 500, 500, true).unwrap();

    let font = Font::new("./fonts/retro-pixel-cute-mono.bdf").unwrap();

    display.set_background(WHITE);
    display.set_font(font);

    while display.is_open() {
        display.clear();
        display.render_text("Hello,", 0, 0).unwrap();
        display.render_text("World!", 0, 16).unwrap();
        display.render_text("Some \ntext", 100, 16).unwrap();
        display.render().unwrap();
    }
}

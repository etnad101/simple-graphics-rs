use simple_graphics::{
    display::{Display, WHITE, BLACK},
    fonts::Font,
};

fn main() {
    let mut display = Display::new("test window", 500, 500, true).unwrap();

    let font = Font::new("./fonts/retro-pixel-cute-mono.bdf").unwrap();

    display.set_background(WHITE);
    display.set_font(font);

    while display.is_open() {
        display.clear();
        display.render_text("Hello,", BLACK, 0, 0).unwrap();
        display.render_text("World!", BLACK, 0, 16).unwrap();
        display.render_text("Some \ntext", BLACK, 100, 16).unwrap();
        display.render().unwrap();
    }
}

mod radiance;
use radiance::{cursor::Cursor, document::Document, input_handler::InputHandler};
use raylib::prelude::*;

fn main() {
    let mut cursor = Cursor::new(20, 20);
    let mut document = Document::empty();

    // Initialize the Raylib window
    let (mut rl, thread) = raylib::init().size(800, 600).title("Radiance").build();

    let mut input_handler = InputHandler::new(&mut cursor);

    // Load a custom font
    let font = rl.load_font(&thread, "DejaVuSansMono.ttf").unwrap();

    // Main game loop
    while !rl.window_should_close() {
        input_handler.process_text_entered(&mut rl, &mut document.data);
        input_handler.process_key_pressed(&mut rl, &mut document.data);

        let mut d = rl.begin_drawing(&thread);

        // Catpucchin machiato base
        d.clear_background(Color::new(36, 39, 58, 255));

        // Draw the custom font text
        d.draw_text_ex(
            &font,
            &document.data,
            Vector2::new(10.0, 10.0),
            20.0,
            2.0,
            Color::new(202, 211, 245, 255),
        );
    }
}

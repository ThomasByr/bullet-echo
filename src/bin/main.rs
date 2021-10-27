extern crate find_folder;
extern crate fps_counter;
extern crate piston_window;
extern crate serde;
extern crate serde_json;
extern crate vector2d;

use bullet_echo::{game::Game, HEIGHT, WIDTH};
use fps_counter::FPSCounter;
use piston_window::*;

fn main() {
    let mut fps_counter = FPSCounter::new();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap_or_else(|_| panic!("assets folder not found"));

    let mut game = Game::new();
    game.load_map("data/smolmap.json"); // put path to map here
    game.spawn_enemy();

    // main window
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .srgb(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut glyphs = window
        .load_font(assets.join("RobotoMono-Thin.ttf"))
        .unwrap_or_else(|e| panic!("Failed to load font: {}", e));

    // main loop
    window.set_position([100, 10]);
    while let Some(event) = window.next() {
        let fps = fps_counter.tick();

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.add_key_pressed(key);
        } else if let Some(Button::Keyboard(key)) = event.release_args() {
            game.remove_key_pressed(key);
        }

        game.update(); // big stuff happening here

        window.draw_2d(&event, |c, g, device| {
            let transform = c.transform.trans(10., 20.); // transform for text

            clear([0.1; 4], g); // background
            game.draw(&c, g, &mut glyphs); // draw game state

            // render fps on window as text
            text::Text::new_color([1.; 4], 9)
                .draw(
                    &mut format!("fps : {}", fps),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap_or_else(|e| panic!("Failed to draw text: {}", e));
            glyphs.factory.encoder.flush(device); // update glyphs before rendering
        });
    }
}

#![warn(clippy::pedantic)]

mod state;
mod player;
mod obstacle;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 40;
    pub const SCREEN_HEIGHT: i32 = 25;
    pub const FRAME_DURATION: f32 = 30.0;
    pub const DRAGON_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];
    pub use crate::state::*;
    pub use crate::obstacle::*;
    pub use crate::player::*;

}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon Enhanced")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}

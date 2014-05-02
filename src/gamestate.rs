use sdl2::mouse::Mouse;
use sdl2::render::Renderer;

use util;

/// Specifies a set of methods that all game states should implement.
pub trait GameState {
  #[allow(unused_variable)]
  fn render(&self, r: &Renderer) -> util::SdlResult {
    Ok(())
  }

  #[allow(unused_variable)]
  fn on_mouse_down(&mut self, mouse: Mouse, x: int, y: int) {
  }
}

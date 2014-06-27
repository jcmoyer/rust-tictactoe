use sdl2::SdlResult;
use sdl2::mouse::Mouse;
use sdl2::render::Renderer;

/// Specifies a set of methods that all game states should implement.
pub trait GameState {
  #[allow(unused_variable)]
  fn render<T>(&self, r: &Renderer<T>) -> SdlResult<()> {
    Ok(())
  }

  #[allow(unused_variable)]
  fn on_mouse_down(&mut self, mouse: Mouse, x: int, y: int) {
  }
}

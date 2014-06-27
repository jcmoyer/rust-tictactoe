#![crate_id="TicTacToe#0.1"]

extern crate sdl2;

use sdl2::video::Window;
use sdl2::video::PosCentered;
use sdl2::video::OpenGL;

use sdl2::render::Renderer;
use sdl2::render::DriverAuto;
use sdl2::render::Accelerated;

use gamestate::GameState;

mod game;
mod render;
mod gamestate;
mod states;

fn main() {
  sdl2::init(sdl2::InitVideo);

  // TODO: since we're going to fail! anyways these could be converted to .unwrap()
  let window = match Window::new("Tic Tac Toe: The Rustening", PosCentered, PosCentered, 800, 600, OpenGL) {
    Ok(window) => window,
    Err(err)   => fail!("failed to create window: {}", err)
  };

  let renderer = match Renderer::from_window(window, DriverAuto, Accelerated) {
    Ok(renderer) => renderer,
    Err(err)     => fail!("failed to create renderer: {}", err)
  };

  // TODO: state machine
  let mut state = states::playstate::PlayState::new();

  'main: loop {
    use sdl2::event::poll_event;
    use sdl2::event::{QuitEvent, MouseButtonDownEvent, NoEvent};

    'event: loop {
      match poll_event() {
        QuitEvent(..) => break 'main,
        MouseButtonDownEvent(_, _, _, button, x, y) => state.on_mouse_down(button, x, y),
        NoEvent => break 'event,
        _ => {}
      }
    }

    state.render(&renderer).unwrap();
    renderer.present();
  }

  sdl2::quit();
}

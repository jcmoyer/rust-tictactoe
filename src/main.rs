#![crate_id="TicTacToe#0.1"]

extern crate sdl2;

use sdl2::video::Window;
use sdl2::video::PosCentered;
use sdl2::video::OpenGL;

use sdl2::render::Renderer;
use sdl2::render::DriverAuto;
use sdl2::render::Accelerated;

use sdl2::rect::Rect;
use sdl2::pixels::RGB;

use sdl2::mouse::Mouse;

use game::field::{Mark, PlayField, FieldArea};
use game::field::{X, O};

use util::SdlResult;

mod game;
mod util;
mod render;

/// Specifies a set of methods that all game states should implement.
trait GameState {
  #[allow(unused_variable)]
  fn render(&self, r: &Renderer) -> SdlResult {
    Ok(())
  }

  #[allow(unused_variable)]
  fn on_mouse_down(&mut self, mouse: Mouse, x: int, y: int) {
  }
}

/// Represents the winner in a game of Tic-tac-toe.
enum WinState {
  Win(Mark),
  Draw,
  Neither
}

struct PlayState {
  field: PlayField,
  area: FieldArea,
  turn: Mark,
  winner: WinState
}
impl PlayState {
  fn new() -> ~PlayState {
    ~PlayState {
      field: PlayField::new(),
      area: FieldArea::from_rect(&Rect::new(100, 0, 600, 600)),
      turn: X,
      winner: Neither
    }
  }
}
impl GameState for PlayState {
  fn render(&self, renderer: &Renderer) -> SdlResult {
    try!(renderer.set_draw_color(RGB(0, 0, 0)));
    try!(renderer.clear());

    try!(render::draw_field(renderer, &self.area));
    try!(renderer.set_draw_color(RGB(0, 255, 0)));

    for row in range(0, 3) {
      for col in range(0, 3) {
        let pt = self.area.project(row, col);

        self.field.get_cell_xy(col as uint, row as uint).map(|mark| {
          // TODO: Try to remove a layer of indirection here
          match mark {
            Some(X) => render::draw_x(renderer, pt.x, pt.y, self.area.cell_width() as i32, self.area.cell_height() as i32),
            Some(O) => render::draw_o(renderer, pt.x, pt.y, self.area.cell_width() as i32, self.area.cell_height() as i32),
            _ => Ok(())
          }
        }).unwrap().unwrap();
      }
    }
    Ok(())
  }

  fn on_mouse_down(&mut self, button: Mouse, x: int, y: int) {
    use sdl2::mouse::LeftMouse;

    match self.winner {
      Neither => {},
      _ => {
        self.field = PlayField::new();
        self.winner = Neither;
      }
    }

    if button == LeftMouse {
      match self.area.unproject(x, y) {
        Some((row,col)) => {
          match self.field.get_mut_cell_xy(col as uint, row as uint) {
            Some(r) => match *r {
              None => {
                *r = Some(self.turn);
                self.turn = self.turn.opposite();
              },
              _ => println!("space already occupied")
            },
            None => fail!("invalid coords! this should never happen!")
          }
        },
        None => println!("clicked out of bounds")
      }
    }

    self.winner = self.check_winner();
    match self.winner {
      Win(mark) => {
        println!("{} wins!", mark);
      },
      Draw => {
        println!("draw")
      },
      Neither => {}
    }
  }
}

impl PlayState {
  fn check_winner(&self) -> WinState {
    for line in self.field.lines() {
      match line.is_win() {
        Some(mark) => {
          return Win(mark)
        },
        None => {}
      };
    };

    if self.field.occupied_count() == 9 {
      return Draw
    };

    return Neither
  }
}

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
  let mut state = PlayState::new();

  loop {
    use sdl2::event::poll_event;
    use sdl2::event::{QuitEvent, MouseButtonDownEvent};

    match poll_event() {
      QuitEvent(..) => break,
      MouseButtonDownEvent(_, _, _, button, x, y) => state.on_mouse_down(button, x, y),
      _ => {}
    }

    state.render(renderer).unwrap();
    renderer.present();
  }

  sdl2::quit();
}

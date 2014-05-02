use sdl2::render::Renderer;

use sdl2::rect::Rect;
use sdl2::pixels::RGB;

use sdl2::mouse::Mouse;

use game::field::{Mark, PlayField, FieldArea};
use game::field::{X, O};

use util::SdlResult;

use render;
use gamestate;

/// Represents the winner in a game of Tic-tac-toe.
enum WinState {
  Win(Mark),
  Draw,
  Neither
}

pub struct PlayState {
  field: PlayField,
  area: FieldArea,
  turn: Mark,
  winner: WinState
}

impl PlayState {
  pub fn new() -> PlayState {
    PlayState {
      field: PlayField::new(),
      area: FieldArea::from_rect(&Rect::new(100, 0, 600, 600)),
      turn: X,
      winner: Neither
    }
  }
}

impl gamestate::GameState for PlayState {
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

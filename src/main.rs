#![crate_id="TicTacToe#0.1"]

extern crate sdl2;

use sdl2::video::Window;
use sdl2::video::PosCentered;
use sdl2::video::OpenGL;

use sdl2::render::Renderer;
use sdl2::render::DriverAuto;
use sdl2::render::Accelerated;

use sdl2::rect::Point;
use sdl2::pixels::RGB;

use sdl2::mouse::Mouse;

type SdlResult = Result<(), ~str>;

/// Draws a player X mark at a given location.
fn draw_x(r: &Renderer, x: i32, y: i32, w: i32, h: i32) -> SdlResult {
  try!(r.draw_line(Point::new(x, y), Point::new(x + w, y + h)));
  try!(r.draw_line(Point::new(x + w, y), Point::new(x, y + h)));
  Ok(())
}

/// Draws a player O mark at a given location.
fn draw_o(r: &Renderer, x: i32, y: i32, w: i32, h: i32) -> SdlResult {
  use std::f32::consts::PI;

  let segs = 16;
  let segr: f32 = 2f32 * PI / segs as f32;

  let cx: f32 = x as f32 + w as f32 / 2f32;
  let cy: f32 = y as f32 + h as f32 / 2f32;
  // technically we have to support oval shapes because the field is dynamically sized
  let rx: f32 = w as f32 / 2f32;
  let ry: f32 = h as f32 / 2f32;

  for i in range(0, segs) {
    let a = i as f32;
    let b = a + 1f32;
    let pt1 = Point::new(
      (cx + (a * segr).cos() * rx) as i32,
      (cy + (a * segr).sin() * ry) as i32);
    let pt2 = Point::new(
      (cx + (b * segr).cos() * rx) as i32,
      (cy + (b * segr).sin() * ry) as i32);
    try!(r.draw_line(pt1, pt2));
  }
  Ok(())
}

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

/// Represents the players in a Tic-tac-toe game.
#[deriving(Clone, Show)]
enum Mark {
  X, O
}

impl Mark {
  /// Returns the opposite mark.
  fn opposite(self) -> Mark {
    match self {
      X => O,
      O => X
    }
  }
}

/// Represents the winner in a game of Tic-tac-toe.
enum WinState {
  Win(Mark),
  Draw,
  Neither
}

/// Stores the board data associated with a game of Tic-tac-toe.
struct PlayField {
  cells: [Option<Mark>, ..9]
}

/// Represents a line of the Tic-tac-toe board. A line consists of three cells.
struct PlayLine {
  cells: [Option<Mark>, ..3]
}

impl PlayLine {
  /// Determines if this line counts as a win. Returns the winner wrapped in an
  /// Option, with None meaning that this line is not a winning one.
  fn is_win(&self) -> Option<Mark> {
    let (xs, os) = self.cells.iter().fold((0, 0), |(xs, os), &place| {
      match place {
        Some(X) => (xs + 1, os),
        Some(O) => (xs, os + 1),
        None    => (xs, os)
      }
    });
    if xs == 3 {
      Some(X)
    } else if os == 3 {
      Some(O)
    } else {
      None
    }
  }
}

enum LineIteratorState {
  Row(uint),
  Column(uint),
  LtrDiagonal,
  RtlDiagonal,
  Finished
}

struct LineIterator<'a> {
  field: &'a PlayField,
  state: LineIteratorState
}

impl<'a> LineIterator<'a> {
  fn from_field<'a>(f: &'a PlayField) -> LineIterator<'a> {
    LineIterator {
      field: f,
      state: Row(0)
    }
  }
}

impl<'a> Iterator<PlayLine> for LineIterator<'a> {
  fn next(&mut self) -> Option<PlayLine> {
    match self.state {
      Row(place) => {
        self.state = if place + 1 < 3 {
          Row(place + 1)
        } else {
          Column(0)
        };
        self.field.horizontal_line(place)
      },
      Column(place) => {
        self.state = if place + 1 < 3 {
          Column(place + 1)
        } else {
          LtrDiagonal
        };
        self.field.vertical_line(place)
      },
      LtrDiagonal => {
        self.state = RtlDiagonal;
        Some(self.field.ltr_diagonal_line())
      },
      RtlDiagonal => {
        self.state = Finished;
        Some(self.field.rtl_diagonal_line())
      },
      Finished => None
    }
  }
}

impl PlayField {
  fn new() -> PlayField {
    PlayField {
      cells: [None, ..9]
    }
  }

  /// Returns the number of occupied cells.
  fn occupied_count(&self) -> int {
    self.cells.iter().fold(0, |s, mark| if mark.is_some() { s + 1 } else { s })
  }

  /// Returns an iterator over the `PlayLine`s in this field.
  fn lines<'a>(&'a self) -> LineIterator<'a> {
    LineIterator::from_field(self)
  }

  fn horizontal_line(&self, row: uint) -> Option<PlayLine> {
    PlayField::map_xy(0, row).map(|ix| PlayLine {
      // there's probably a trick to do this with a slice
      cells: [self.cells[ix], self.cells[ix + 1], self.cells[ix + 2]]
    })
  }

  fn vertical_line(&self, col: uint) -> Option<PlayLine> {
    PlayField::map_xy(col, 0).map(|ix| PlayLine {
      cells: [self.cells[ix], self.cells[ix + 3], self.cells[ix + 6]]
    })
  }

  fn ltr_diagonal_line(&self) -> PlayLine {
    PlayLine {
      cells: [self.cells[0], self.cells[4], self.cells[8]]
    }
  }

  fn rtl_diagonal_line(&self) -> PlayLine {
    PlayLine {
      cells: [self.cells[2], self.cells[4], self.cells[6]]
    }
  }

  fn map_xy(x: uint, y: uint) -> Option<uint> {
    if y < 3 && y < 3 {
      Some(y * 3 + x)
    } else {
      None
    }
  }

  fn get_cell_xy(&self, x: uint, y: uint) -> Option< Option<Mark> > {
    PlayField::map_xy(x, y).map(|ix| self.cells[ix])
  }
  
  fn get_mut_cell_xy<'a>(&'a mut self, x: uint, y: uint) -> Option< &'a mut Option<Mark> > {
    match PlayField::map_xy(x, y) {
      Some(ix) => Some(&mut self.cells[ix]),
      None     => None
    }
  }
}

/// Represents an on-screen area where a Tic-tac-toe board can be interacted
/// with.
struct FieldArea {
  x: int, y: int,
  w: int, h: int
}

impl FieldArea {
  /// Returns the width of an individual cell.
  fn cell_width(&self) -> int {
    self.w / 3
  }

  /// Returns the height of an individual cell.
  fn cell_height(&self) -> int {
    self.h / 3
  }

  /// Computes the row at a given on-screen Y-coordinate.
  fn unproject_row(&self, y: int) -> Option<int> {
    if y >= self.y {
      match (y - self.y) / self.cell_height() {
        ix if ix > 2 => None,
        ix           => Some(ix)
      }
    } else {
      None
    }
  }

  /// Computes the column at a given on-screen X-coordinate.
  fn unproject_col(&self, x: int) -> Option<int> {
    if x >= self.x {
      match (x - self.x) / self.cell_width() {
        ix if ix > 2 => None,
        ix           => Some(ix)
      }
    } else {
      None
    }
  }

  /// Computes the given row and column given on-screen X and Y coordinates.
  fn unproject(&self, x: int, y: int) -> Option<(int, int)> {
    let row = match self.unproject_row(y) {
      Some(row) => row,
      None      => return None
    };
    let col = match self.unproject_col(x) {
      Some(col) => col,
      None      => return None
    };
    Some((row, col))
  }

  /// Computes the on-screen X and Y coordinates for a given row and column.
  fn project(&self, row: int, col: int) -> Point {
    let x = (self.x + col * self.cell_width()) as i32;
    let y = (self.y + row * self.cell_height()) as i32;
    Point::new(x, y)
  }
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
      area: FieldArea {
        x: 100, y: 0, w: 600, h: 600
      },
      turn: X,
      winner: Neither
    }
  }
}
impl GameState for PlayState {
  fn render(&self, renderer: &Renderer) -> SdlResult {
    try!(renderer.set_draw_color(RGB(0, 0, 0)));
    try!(renderer.clear());

    try!(renderer.set_draw_color(RGB(255, 0, 0)));

    for i in range(1, 3) {
      let x1 = self.area.x as i32;
      let y1 = (self.area.y + self.area.cell_height() * i) as i32;
      let x2 = (self.area.x + self.area.w) as i32;
      let y2 = (self.area.y + self.area.cell_height() * i) as i32;
      try!(renderer.draw_line(Point::new(x1, y1), Point::new(x2, y2)));
    }
    for i in range(1, 3) {
      let x1 = (self.area.x + self.area.cell_width() * i) as i32;
      let y1 = self.area.y as i32;
      let x2 = (self.area.x + self.area.cell_width() * i) as i32;
      let y2 = (self.area.y + self.area.h) as i32;
      try!(renderer.draw_line(Point::new(x1, y1), Point::new(x2, y2)));
    }

    try!(renderer.set_draw_color(RGB(0, 255, 0)));

    for row in range(0, 3) {
      for col in range(0, 3) {
        let pt = self.area.project(row, col);

        self.field.get_cell_xy(col as uint, row as uint).map(|mark| {
          // TODO: Try to remove a layer of indirection here
          match mark {
            Some(X) => draw_x(renderer, pt.x, pt.y, self.area.cell_width() as i32, self.area.cell_height() as i32),
            Some(O) => draw_o(renderer, pt.x, pt.y, self.area.cell_width() as i32, self.area.cell_height() as i32),
            _ => Ok(())
          }
        }).unwrap().unwrap();
      }
    }
    Ok(())
  }

  fn on_mouse_down(&mut self, button: Mouse, x: int, y: int) {
    match self.winner {
      Neither => {},
      _ => {
        self.field = PlayField::new();
        self.winner = Neither;
      }
    }

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

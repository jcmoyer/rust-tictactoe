/// Represents the players in a Tic-tac-toe game.
#[deriving(Clone, Show)]
pub enum Mark {
  X, O
}

impl Mark {
  /// Returns the opposite mark.
  pub fn opposite(self) -> Mark {
    match self {
      X => O,
      O => X
    }
  }
}

/// Represents a line of the Tic-tac-toe board. A line consists of three cells.
struct PlayLine {
  cells: [Option<Mark>, ..3]
}

impl PlayLine {
  /// Determines if this line counts as a win. Returns the winner wrapped in an
  /// Option, with None meaning that this line is not a winning one.
  pub fn is_win(&self) -> Option<Mark> {
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

/// Stores the board data associated with a game of Tic-tac-toe.
pub struct PlayField {
  cells: [Option<Mark>, ..9]
}

impl PlayField {
  pub fn new() -> PlayField {
    PlayField {
      cells: [None, ..9]
    }
  }

  /// Returns the number of occupied cells.
  pub fn occupied_count(&self) -> int {
    self.cells.iter().fold(0, |s, mark| if mark.is_some() { s + 1 } else { s })
  }

  /// Returns an iterator over the `PlayLine`s in this field.
  pub fn lines<'a>(&'a self) -> LineIterator<'a> {
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

  pub fn map_xy(x: uint, y: uint) -> Option<uint> {
    if y < 3 && y < 3 {
      Some(y * 3 + x)
    } else {
      None
    }
  }

  pub fn get_cell_xy(&self, x: uint, y: uint) -> Option<Option<Mark>> {
    PlayField::map_xy(x, y).map(|ix| self.cells[ix])
  }
  
  pub fn get_mut_cell_xy<'a>(&'a mut self, x: uint, y: uint) -> Option<&'a mut Option<Mark>> {
    match PlayField::map_xy(x, y) {
      Some(ix) => Some(&mut self.cells[ix]),
      None     => None
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

use sdl2::SdlResult;
use sdl2::render::Renderer;
use sdl2::rect::Point;
use sdl2::pixels::RGB;

use game::field::FieldArea;

/// Draws a player X mark at a given location.
pub fn draw_x<T>(r: &Renderer<T>, x: i32, y: i32, w: i32, h: i32) -> SdlResult<()> {
  try!(r.draw_line(Point::new(x, y), Point::new(x + w, y + h)));
  try!(r.draw_line(Point::new(x + w, y), Point::new(x, y + h)));
  Ok(())
}

/// Draws a player O mark at a given location.
pub fn draw_o<T>(r: &Renderer<T>, x: i32, y: i32, w: i32, h: i32) -> SdlResult<()> {
  use std::f32::consts::PI;

  let segs = 16;
  let segr: f32 = 2f32 * PI / segs as f32;

  let cx: f32 = x as f32 + w as f32 / 2f32;
  let cy: f32 = y as f32 + h as f32 / 2f32;
  // technically we have to support oval shapes because the field is dynamically sized
  let rx: f32 = w as f32 / 2f32;
  let ry: f32 = h as f32 / 2f32;

  for i in range(0i, segs) {
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

pub fn draw_field<T>(r: &Renderer<T>, area: &FieldArea) -> SdlResult<()> {
  try!(r.set_draw_color(RGB(255, 0, 0)));
  for i in range(1i, 3i) {
    let x1 = area.x as i32;
    let y1 = (area.y + area.cell_height() * i) as i32;
    let x2 = (area.x + area.w) as i32;
    let y2 = (area.y + area.cell_height() * i) as i32;
    try!(r.draw_line(Point::new(x1, y1), Point::new(x2, y2)));
  }
  for i in range(1i, 3i) {
    let x1 = (area.x + area.cell_width() * i) as i32;
    let y1 = area.y as i32;
    let x2 = (area.x + area.cell_width() * i) as i32;
    let y2 = (area.y + area.h) as i32;
    try!(r.draw_line(Point::new(x1, y1), Point::new(x2, y2)));
  }
  Ok(())
}

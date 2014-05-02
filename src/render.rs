use sdl2::render::Renderer;
use sdl2::rect::Point;

use util::SdlResult;

mod util;

/// Draws a player X mark at a given location.
pub fn draw_x(r: &Renderer, x: i32, y: i32, w: i32, h: i32) -> SdlResult {
  try!(r.draw_line(Point::new(x, y), Point::new(x + w, y + h)));
  try!(r.draw_line(Point::new(x + w, y), Point::new(x, y + h)));
  Ok(())
}

/// Draws a player O mark at a given location.
pub fn draw_o(r: &Renderer, x: i32, y: i32, w: i32, h: i32) -> SdlResult {
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

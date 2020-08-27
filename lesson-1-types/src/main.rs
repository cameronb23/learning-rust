struct Point {
  x: f32,
  y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
  // A rectangle can be specified by where the top left and bottom right
  // corners are in space.
  top_left: Point,
  bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f32 {
  // get lengths of each side
  let length: f32 = r.bottom_right.x - r.top_left.x;
  let width: f32 = r.bottom_right.y - r.top_left.y;

  println!("Rectangle: Length {}, Width {}", length, width);

  // calculate area with absolute vals
  width.abs() * length.abs()
}

/**
 * Add a function square which takes a Point and a f32
 * as arguments, and returns a Rectangle with its lower left
 * corner on the point, and a width and height
 * corresponding to the f32.
 **/
fn square(point: Point, dist: f32) -> Rectangle {
  // point is lower left
  // we need top left and bottom right
  // top left
  let top_left: Point = Point {
    y: point.y + dist,
    ..point
  };

  // bottom right
  let bottom_right: Point = Point {
    x: point.x + dist,
    ..point
  };

  Rectangle {
    top_left,
    bottom_right,
  }
}

fn main() {
  let point: Point = Point { x: 10.3, y: 0.4 };

  // Make a new point by using struct update syntax to use the fields of our
  // other one
  let bottom_right = Point { x: 5.2, ..point };

  // Destructure the point using a `let` binding
  let Point {
    x: top_edge,
    y: left_edge,
  } = point;

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point {
      x: left_edge,
      y: top_edge,
    },
    bottom_right: bottom_right,
  };

  println!(
    "Rectangle: ({}, {}), ({}, {})",
    _rectangle.top_left.x,
    _rectangle.top_left.y,
    _rectangle.bottom_right.x,
    _rectangle.bottom_right.y
  );

  println!("Area: {}", rect_area(_rectangle));

  let test_point: Point = Point { x: 10.0, y: 10.0 };
  let new_rect = square(test_point, 10.0);

  println!(
    "TopL ({}, {}) BottomR  ({}, {})",
    new_rect.top_left.x, new_rect.top_left.y, new_rect.bottom_right.x, new_rect.bottom_right.y
  );
}

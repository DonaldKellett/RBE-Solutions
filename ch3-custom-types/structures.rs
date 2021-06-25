#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
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

fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle {
    top_left: Point { x: x1, y: y1 },
    bottom_right: Point { x: x2, y: y2 }
  } = rect;
  (y2 - y1) * (x2 - x1)
}

fn square(lower_left: Point, side_length: f32) -> Rectangle {
  let Point { x, y } = lower_left;
  Rectangle {
    top_left: Point { x, y: y - side_length },
    bottom_right: Point { x: x + side_length, y  }
  }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let my_rect = Rectangle {
      top_left: Point { x: 2.0, y: 4.0 },
      bottom_right: Point { x: 7.0, y: 15.0 }
    };
    println!("Rectangle area: {}", rect_area(my_rect));

    let my_square = square(Point { x: 10.0, y: 13.0 }, 5.0);
    println!("Square top left: ({}, {})",
      my_square.top_left.x, my_square.top_left.y);
    println!("Square bottom right: ({}, {})",
      my_square.bottom_right.x, my_square.bottom_right.y);
}


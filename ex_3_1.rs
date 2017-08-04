// There are three types of structs:
// 1. A unit struct
// 2. Tuple structs (essentially named tuples)
// 3. Structs with fields (classic C structs)

// 1. Unit struct
struct Nil;

// 2. Tuple struct
struct Pair(i32, f32);

// 3. Struct with two fields
#[derive (Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
#[allow (dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Instantiate a `Point`
    let p = Point { x: 1.2, y: 3.5 };
    println!("Point is: {:?}", p);

    // Access the fields of a point
    println!("Point coords: {}, {}", p.x, p.y);

    // Destructure the point using a let binding
    let Point { x: my_x, y: my_y } = p;

    let r = Rectangle { p1: Point { x: my_y, y: my_x }, p2: p };

    let unit_struct = Nil;

    let tuple_struct = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", tuple_struct.0, tuple_struct.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = tuple_struct;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rect coords are {:?} and {:?}", r.p1, r.p2);
    println!("area of rect is {}", rect_area(r));
}

fn rect_area(r: Rectangle) -> f64 {
    let Rectangle { p1: Point { x: x_1, y: y_1 }, p2: Point { x: x_2, y: y_2 } } = r;

    let width = (x_1 - x_2).abs();
    let length = (y_1 - y_2).abs();

    width * length
}


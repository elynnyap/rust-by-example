use std::fmt;
use std::mem;

fn main() {

    /* LITERALS AND OPERATORS */

    // variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let a_int = 5i32; // suffix annotation

    let default_float = 3.14; // default for float is f64
    let default_int = 32; // default for int is i32

    let mut mutable = 12; // mutable i32

    // Error -- the type of a variable can't be changed
     //mutable = true;

    /* TUPLES */
    // tuples can have different types
    let long_tuple = (3.0f64, 4u32, -4i64, 
                      'a', true, -2i16);

    // tuple indexing
    println!("First element is {}", long_tuple.0);
    println!("Second element is {}", long_tuple.1);

    let pair = (1, true);
    // we can print tuples
    println!("tuple: {:?}", pair);
    println!("tuple: {:?}", reverse(pair));

    // use a comma to distinguish a one-element tuple from a parenthesized value
    println!("this is a tuple: {:?}", (5u32,));
    println!("this is a value: {}", (5u32));
    
    // print the matrix using its Display trait
    let m = Matrix ( 1.2, 1.3, 1.4, 1.5 );
    println!("Matrix:\n {}", m);
    println!("Transposed matrix:\n {}", transpose(m));

    /* SLICES */

    // Fixed size array (signature is superfluous
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elems can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // arrays are stack-allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices can point to a sub-section of the array
    println!("borrow a sectino of an array as a slice");
    analyze_slice(&ys[1..5]);
}

/* TUPLES */
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer) // leave out semicolon bc we want to return the value of this expression
}

fn transpose(m: Matrix) -> Matrix {
    Matrix (m.0, m.2, m.1, m.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Add the Display trait to Matrix
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

/* SLICES */
// This function borrows a slice
// Slices are similar to arrays but their size is not known at compile time
// A slice is a two-word object: the first word is a pointer to the data
// and the second word is the length of the slice.
fn analyze_slice(s: &[i32]) {
    println!("First element of the slice is {}", s[0]);
    println!("Length of the slice is {}", s.len());
}


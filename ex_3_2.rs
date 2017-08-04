#![allow(dead_code)]

enum Person {
    // any variant which is valid as a struct is also valid as an enum
    // 1. unit-like
    Engineer,
    Scientist,
    // 2. tuple-like
    Height(u32),
    Weight(u32),
    // 3. struct with fields
    Info { name: String, height: u32 }
}

// takes a Person as an arg and returns nothing
fn inspect(p: Person) {
    // match must cover all possible cases for an enum
    match p {
        Person::Engineer => println!("is an engineer"),
        Person::Scientist => println!("is an scientist"),
        // destructure `i` from inside the enum
        Person::Height(i) => println!("has height {:?}", i),
        Person::Weight(i) => println!("has weight {:?}", i),
        // destructure Info into name and height
        Person::Info { name, height } => println!("has name {:?} and height {:?}", name, height),
    }
}
        
fn main() {
    let eng = Person::Engineer;
    let sc = Person::Scientist;
    let person1 = Person::Height(150);
    let person2 = Person::Weight(50);
    let person3 = Person::Info { name: String::from("Bob"), height: 150 };

    inspect(eng);
    inspect(sc);
    inspect(person1);
    inspect(person2);
    inspect(person3);

    /* LINKED LISTS */
    let mut lst = List::new();

    lst = lst.prepend(1);
    lst = lst.prepend(2);
    lst = lst.prepend(3);

    println!("list length is {:?}", lst.len());
    println!("list elems are {:?}", lst.stringify());
}

enum List {
    Nil, // empty node
    Cons(u32, Box<List>), // tuple that wraps an elem and pointer to next node.
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, new_head: u32) -> List {
        List::Cons(new_head, Box::new(self))
    }

    fn stringify(&self) -> String {
        match *self {
            List::Nil => String::from("Nil"),
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
        }
    }

    fn len(&self) -> u32 {
        match *self {
            List::Nil => 0,
            List::Cons(_, ref tail) => 1 + tail.len(),
        }
    }
}

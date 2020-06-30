use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f32,
    imaginary: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ real: {}, imag: {} }}", self.real, self.imaginary)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{0:X}{2:X}{2:X}",
            self.red, self.green, self.blue
        )
    }
}

fn main() {
    let c = Complex {
        real: 2.1,
        imaginary: 3.4,
    };
    println!("Number: {}", c);

    // running display for the List struct
    let v = List(vec![1, 2, 3, 4]);
    println!("List: {}", v);

    // formatting the color struct for better output
    let c = Color {
        red: 121,
        green: 243,
        blue: 16,
    };
    println!("Color: {}", c);
}

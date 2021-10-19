pub fn run() {
    //let integer: Point<i32> = Point {x: 5, y: 10};
    let float: Point<f32> = Point {x: 1.0, y:4.0};

}

// generic function
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }

// generic struct
struct Point<T> {
    x: T,
    y: T
}

// generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Point with type f32 only can use this method
// https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    } 
}
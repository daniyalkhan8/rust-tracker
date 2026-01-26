#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y
}

impl <X, Y> Point<X, Y>
where
    X: Into<f64> + Copy,
    Y: Into<f64> + Copy,
{
    fn x(&self) -> &X {
        &self.x
    }

    fn distance_from_origin(&self) -> f64 {
        let x: f64 = self.x.into();
        let y: f64 = self.y.into();
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

impl <X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, point: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for value in list {
        if value > largest {
            largest = value
        }
    }

    largest
}

fn main() {
    let list1 = [1, 4, 2, 5, 3, 17];
    let list1_largest = largest(&list1);
    println!("{list1_largest}");

    let list2 = ["c", "y", "q", "a"];
    let list2_largest = largest(&list2);
    println!("{list2_largest}");

    let point1 = Point {x: 5, y: 10.4};
    let point1_x = point1.x();
    let point1_y = point1.y;
    let point1_dof = point1.distance_from_origin();
    println!("{point1_x}, {point1_y}");
    println!("{point1_dof}");

    let point2 = Point {x: 17.98, y: 21.4};
    let point2_x = point2.x();
    let point2_y = point2.y;
    let point2_dof = point2.distance_from_origin();
    println!("{point2_x}, {point2_y}");
    println!("{point2_dof}");

    let point3 = point2.mixup(point1);
    println!("{point3:?}");
}

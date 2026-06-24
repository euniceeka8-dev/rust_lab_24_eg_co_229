// Lab Session 2 - Control Flow & Pattern Matching
// Exercise 2.3: Enums & match

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // sides a, b, c
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        // TODO 2: Add Triangle arm - use Heron's formula:
        // s = (a+b+c)/2  area = sqrt(s*(s-a)*(s-b)*(s-c))
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn describe_direction(d: &Direction) {
    // TODO 3: Use match to print a sentence for each direction,
    // e.g. "Heading North - towards the mountains"
    match d {
        Direction::North => println!("Heading North - towards the mountains"),
        Direction::South => println!("Heading South - towards the valley"),
        Direction::East => println!("Heading East - towards the sunrise"),
        Direction::West => println!("Heading West - towards the sunset"),
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for s in &shapes {
        println!("{:?} -> area = {:.2}", s, area(s));
    }

    println!();
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for d in &directions {
        describe_direction(d);
    }
}
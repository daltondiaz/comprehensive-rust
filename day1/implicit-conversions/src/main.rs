fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));

    let x1: f32 = 12.0;
    let y1: i128 = 111;

    println!("{x1} * {y1} = {}", multiply(x1 as i16, y1 as i16));

    let x2: bool = true;
    let y2: bool = false;

    println!("{x2} * {y2} = {}", multiply(x2 as i16, y2 as i16));
}

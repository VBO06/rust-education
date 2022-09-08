const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}

fn main() {
    let speed = clamp_speed(10000);
    println!("{:?}", speed);
}
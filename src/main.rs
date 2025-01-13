use num_traits::ToPrimitive;

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap(); // convert f32 -> f64 using num_traits crate
    let b_f64 = b.to_f64().unwrap(); // convert f32 -> f64 using num_traits crate

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    let a_f64 = a as f64; // convert f32 -> f64 on the fly

    println!("{}", solve(a, b));
}

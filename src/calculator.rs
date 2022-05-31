use std::arch::x86_64::_MM_FROUND_TO_NEG_INF;

fn dodaj(a: f32, b: f32) -> f32 {
    return a + b;
}
fn odejmij(a: f32, b: f32) -> f32 {
    return a - b;
}
fn pomoz(a: f32, b: f32) -> f32 {
    return a * b;
}
fn podziel(a: f32, b: f32) -> f32 {
    return a / b;
}
fn main() {
    let mut znak=String::from("*");
    let mut wynik: f32;
    let mut num1: f32 = 1.1;
    let mut num2: f32 = 2.3;
    match znak.as_str() {
        "+" => wynik = dodaj(num1, num2),
        "-" => wynik = odejmij(num1, num2),
        "*" => wynik = pomoz(num1, num2),
        "/" => wynik = podziel(num1, num2),
    }
    println!("{} {} {} = {}", num1, znak, num2, wynik);
}

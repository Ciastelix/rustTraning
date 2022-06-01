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
fn wybierz(a: f32, b: f32, znak: char) -> f32 {
    let wynik: f32;
    match znak {
        '+' => wynik = dodaj(a, b),
        '-' => wynik = odejmij(a, b),
        '*' => wynik = pomoz(a, b),
        '/' => {
            if b == 0.0 {
                println!("Nie można dzielić przez 0");
                wynik = 0.0;
            } else {
                wynik = podziel(a, b);
            }
        }
        _ => {
            println!("Niepoprawny znak!");
            wynik = 0.0;
        }
    }
    return wynik;
}
fn main() {
    let znak: char = '*';
    let num1: f32 = 1.1;
    let num2: f32 = 2.3;
    println!("{} {} {} = {}", num1, znak, num2, wybierz(num1, num2, znak));
}

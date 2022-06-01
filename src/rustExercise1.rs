fn expected_minues_in_oven() -> i16 {
    return 40;
}
fn remaining_time_in_oven(time: i16) -> i16 {
    return expected_minues_in_oven()-time;
}
fn preparation_time_in_minutes(numer_of_layers: i16) -> i16 {
    return 2 * numer_of_layers;
}
fn elapsed_time_in_minues(numer_of_layers: i16, time_of_cooking: i16) -> i16 {
    return preparation_time_in_minutes(numer_of_layers) + remaining_time_in_oven(time_of_cooking);
}
fn main() {
    let n_layers: i16 = 3;
    let time_of_cooking: i16 = 30;
    println!(
        "Time you sent making lasange is: {} minutes",
        elapsed_time_in_minues(n_layers, time_of_cooking)
    );
}

use std::io;
fn main() {
    let mut weight_str: String = String::new();
    println!("Enter Your Weight:");
    io::stdin().read_line(&mut weight_str).unwrap();

    let weight = weight_str.trim().parse().unwrap();
    let weight_mars = calc_weight3(weight);
    println!("your weight on mars will be {}kg", weight_mars)
    // // calc_weight(70.0);
    // calc_weight1();
    // let age = calc_weight2();
    // let age_2 = calc_weight3(70.0);
    // println!("your age : {}", age);
    // println!("your age_2 : {}", age_2);
}

// fn calc_weight1() {
//     println!(
//         "calc_weight1 your weight on mars : {} ",
//         (70.0 * 3.73) / 9.81
//     );
// }

// fn calc_weight2() -> f32 {
//     return (70.0 * 3.73) / 9.81;
// }

fn calc_weight3(weight: f32) -> f32 {
    (weight * 3.73) / 9.81
}

use std::io;

fn main() {
    println!("Hello, world!");

    let mut s = String::from("Original value");
    println!("Pre some_func: '{}'", s);
    some_func(&mut s);
    println!("Post some_func: '{}'", s);


    println!("Type the weight to convert (kg):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight_input: f32 = input.trim().parse().unwrap();
    let mut res = calculate_weight_on_mars(weight_input);
    println!("Mars weight is {}kg", res);
    res = res * 1000.0;
    println!("Or {}g", res);
}

fn some_func(p: &mut String) {
    // 'p' borrows ownership (& sign) from other methods
    p.push_str("!!!");
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    // weight represented in kilos
    (weight / 9.81) * 3.711
}

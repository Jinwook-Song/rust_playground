fn main() {
    let weight: f32 = 100.0;
    let mut mars_weight = calculate_weight_on_mars(weight);

    println!("{}kg Weight on Mars: {}kg", weight, mars_weight);

    mars_weight *= 1000.0;

    println!("{}g", mars_weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}

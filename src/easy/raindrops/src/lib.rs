pub fn raindrops(n: u32) -> String {
    let mut raindrop = "".to_owned();
    if n % 3 == 0 {
        raindrop.push_str("Pling");
    } if n % 5 == 0 {
        raindrop.push_str("Plang");
    } if n % 7 == 0 {
        raindrop.push_str("Plong");
    } if raindrop == "" {
        return n.to_string();
    }
    raindrop
}

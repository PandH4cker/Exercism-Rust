pub fn factors(n: u64) -> Vec<u64> {
    let mut factors_vec = vec![];
    let mut number_copy = n.clone();
    while number_copy % 2 == 0 {
        factors_vec.push(2);
        number_copy /= 2;
    }
    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        while number_copy % i == 0 {
            factors_vec.push(i);
            number_copy /= i;
        }
    }
    if number_copy > 2 {
        factors_vec.push(number_copy);
    }
    factors_vec
}

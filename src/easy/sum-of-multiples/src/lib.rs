pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| factors.iter().any(|modulo| {
        if *modulo != 0 {
            return x % modulo == 0;
        } false
    })).sum()
}

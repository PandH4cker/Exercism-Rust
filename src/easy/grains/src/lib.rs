pub fn square(s: u32) -> u64 {
    if s >= 1 && s <=64 {
        return 2_u64.pow(s - 1);
    }
    panic!("Square must be between 1 and 64")
}

pub fn total() -> u64 {
    (2_u128.pow(64) - 1) as u64
}

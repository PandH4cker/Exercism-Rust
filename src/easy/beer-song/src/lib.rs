pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else {
        format!(
            "{nth_bottle} bottle{plural_bottle} of beer on the wall, {nth_bottle} bottle{plural_bottle} of beer.\nTake {it} down and pass it around, {bottle_left} bottle{plural_bottle_left} of beer on the wall.\n",
            nth_bottle = n.to_string(),
            it = if n - 1 == 0 { "it" } else { "one" },
            bottle_left = if n - 1 != 0 { (n - 1).to_string() } else { "no more".to_owned() },
            plural_bottle = if n > 1 { "s" } else { "" },
            plural_bottle_left = if n - 1 > 1 || n - 1 == 0 { "s" } else { "" }
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut sing_str = "".to_owned();
    for i in (end..=start).rev() {
        sing_str.push_str(&*verse(i));
        if i != end {
            sing_str.push_str("\n")
        }
    }
    sing_str
}

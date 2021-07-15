use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|chunk| format!(
            "For want of a {first} the {second} was lost.",
            first = chunk[0],
            second = chunk[1]
        ))
        .chain(once(
            format!("And all for the want of a {first}.", first = list[0])
        ))
        .collect::<Vec<String>>()
        .join("\n")
}

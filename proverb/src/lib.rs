use std::iter::once; 

pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() { return String::new() }

    let last_line = format!("And all for the want of a {}.", list[0]);
    (0..list.len() - 1)
        .map(|i| format!("For want of a {} the {} was lost.", list[i], list[i+1]))
        .chain(once(last_line))
        .collect::<Vec<String>>()
        .join("\n")
}

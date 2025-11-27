// src/lib/utils.rs
// Converting a JavaScript utility to Rust

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn get_initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect()
}

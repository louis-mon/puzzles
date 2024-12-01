use std::fs;

#[macro_export]
macro_rules! read_day_input {
    () => {{
        let replaced = file!().replace(".rs", ".input");
        fs::read_to_string(replaced).unwrap()
    }};
}

pub fn parse_u32(s: &str) -> u32 {
    s.parse().unwrap()
}

pub fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}

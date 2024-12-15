#[macro_export]
macro_rules! read_day_input {
    () => {{
        use std::fs;
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

pub fn parse_f64(s: &str) -> f64 {
    s.parse().unwrap()
}

pub fn parse_long(s: &str) -> i64 {
    s.parse().unwrap()
}

pub fn parse_u128(s: &str) -> u128 {
    s.parse().unwrap()
}

pub fn split_2<'a>(s: &'a str, p: &str) -> (&'a str, &'a str) {
    let mut split = s.split(p);
    (split.next().unwrap(), split.next().unwrap())
}

pub fn split_2_str(s: &str, p: &str) -> (String, String) {
    let (s1, s2) = split_2(s, p);
    (String::from(s1), String::from(s2))
}

pub fn to_tuple2<T>(mut v: impl Iterator<Item = T>) -> (T, T) {
    (v.next().unwrap(), v.next().unwrap())
}

pub fn to_tuple2_str<'a>(mut v: impl Iterator<Item = &'a str>) -> (String, String) {
    (
        String::from(v.next().unwrap()),
        String::from(v.next().unwrap()),
    )
}

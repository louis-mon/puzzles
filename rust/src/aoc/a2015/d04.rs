pub fn run() {
    println!("{}", solve(String::from("yzbqklnj")));
}

fn solve(s: String) -> i32 {
    let mut i = 0;
    loop {
        let comb = format!("{s}{i}");
        let m = md5::compute(comb);
        if m.starts_with(&[0; 3]) {
            return i;
        }
        i += 1;
    }
}

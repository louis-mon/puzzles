pub fn run() {
    solve();
}

fn base_b(b: i64) -> i64 {
    10_i64.pow(b.ilog10() + 1)
}

fn solve() {
    let n: i64 = 100000000;
    let mut res = 0;
    for b in 1..n {
        let base_b1 = base_b(b);
        let a1 = 2 * b - base_b1;
        let a0 = b * b - b;
        let delta = a1 * a1 - 4 * a0;
        if delta <= 0 {
            continue;
        }
        let delta_root = delta.isqrt();
        if delta_root * delta_root != delta {
            continue;
        }
        let s1 = -a1 - delta_root;
        let s2 = -a1 + delta_root;
        for a2 in [s1, s2] {
            if a2 <= 0 || a2 % 2 != 0 {
                continue;
            }
            let a = a2 / 2;
            let c = a * base_b1 + b;
            let nb_digits = c.ilog10();
            let b_digits = b.ilog10();
            println!("({a}+{b})^2 = {c}  ({b_digits},{nb_digits})");
            res += c;
            if (a + b).pow(2) != c {
                panic!("error")
            }
        }
    }
    println!("{res}");
}

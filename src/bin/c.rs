use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;

fn main() {
    input!{
        n_as_chars: Chars
    }

    let n_as_i32 = n_as_chars.into_iter().map(|x| x as i32 - 48).collect::<Vec<i32>>();

    let mut ans = -1;

    for i in 0..n_as_i32.len(){
        for combination in (0..n_as_i32.len()).combinations(n_as_i32.len() - i) {
            let sum = combination.into_iter().fold(0, |acc, i| acc + n_as_i32[i]);
            if sum % 3 == 0 {ans = i as i32; break}
        }
        if ans >= 0 {break}
    }
    println!("{}", ans)
}

use proconio::input;
use num_integer::sqrt;

fn main() {
    input! {
        n : i32,
        a : [i32; n]
    }


    let primes = make_primes(1001);

    let mut max_gcd = (0, 0);


    for p in primes{
        let mut gcd = 0;
        for &test in &a {
            if p > test { continue; }
            if test % p == 0 { gcd += 1}
        }
        if gcd > max_gcd.1 {
            max_gcd.0 = p;
            max_gcd.1 = gcd;
        }
    }

    println!("{}", max_gcd.0);

}

fn make_primes(n: usize) -> Vec<i32> {
    let mut prime_table = vec![true; n];
    prime_table[0] = false;
    prime_table[1] = false;

    for i in 2..n{
        if i * i > sqrt(n) {
            break;
        }
        if !prime_table[i] {
            continue;
        }
        let mut j = i + i;
        loop {
            if j >= n {break;}
            prime_table[j] = false;
            j = j + i;
        }
    }

    let mut primes = vec![];

    for i in 2..n {
        if prime_table[i]{
            primes.push(i as i32)
        }
    }

    return primes;
}
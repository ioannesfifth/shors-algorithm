use std::mem;
use num_bigint::BigUint;
use rand::Rng;

fn biguint(x: u32) -> BigUint {
    return BigUint::from(x);
}

fn guess_g() -> u32 {
    let mut rng = rand::thread_rng();
    
    let arbritrary_limit = 1000;
    return rng.gen_range(2..arbritrary_limit);
}

fn calculate_guessed_factor(g: BigUint, p: u32) -> Result<BigUint, String> {
    if p % 2 != 0 {
        return Err("p is not even".to_string());
    } else {
        Ok(g.pow(p/2) + biguint(1))
    }
}

fn find_p(g: BigUint, n: u32) -> Result<u32, String> {
    let mut p = 1;
    for tmp_p in 1..999 {
        if (g.pow(tmp_p) - biguint(1)) % n == biguint(0) {
            p = tmp_p;
            break
        } else {
            continue
        };
    }
    if p != 1 {
        Ok(p)
    } else {
        Err("p too hard to calculate".to_string())
    }
}

fn find_gcd(a: BigUint, b: BigUint) -> Result<BigUint, String> {
    if b == biguint(0) {
       return Err("Cannot divide by 0".to_string());
    } 
    
    let mut a = a;
    let mut b = b;
    let mut r;
    println!("a: {}, b: {}", a, b); 
    return loop {
        if a < b {
            mem::swap(&mut a, &mut b);
        }
        r = a % b.clone();

        if r == biguint(0) {
            break Ok(b)
        }

        a = b;
        b = r.clone();
        println!("a: {}, b: {}, r: {}", a, b, r);
    };
}
 
fn main() {
    println!("Start!");
    let n = 314191;
    println!("===== Guessing g");
    let g = biguint(guess_g());
    println!("g: {}", g);
    println!("===== Finding p");
    let p = find_p(g.clone(), n);
    match p {
        Ok(p) => {
            println!("p: {}", p);
            println!("===== Calculating g**p/2 + 1");
            let guess = calculate_guessed_factor(g, p);
            
            println!(
                "{}", match guess {
                    Ok(guess) => {
                        println!("===== Finding gcd");
                        let gcd = find_gcd(biguint(n), guess);
                        match gcd {
                            Ok(gcd) => {
                                println!("gcd: {}", gcd);
                                let factor = n / gcd;
                                println!("===== Checking if factor");
                                if n % factor.clone() == biguint(0) {
                                    if factor != biguint(n) && factor != biguint(1) {
                                        format!("{} is a factor", factor)
                                    } else {
                                        format!("Cannot find factor")
                                    }
                                } else {
                                    format!("{} is not a factor", factor)
                                }
                            },
                            Err(gcd) => format!("Error: {gcd}"),
                        }
                    },
                    Err(guess) => format!("Error: {guess}"),
                }
            )
        },
        Err(p) => println!("Error: {}", p),
    }
}

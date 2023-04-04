use std::mem;
// use text_io::read;

use rand::Rng;

fn guess_g() -> u32 {
    let mut rng = rand::thread_rng();
    
    let arbritrary_limit = 10;
    return rng.gen_range(2..arbritrary_limit);
}

fn calculate_guessed_factor(g: u32, p: u32) -> Result<u32, String> {
    if p % 2 != 0 {
        return Err("p is not even".to_string());
    } else {
        Ok(g.pow(p/2) + 1)
    }
}

fn find_p(g: u32, n: u32) -> Result<u32, String> {
    let mut p = 1;
    for tmp_p in 1..10 {
        if (g.pow(tmp_p) - 1) % n == 0 {
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

fn find_gcd(a: u32, b: u32) -> Result<u32, String> {
    if b == 0 {
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
        r = a % b;

        if r == 0 {
            break Ok(b)
        }

        a = b;
        b = r;
        println!("a: {}, b: {}, r: {}", a, b, r);
    };
}
 
fn main() {
    println!("Start!");
    // print!("Input: ");
    // let n: u32 = read!();
    let n: u32 = 15;
    println!("===== Guessing g");
    let g = guess_g();
    println!("g: {}", g);
    println!("===== Finding p");
    let p = find_p(g, n);
    match p {
        Ok(p) => {
            println!("p: {}", p);
            println!("===== Calculating g**p/2 + 1");
            let guess = calculate_guessed_factor(g, p);
            
            println!(
                "{}", match guess {
                    Ok(guess) => {
                        println!("===== Finding gcd");
                        let gcd = find_gcd(n, guess);
                        match gcd {
                            Ok(gcd) => {
                                println!("gcd: {}", gcd);
                                let factor = n / gcd;
                                println!("===== Checking if factor");
                                if n % factor == 0 {
                                    if  factor != n && factor != 1 {
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

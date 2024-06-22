use anyhow::Ok;
// use rand::Rng;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()_";

pub fn process_gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("upper won't be empty"))
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("upper won't be empty"))
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("upper won't be empty"))
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("upper won't be empty"))
    }

    for _ in 0..(length - password.len() as u8) {
        // let idx = rng.gen_range(0..chars.len());
        // password.push(chars[idx] as char);
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }
    // TODO: make sure the password has at least one of each type
    password.shuffle(&mut rng);

    // output password strength in stderr
    let password = String::from_utf8(password)?;
    println!("{}", password);
    let estimate = zxcvbn(&password, &[])?;
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}

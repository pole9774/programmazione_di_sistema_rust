// checks if a string is made only of digits and spaces
fn check_format(code: &str) -> bool {
    for c in code.chars() {
        if !(c.is_digit(10) || c.is_whitespace()) {
            return false;
        }
    }
    return true;
}

// removes spaces
fn remove_spaces(code: &str) -> String {
    code.chars().filter(|c| !c.is_whitespace()).collect()
}

// sum of char and integer (luhn)
fn char_lsum(c: char, n: i32, flag: bool) -> Option<i32> {
    let mut i = c.to_digit(10)? as i32;
    if flag {
        i = 2 * i;
        if i > 9 {
            i = i - 9;
        }
        Some(i + n)
    } else {
        Some(i + n)
    }
}

// computes the Luhn sum of a string made only of digits
fn str_lsum(code: &str) -> i32 {
    let mut res = 0;
    let mut i = 0;

    if code.len() % 2 == 0 {
        // even -> 1st, 3rd, ... (i even) * 2
        for c in code.chars() {
            if i % 2 == 0 {
                res = char_lsum(c, res, true).unwrap();
            } else {
                res = char_lsum(c, res, false).unwrap();
            }
            i = i + 1;
        }
    } else {
        // odd -> 2nd, 4th, ... (i odd) * 2
        for c in code.chars() {
            if i % 2 == 0 {
                res = char_lsum(c, res, false).unwrap();
            } else {
                res = char_lsum(c, res, true).unwrap();
            }
            i = i + 1;
        }
    }

    return res;
}

// checks Luhn validity
pub fn is_valid(code: &str) -> bool {
    
    if !(check_format(code)) {
        return false;
    }

    let s = remove_spaces(code);

    if s.len() < 2 {
        return false;
    }

    if str_lsum(&s) % 10 == 0 {
        return true;
    } else {
        return false;
    }
}

// cargo test
// cargo test -- --ignored


// alternative:
/*
pub fn is_valid(code: &str) -> bool {

    let mut digits = String::new();
    for c in code.chars() {
        match c {
            c if c.is_digit(10) => digits.push(c),
            ' ' => (),
            _ => {
                return false;
            }
        }
    }

    if digits.len() < 2 {
        return false;
    }

    let mut sum = 0;
    for (idx, sdigit) in digits.chars().rev().enumerate() {
        let num = sdigit.to_digit(10).unwrap();

        sum += if idx % 2 == 0 {
            num
        } else if num * 2 > 9 {
            num * 2 - 9
        } else {
            num * 2
        };
    }

    sum % 10 == 0
}
*/

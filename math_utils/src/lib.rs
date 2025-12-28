pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

pub fn division(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

pub fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
use math_utils::{addition, subtraction, multiplication, division, factorial, greatest_common_divisor, is_prime};
mod tests {
    use super::*;
    #[test]
    fn test_addition() {
        assert_eq!(addition(2, 3), 5);
    }
    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction(5, 3), 2);
    }
    #[test]
    fn test_multiplication() {
        assert_eq!(multiplication(4, 3), 12);
    }
    #[test]
    fn test_division() {
        assert_eq!(division(6, 3), Some(2));
        assert_eq!(division(6, 0), None);
    }
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(greatest_common_divisor(48, 18), 6);
    }
    #[test]
    fn test_is_prime() {
        assert!(is_prime(7));
        assert!(!is_prime(10));
    }
}
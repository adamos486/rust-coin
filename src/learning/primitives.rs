pub struct Primitives {}

impl Primitives {
    pub fn explore_integers() {
        let i8_example: i8 = 127;
        let i16_example: i16 = 32_767;
        let i32_example: i32 = 2_147_483_647;
        let i64_example: i64 = 9_223_372_036_854_775_807;
        let i128_example: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

        println!("Signed Integers:");
        println!("i8: {i8_example} (range -128 to 127)");
        println!("i16: {i16_example} (range -32,768 to 32,767)");
        println!("i32: {i32_example} (range -2,147,483,648 to 2,147,483,647)");
        println!("i64: {i64_example} (range -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)");
        println!("i128: {i128_example} (range -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687)");

        println!("\nUnsigned Integers:");
        let u8_example: u8 = 255;
        let u16_example: u16 = 65_535;
        let u32_example: u32 = 4_294_967_295;
        let u64_example: u64 = 18_446_744_073_709_551_615;
        let u128_example: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

        println!("\nUnsigned Integers:");
        println!("u8: {u8_example} (range: 0 to 255)");
        println!("u16: {u16_example} (range: 0 to 65,535)");
        println!("u32: {u32_example} (range: 0 to 4,294,967,295)");
        println!("u64: {u64_example} (range: very large)");
        println!("u128: {u128_example} (range: enormous)");

        let isize_example: isize = 9000;
        let usize_example: usize = 9000;

        println!("\nPlatform-Dependent Integers:");
        println!("isize: {isize_example} (range: depends on platform)");
        println!("usize: {usize_example} (range: depends on platform)");
        println!("On this system, usize is {} bytes", std::mem::size_of::<usize>());

        let a: i32 = 10;
        let b: i32 = 3;
        println!("\nInteger Operations:");
        println!("Additions: {a} + {b} = {}", a+b);
        println!("Subtraction: {a} - {b} = {}", a-b);
        println!("Multiplication: {a} * {b} = {}", a*b);
        println!("Division: {a} / {b} = {}", a/b);
        println!("Remainder: {a} % {b} = {}", a%b);

        println!("\nInteger Overflow:");
        let max_u8: u8 = u8::MAX;
        println!("Max u8: {max_u8}");
        // Uncomment the following line to see overflow in action
        //let overflow_u8 = max_u8 + 1;
        println!("Wrapping: 255_u8 + 1 = {}", u8::wrapping_add(max_u8, 1));

        println!("\nMemory size (bytes):");
        println!("u8: {} byte", std::mem::size_of::<u8>());
        println!("u16: {} bytes", std::mem::size_of::<u16>());
        println!("u32: {} bytes", std::mem::size_of::<u32>());
        println!("u64: {} bytes", std::mem::size_of::<u64>());
        println!("u128: {} bytes", std::mem::size_of::<u128>());

        let pi_scaled_u64: u64 = 31_415_926_535;  // First 10 digits fit in u64
        let pi_scaled_u128: u128 = 31_415_926_535_897_932_384; // First 20 digits fit in u128

        println!("Pi to 20 digits: 3.14159265358979323846...");
        println!("Stored as u64 (10 digits): {pi_scaled_u64}");
        println!("Stored as u128 (20 digits): {pi_scaled_u128}");
    }

    pub fn explore_floating_point() {
        println!("\nFLOATING POINT TYPES\n---------------");
        let f32_example: f32 = std::f32::consts::PI;
        let f64_example: f64 = std::f64::consts::PI;

        println!("f32: {f32_example} (single precision floating point)");
        println!("f64: {f64_example} (double precision floating point)");

        println!("\nPrecision comparison:");
        println!("f32 stored as: {f32_example:.20}");
        println!("f64 stored as: {f64_example:.20}");

        let a: f64 = 10.5;
        let b: f64 = 3.2;
        println!("\nFloating Point Operations:");
        println!("Addition: {a} + {b} = {}", a + b);
        println!("Subtraction: {a} - {b} = {}", a - b);
        println!("Multiplication: {a} * {b} = {}", a * b);
        println!("Division: {a} / {b} = {}", a / b);

        println!("\nSpecial floating point values:");
        println!("Infinity: {}", f32::INFINITY);
        println!("Negative Infinity: {}", f32::NEG_INFINITY);
        println!("Not a Number (NaN): {}", f32::NAN);
        println!("Is NaN equal to itself? {}", f32::NAN.is_nan());
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn explore_booleans() {
        println!("\nBOOLEAN TYPES\n---------------");

        let t: bool = true;
        let f: bool = false;

        println!("Boolean values: {t}, {f}");

        println!("\nLogical operations:");
        println!("AND: true && false = {}", true && false);
        println!("OR: true || false = {}", true || false);
        println!("NOT: !true = {}", !true);

        let a = 10;
        let b = 20;
        println!("\nBoolean with comparisons:");
        println!("{a} == {b}: {}", a == b);
        println!("{a} != {b}: {}", a != b);
        println!("{a} > {b}: {}", a > b);
        println!("{a} < {b}: {}", a < b);
        println!("{a} >= {b}: {}", a >= b);
        println!("{a} <= {b}: {}", a <= b);

        let number = 15;
        println!("\nControl flow with boolean (checking if {number} is even):");
        if number % 2 == 0 {
            println!("{number} is even");
        } else {
            println!("{number} is odd");
        }
    }

    pub fn explore_characters() {
        println!("\nCHARACTER TYPES\n---------------");
        let c: char = 'z';
        let z_crab: char = '🦀'; // Rust mascot!
        let heart: char = '❤';

        println!("Characters: {c}, {z_crab}, {heart}");
        println!("Size of char: {} bytes", std::mem::size_of::<char>());

        // Unicode values
        println!("\nUnicode scalar values:");
        println!("'z' as Unicode: U+{:04X}", c as u32);
        println!("'🦀' as Unicode: U+{:04X}", z_crab as u32);
        println!("'❤' as Unicode: U+{:04X}", heart as u32);

        println!("\nCharacter methods:");
        println!("Is 'z' alphabetic? {}", c.is_alphabetic());
        println!("Is 'z' numeric? {}", c.is_numeric());
        println!("Is 'z' whitespace? {}", c.is_whitespace());
        println!("Is '🦀' alphabetic? {}", z_crab.is_alphabetic());
        println!("Is '🦀' a symbol? {}", z_crab.is_ascii());
    }

    pub fn explore_compound_types() {
        println!("\nCOMPOUND TYPES\n---------------");

        println!("Tuples:");
        let tuple: (i32, f64, char) = (42, std::f64::consts::PI, 'A');
        println!("A tuple with different types: {tuple:?}");
        println!("Accessing tuple elements: {}, {}, {}", tuple.0, tuple.1, tuple.2);

        let (x, y, z) = tuple;
        println!("Destructured values: {x}, {y}, {z}");

        println!("\nArrays:");
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        let repeated: [i32; 3] = [0; 3];
        println!("An array: {array:?}");
        println!("A repeated array: {repeated:?}");
        println!("Array length: {}", array.len());
        println!("First element: {}", array[0]);
        println!("Last element: {}", array[array.len() - 1]);
        println!("Accessing array elements: {}, {}, {}", array[0], array[1], array[2]);

        println!("\nSlices:");
        let slice: &[i32] = &array[1..4];
        println!("Slice from array: {slice:?}");
        println!("Slice length: {}", slice.len());
    }

    // Helper functions for testing
    #[cfg(test)]
    fn get_max_i8() -> i8 {
        i8::MAX
    }

    #[cfg(test)]
    fn get_min_i8() -> i8 {
        i8::MIN
    }

    #[cfg(test)]
    fn get_max_u8() -> u8 {
        u8::MAX
    }

    #[cfg(test)]
    fn get_min_u8() -> u8 {
        u8::MIN
    }

    #[cfg(test)]
    fn bitwise_and(a: u8, b: u8) -> u8 {
        a & b
    }

    #[cfg(test)]
    fn bitwise_or(a: u8, b: u8) -> u8 {
        a | b
    }

    #[cfg(test)]
    fn bitwise_xor(a: u8, b: u8) -> u8 {
        a ^ b
    }

    #[cfg(test)]
    fn bitwise_not(a: u8) -> u8 {
        !a
    }

    #[cfg(test)]
    fn left_shift(a: u8, b: u32) -> u8 {
        a << b
    }

    #[cfg(test)]
    fn right_shift(a: u8, b: u32) -> u8 {
        a >> b
    }

    #[cfg(test)]
    fn checked_add(a: u8, b: u8) -> Option<u8> {
        a.checked_add(b)
    }

    #[cfg(test)]
    fn saturating_add(a: u8, b: u8) -> u8 {
        a.saturating_add(b)
    }

    #[cfg(test)]
    fn add_integers(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    fn subtract_integers(a: i32, b: i32) -> i32 {
        a - b
    }

    #[cfg(test)]
    fn multiply_integers(a: i32, b: i32) -> i32 {
        a * b
    }

    #[cfg(test)]
    fn divide_integers(a: i32, b: i32) -> i32 {
        a / b
    }

    #[cfg(test)]
    fn remainder_integers(a: i32, b: i32) -> i32 {
        a % b
    }

    #[cfg(test)]
    fn wrapping_add_u8(a: u8, b: u8) -> u8 {
        u8::wrapping_add(a, b)
    }

    #[cfg(test)]
    fn get_pi_f32() -> f32 {
        std::f32::consts::PI
    }

    #[cfg(test)]
    fn get_pi_f64() -> f64 {
        std::f64::consts::PI
    }

    #[cfg(test)]
    fn add_floats(a: f64, b: f64) -> f64 {
        a + b
    }

    #[cfg(test)]
    fn subtract_floats(a: f64, b: f64) -> f64 {
        a - b
    }

    #[cfg(test)]
    fn multiply_floats(a: f64, b: f64) -> f64 {
        a * b
    }

    #[cfg(test)]
    fn divide_floats(a: f64, b: f64) -> f64 {
        a / b
    }

    #[cfg(test)]
    fn get_infinity() -> f32 {
        f32::INFINITY
    }

    #[cfg(test)]
    fn get_neg_infinity() -> f32 {
        f32::NEG_INFINITY
    }

    #[cfg(test)]
    fn is_nan_equal_to_itself() -> bool {
        // This is intentionally using == to demonstrate that NaN is not equal to itself
        // The test will verify that this returns false
        !f32::NAN.is_nan() // This will always be false, as NaN.is_nan() is always true
    }

    #[cfg(test)]
    fn float_epsilon_comparison(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    #[cfg(test)]
    fn float_ulp_comparison(a: f64, b: f64, ulps: i64) -> bool {
        // A simple ULP (Units in the Last Place) comparison
        // This is a more sophisticated way to compare floating point numbers
        let a_bits = a.to_bits() as i64;
        let b_bits = b.to_bits() as i64;

        // Handle special cases like NaN and infinities
        if a.is_nan() || b.is_nan() {
            return false;
        }

        if a.is_infinite() && b.is_infinite() {
            return a.is_sign_positive() == b.is_sign_positive();
        }

        // Handle zero
        if a == 0.0 && b == 0.0 {
            return true;
        }

        // Make sure a and b have the same sign
        if (a_bits < 0) != (b_bits < 0) {
            return false;
        }

        // Calculate the difference in ULPs
        let diff = (a_bits - b_bits).abs();
        diff <= ulps
    }

    #[cfg(test)]
    fn get_min_positive_f64() -> f64 {
        f64::MIN_POSITIVE
    }

    #[cfg(test)]
    fn get_max_f64() -> f64 {
        f64::MAX
    }

    #[cfg(test)]
    fn is_char_alphabetic(c: char) -> bool {
        c.is_alphabetic()
    }

    #[cfg(test)]
    fn is_char_numeric(c: char) -> bool {
        c.is_numeric()
    }

    #[cfg(test)]
    fn is_char_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    #[cfg(test)]
    fn is_char_ascii(c: char) -> bool {
        c.is_ascii()
    }

    #[cfg(test)]
    fn char_to_lowercase(c: char) -> char {
        c.to_lowercase().next().unwrap()
    }

    #[cfg(test)]
    fn char_to_uppercase(c: char) -> char {
        c.to_uppercase().next().unwrap()
    }

    #[cfg(test)]
    fn logical_and(a: bool, b: bool) -> bool {
        a && b
    }

    #[cfg(test)]
    fn logical_or(a: bool, b: bool) -> bool {
        a || b
    }

    #[cfg(test)]
    fn logical_not(a: bool) -> bool {
        !a
    }

    #[cfg(test)]
    fn is_equal(a: i32, b: i32) -> bool {
        a == b
    }

    #[cfg(test)]
    fn is_not_equal(a: i32, b: i32) -> bool {
        a != b
    }

    #[cfg(test)]
    fn is_greater(a: i32, b: i32) -> bool {
        a > b
    }

    #[cfg(test)]
    fn is_less(a: i32, b: i32) -> bool {
        a < b
    }

    #[cfg(test)]
    fn is_greater_or_equal(a: i32, b: i32) -> bool {
        a >= b
    }

    #[cfg(test)]
    fn is_less_or_equal(a: i32, b: i32) -> bool {
        a <= b
    }

    #[cfg(test)]
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    #[cfg(test)]
    fn char_to_unicode(c: char) -> u32 {
        c as u32
    }

    // Helper functions for compound types
    #[cfg(test)]
    fn create_tuple() -> (i32, f64, char) {
        (42, std::f64::consts::PI, 'A')
    }

    #[cfg(test)]
    fn get_tuple_first(tuple: (i32, f64, char)) -> i32 {
        tuple.0
    }

    #[cfg(test)]
    fn get_tuple_second(tuple: (i32, f64, char)) -> f64 {
        tuple.1
    }

    #[cfg(test)]
    fn get_tuple_third(tuple: (i32, f64, char)) -> char {
        tuple.2
    }

    #[cfg(test)]
    fn create_array() -> [i32; 5] {
        [1, 2, 3, 4, 5]
    }

    #[cfg(test)]
    fn create_repeated_array(value: i32, length: usize) -> Vec<i32> {
        vec![value; length]
    }

    #[cfg(test)]
    fn get_array_element(arr: &[i32], index: usize) -> Option<i32> {
        arr.get(index).copied()
    }

    #[cfg(test)]
    fn get_array_slice(arr: &[i32], start: usize, end: usize) -> &[i32] {
        &arr[start..end]
    }

    #[cfg(test)]
    fn sum_array(arr: &[i32]) -> i32 {
        arr.iter().sum()
    }

    #[cfg(test)]
    fn concatenate_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(arr1.len() + arr2.len());
        result.extend_from_slice(arr1);
        result.extend_from_slice(arr2);
        result
    }

    // Helper functions for string operations
    #[cfg(test)]
    fn string_length(s: &str) -> usize {
        s.len()
    }

    #[cfg(test)]
    fn string_char_count(s: &str) -> usize {
        s.chars().count()
    }

    #[cfg(test)]
    fn string_concatenate(s1: &str, s2: &str) -> String {
        format!("{}{}", s1, s2)
    }

    #[cfg(test)]
    fn string_contains(s: &str, substring: &str) -> bool {
        s.contains(substring)
    }

    #[cfg(test)]
    fn string_to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[cfg(test)]
    fn string_to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    // Helper functions for type conversions
    #[cfg(test)]
    fn i32_to_f64(i: i32) -> f64 {
        i as f64
    }

    #[cfg(test)]
    fn f64_to_i32(f: f64) -> i32 {
        f as i32
    }

    #[cfg(test)]
    fn i32_to_string(i: i32) -> String {
        i.to_string()
    }

    #[cfg(test)]
    fn string_to_i32(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for explore_integers
    #[test]
    fn test_explore_integers_does_not_panic() {
        // This test just ensures the function doesn't panic
        Primitives::explore_integers();
    }

    #[test]
    fn test_integer_max_values() {
        assert_eq!(Primitives::get_max_i8(), 127);
        assert_eq!(Primitives::get_max_u8(), 255);
        assert_eq!(Primitives::get_min_i8(), -128);
        assert_eq!(Primitives::get_min_u8(), 0);
    }

    #[test]
    fn test_integer_operations() {
        let a = 10;
        let b = 3;

        assert_eq!(Primitives::add_integers(a, b), 13);
        assert_eq!(Primitives::subtract_integers(a, b), 7);
        assert_eq!(Primitives::multiply_integers(a, b), 30);
        assert_eq!(Primitives::divide_integers(a, b), 3);
        assert_eq!(Primitives::remainder_integers(a, b), 1);
    }

    #[test]
    fn test_integer_wrapping() {
        let max_u8 = u8::MAX; // 255
        assert_eq!(Primitives::wrapping_add_u8(max_u8, 1), 0);
    }

    #[test]
    fn test_bitwise_operations() {
        // Test AND
        assert_eq!(Primitives::bitwise_and(0b1010, 0b1100), 0b1000);

        // Test OR
        assert_eq!(Primitives::bitwise_or(0b1010, 0b1100), 0b1110);

        // Test XOR
        assert_eq!(Primitives::bitwise_xor(0b1010, 0b1100), 0b0110);

        // Test NOT
        assert_eq!(Primitives::bitwise_not(0b00000000), 0b11111111);
        assert_eq!(Primitives::bitwise_not(0b11111111), 0b00000000);

        // Test shifts
        assert_eq!(Primitives::left_shift(0b00000001, 1), 0b00000010);
        assert_eq!(Primitives::left_shift(0b00000001, 7), 0b10000000);

        assert_eq!(Primitives::right_shift(0b10000000, 1), 0b01000000);
        assert_eq!(Primitives::right_shift(0b10000000, 7), 0b00000001);
    }

    #[test]
    fn test_checked_and_saturating_operations() {
        // Test checked_add
        assert_eq!(Primitives::checked_add(200, 56), None); // Would overflow
        assert_eq!(Primitives::checked_add(200, 50), Some(250));

        // Test saturating_add
        assert_eq!(Primitives::saturating_add(200, 100), 255); // Saturates at max
        assert_eq!(Primitives::saturating_add(100, 50), 150); // Normal addition
    }

    #[test]
    fn test_integer_memory_sizes() {
        assert_eq!(std::mem::size_of::<u8>(), 1);
        assert_eq!(std::mem::size_of::<u16>(), 2);
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::size_of::<u64>(), 8);
        assert_eq!(std::mem::size_of::<u128>(), 16);
    }

    // Tests for explore_floating_point
    #[test]
    fn test_explore_floating_point_does_not_panic() {
        // This test just ensures the function doesn't panic
        Primitives::explore_floating_point();
    }

    #[test]
    fn test_pi_values() {
        let pi_f32 = Primitives::get_pi_f32();
        let pi_f64 = Primitives::get_pi_f64();

        // Test that PI values are approximately correct
        assert!((pi_f32 - 3.14159).abs() < 0.00001);
        assert!((pi_f64 - 3.14159265358979323846).abs() < 0.00000000000000001);

        // Test that f64 has more precision than f32
        let pi_f32_as_f64 = pi_f32 as f64;
        assert!(pi_f64 != pi_f32_as_f64);
    }

    #[test]
    fn test_float_operations() {
        let a = 10.5;
        let b = 3.2;

        // Use approximate equality for floating point
        assert!((Primitives::add_floats(a, b) - 13.7).abs() < f64::EPSILON);
        assert!((Primitives::subtract_floats(a, b) - 7.3).abs() < f64::EPSILON);
        assert!((Primitives::multiply_floats(a, b) - 33.6).abs() < f64::EPSILON);
        assert!((Primitives::divide_floats(a, b) - 3.28125).abs() < f64::EPSILON);
    }

    #[test]
    fn test_special_float_values() {
        assert!(Primitives::get_infinity().is_infinite());
        assert!(Primitives::get_infinity() > 0.0);

        assert!(Primitives::get_neg_infinity().is_infinite());
        assert!(Primitives::get_neg_infinity() < 0.0);

        // NaN is not equal to itself
        assert!(!Primitives::is_nan_equal_to_itself());
        assert!(f32::NAN.is_nan());
    }

    #[test]
    fn test_float_comparison_methods() {
        // Test epsilon comparison
        let a = 0.1 + 0.2;
        let b = 0.3;

        // Direct comparison will likely fail due to floating point precision
        assert!(a != b);

        // Epsilon comparison should pass
        assert!(Primitives::float_epsilon_comparison(a, b, 1e-10));

        // ULP comparison should also pass
        assert!(Primitives::float_ulp_comparison(a, b, 2));

        // Test with values that are further apart
        assert!(!Primitives::float_epsilon_comparison(0.1, 0.10001, 1e-6));
        assert!(Primitives::float_epsilon_comparison(0.1, 0.10001, 1e-4));
    }

    #[test]
    fn test_float_limits() {
        // Test minimum positive value
        let min_positive = Primitives::get_min_positive_f64();
        assert!(min_positive > 0.0);
        assert!(min_positive / 2.0 < min_positive); // Underflow to subnormal or zero

        // Test maximum value
        let max_value = Primitives::get_max_f64();
        assert!(max_value.is_finite());
        assert!((max_value * 2.0).is_infinite()); // Overflow to infinity
    }

    #[test]
    fn test_float_rounding_errors() {
        // Demonstrate rounding errors in floating point
        let sum = (0..10).map(|_| 0.1).sum::<f64>();
        assert!(sum != 1.0); // Due to rounding errors

        // But it should be close
        assert!((sum - 1.0).abs() < 1e-10);
    }

    // Tests for explore_booleans
    #[test]
    fn test_explore_booleans_does_not_panic() {
        // This test just ensures the function doesn't panic
        Primitives::explore_booleans();
    }

    #[test]
    fn test_logical_operations() {
        assert_eq!(Primitives::logical_and(true, true), true);
        assert_eq!(Primitives::logical_and(true, false), false);
        assert_eq!(Primitives::logical_and(false, true), false);
        assert_eq!(Primitives::logical_and(false, false), false);

        assert_eq!(Primitives::logical_or(true, true), true);
        assert_eq!(Primitives::logical_or(true, false), true);
        assert_eq!(Primitives::logical_or(false, true), true);
        assert_eq!(Primitives::logical_or(false, false), false);

        assert_eq!(Primitives::logical_not(true), false);
        assert_eq!(Primitives::logical_not(false), true);
    }

    #[test]
    fn test_short_circuit_evaluation() {
        // In Rust, boolean operations short-circuit

        // For AND, if the first operand is false, the second is not evaluated
        let mut counter = 0;
        let result = false && {
            counter += 1;
            true
        };
        assert_eq!(result, false);
        assert_eq!(counter, 0); // Second operand was not evaluated

        // For OR, if the first operand is true, the second is not evaluated
        let mut counter = 0;
        let result = true || {
            counter += 1;
            false
        };
        assert_eq!(result, true);
        assert_eq!(counter, 0); // Second operand was not evaluated
    }

    #[test]
    fn test_complex_boolean_expressions() {
        // Test more complex boolean expressions

        // De Morgan's laws
        let a = true;
        let b = false;

        // !(a && b) == !a || !b
        assert_eq!(!(a && b), !a || !b);

        // !(a || b) == !a && !b
        assert_eq!(!(a || b), !a && !b);

        // Test precedence: && has higher precedence than ||
        assert_eq!(true || false && false, true); // Equivalent to: true || (false && false)
        assert_eq!((true || false) && false, false);
    }

    #[test]
    fn test_comparison_operations() {
        let a = 10;
        let b = 20;

        assert_eq!(Primitives::is_equal(a, a), true);
        assert_eq!(Primitives::is_equal(a, b), false);

        assert_eq!(Primitives::is_not_equal(a, b), true);
        assert_eq!(Primitives::is_not_equal(a, a), false);

        assert_eq!(Primitives::is_greater(b, a), true);
        assert_eq!(Primitives::is_greater(a, b), false);

        assert_eq!(Primitives::is_less(a, b), true);
        assert_eq!(Primitives::is_less(b, a), false);

        assert_eq!(Primitives::is_greater_or_equal(b, a), true);
        assert_eq!(Primitives::is_greater_or_equal(a, a), true);
        assert_eq!(Primitives::is_greater_or_equal(a, b), false);

        assert_eq!(Primitives::is_less_or_equal(a, b), true);
        assert_eq!(Primitives::is_less_or_equal(a, a), true);
        assert_eq!(Primitives::is_less_or_equal(b, a), false);
    }

    #[test]
    fn test_even_odd() {
        assert_eq!(Primitives::is_even(2), true);
        assert_eq!(Primitives::is_even(3), false);
        assert_eq!(Primitives::is_even(0), true);
        assert_eq!(Primitives::is_even(-2), true);
        assert_eq!(Primitives::is_even(-3), false);
    }

    // Tests for explore_characters
    #[test]
    fn test_explore_characters_does_not_panic() {
        // This test just ensures the function doesn't panic
        Primitives::explore_characters();
    }

    #[test]
    fn test_char_size() {
        // In Rust, char is always 4 bytes (32 bits) to accommodate Unicode
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn test_char_to_unicode() {
        assert_eq!(Primitives::char_to_unicode('z'), 0x007A);
        assert_eq!(Primitives::char_to_unicode('🦀'), 0x1F980);
        assert_eq!(Primitives::char_to_unicode('❤'), 0x2764);
    }

    #[test]
    fn test_char_operations() {
        // Test char comparisons
        assert!('a' < 'b');
        assert!('Z' < 'a');

        // Test char to numeric conversions
        assert_eq!('A' as u8, 65);
        assert_eq!('0' as u8, 48);

        // Test numeric to char conversions
        assert_eq!(char::from(65), 'A');
        assert_eq!(char::from(48), '0');
    }

    #[test]
    fn test_char_properties() {
        // Test alphabetic property
        assert!(Primitives::is_char_alphabetic('a'));
        assert!(Primitives::is_char_alphabetic('Z'));
        assert!(!Primitives::is_char_alphabetic('1'));
        assert!(!Primitives::is_char_alphabetic(' '));

        // Test numeric property
        assert!(Primitives::is_char_numeric('1'));
        assert!(Primitives::is_char_numeric('0'));
        assert!(!Primitives::is_char_numeric('a'));
        assert!(!Primitives::is_char_numeric(' '));

        // Test whitespace property
        assert!(Primitives::is_char_whitespace(' '));
        assert!(Primitives::is_char_whitespace('\t'));
        assert!(Primitives::is_char_whitespace('\n'));
        assert!(!Primitives::is_char_whitespace('a'));

        // Test ASCII property
        assert!(Primitives::is_char_ascii('a'));
        assert!(Primitives::is_char_ascii('1'));
        assert!(Primitives::is_char_ascii(' '));
        assert!(!Primitives::is_char_ascii('🦀'));
    }

    #[test]
    fn test_char_case_conversion() {
        // Test lowercase conversion
        assert_eq!(Primitives::char_to_lowercase('A'), 'a');
        assert_eq!(Primitives::char_to_lowercase('Z'), 'z');
        assert_eq!(Primitives::char_to_lowercase('a'), 'a'); // Already lowercase
        assert_eq!(Primitives::char_to_lowercase('1'), '1'); // Not a letter

        // Test uppercase conversion
        assert_eq!(Primitives::char_to_uppercase('a'), 'A');
        assert_eq!(Primitives::char_to_uppercase('z'), 'Z');
        assert_eq!(Primitives::char_to_uppercase('A'), 'A'); // Already uppercase
        assert_eq!(Primitives::char_to_uppercase('1'), '1'); // Not a letter
    }

    #[test]
    fn test_char_string_conversion() {
        // Test char to string conversion
        assert_eq!('a'.to_string(), "a");
        assert_eq!('🦀'.to_string(), "🦀");

        // Test string to char conversion (for single character strings)
        let s = "a";
        assert_eq!(s.chars().next().unwrap(), 'a');

        let emoji = "🦀";
        assert_eq!(emoji.chars().next().unwrap(), '🦀');
    }

    // Tests for explore_compound_types
    #[test]
    fn test_explore_compound_types_does_not_panic() {
        // This test just ensures the function doesn't panic
        Primitives::explore_compound_types();
    }

    #[test]
    fn test_tuple_operations() {
        let tuple = Primitives::create_tuple();

        // Test tuple access
        assert_eq!(Primitives::get_tuple_first(tuple), 42);
        assert_eq!(Primitives::get_tuple_second(tuple), std::f64::consts::PI);
        assert_eq!(Primitives::get_tuple_third(tuple), 'A');

        // Test tuple destructuring
        let (x, y, z) = tuple;
        assert_eq!(x, 42);
        assert_eq!(y, std::f64::consts::PI);
        assert_eq!(z, 'A');
    }

    #[test]
    fn test_array_operations() {
        let array = Primitives::create_array();

        // Test array access
        assert_eq!(array[0], 1);
        assert_eq!(array[4], 5);

        // Test array length
        assert_eq!(array.len(), 5);

        // Test safe access with get
        assert_eq!(Primitives::get_array_element(&array, 2), Some(3));
        assert_eq!(Primitives::get_array_element(&array, 10), None); // Out of bounds

        // Test slicing
        let slice = Primitives::get_array_slice(&array, 1, 4);
        assert_eq!(slice, &[2, 3, 4]);

        // Test sum
        assert_eq!(Primitives::sum_array(&array), 15); // 1 + 2 + 3 + 4 + 5 = 15
    }

    #[test]
    fn test_array_creation() {
        // Test creating a repeated array
        let repeated = Primitives::create_repeated_array(42, 3);
        assert_eq!(repeated, vec![42, 42, 42]);

        // Test array concatenation
        let arr1 = [1, 2, 3];
        let arr2 = [4, 5, 6];
        let concatenated = Primitives::concatenate_arrays(&arr1, &arr2);
        assert_eq!(concatenated, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_array_bounds_checking() {
        let array = Primitives::create_array();

        // Using get() for safe access
        assert!(array.get(10).is_none());

        // Test in-bounds access
        assert!(array.get(0).is_some());
        assert!(array.get(4).is_some());

        // Test length
        assert_eq!(array.len(), 5);
    }

    // Tests for string operations
    #[test]
    fn test_string_operations() {
        // Test string length
        assert_eq!(Primitives::string_length("hello"), 5);
        assert_eq!(Primitives::string_length("🦀"), 4); // UTF-8 encoded length

        // Test character count
        assert_eq!(Primitives::string_char_count("hello"), 5);
        assert_eq!(Primitives::string_char_count("🦀"), 1); // One Unicode character

        // Test string concatenation
        assert_eq!(Primitives::string_concatenate("hello", " world"), "hello world");

        // Test string contains
        assert!(Primitives::string_contains("hello world", "world"));
        assert!(!Primitives::string_contains("hello world", "rust"));

        // Test case conversion
        assert_eq!(Primitives::string_to_uppercase("hello"), "HELLO");
        assert_eq!(Primitives::string_to_lowercase("HELLO"), "hello");
    }

    // Tests for type conversions
    #[test]
    fn test_numeric_type_conversions() {
        // Test integer to float conversion
        assert_eq!(Primitives::i32_to_f64(42), 42.0);

        // Test float to integer conversion (truncation)
        assert_eq!(Primitives::f64_to_i32(42.9), 42);
        assert_eq!(Primitives::f64_to_i32(-42.9), -42);
    }

    #[test]
    fn test_string_conversions() {
        // Test integer to string conversion
        assert_eq!(Primitives::i32_to_string(42), "42");
        assert_eq!(Primitives::i32_to_string(-42), "-42");

        // Test string to integer conversion
        assert_eq!(Primitives::string_to_i32("42"), Ok(42));
        assert_eq!(Primitives::string_to_i32("-42"), Ok(-42));
        assert!(Primitives::string_to_i32("not a number").is_err());
    }

    #[test]
    fn test_integer_operations_with_negative_numbers() {
        // Test operations with negative numbers
        assert_eq!(Primitives::add_integers(-10, -5), -15);
        assert_eq!(Primitives::subtract_integers(-10, 5), -15);
        assert_eq!(Primitives::multiply_integers(-10, -5), 50);
        assert_eq!(Primitives::multiply_integers(-10, 5), -50);
        assert_eq!(Primitives::divide_integers(-10, 5), -2);
        assert_eq!(Primitives::remainder_integers(-10, 3), -1);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_division_by_zero() {
        Primitives::divide_integers(10, 0);
    }
}
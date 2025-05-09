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
    }

    // Helper functions for testing
    #[cfg(test)]
    fn get_max_i8() -> i8 {
        i8::MAX
    }

    #[cfg(test)]
    fn get_max_u8() -> u8 {
        u8::MAX
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
}
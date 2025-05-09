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
    }
}
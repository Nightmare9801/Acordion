/// The function `abs_b` calculates the absolute value of a 16-bit signed integer using bitwise
/// operations in Rust.
/// 
/// Arguments:
/// 
/// * `i`: The parameter `i` is an input parameter of type `i16`, which is a signed 16-bit integer.
/// 
/// Returns:
/// 
/// The function `abs_b` is returning the absolute value of the input `i`, which is an `i16` integer.
pub fn abs_b(i: i16)-> i16{
    (i ^ (i >> 15)) - (i >> 15)
}

/// The function `get_lsb` in Rust returns the least significant bit of an integer.
/// 
/// Arguments:
/// 
/// * `i`: The parameter `i` in the function `get_lsb` is an `i32` integer.
/// 
/// Returns:
/// 
/// The function `get_lsb` returns the least significant bit (LSB) of the input `i` as an integer.
pub fn get_lsb(i: i32)-> i32{
    (i)&(-i)
}

/// The function `get_without_lsb` in Rust returns the input integer with its least significant bit
/// (LSB) set to zero.
/// 
/// Arguments:
/// 
/// * `i`: The function `get_without_lsb` takes an `i32` integer as input and returns the same integer
/// with the least significant bit (LSB) cleared.
/// 
/// Returns:
/// 
/// The function `get_without_lsb` returns the input `i` with the least significant bit (LSB) set to 0.
pub fn get_without_lsb(i: i32)-> i32{
    i&(i-1)
}

/// The function `get_indi_bits` takes an integer as input and returns a vector containing individual
/// bits of the input integer.
/// 
/// Arguments:
/// 
/// * `i`: The function `get_indi_bits` takes an input parameter `i` of type `i16`, which is a 16-bit
/// signed integer. The function then extracts individual bits from the input integer `i` and returns
/// them as a vector of `i16` values.
/// 
/// Returns:
/// 
/// A vector of individual bits of the input number `i` is being returned.
pub fn get_indi_bits(mut i: i16)-> Vec<i16>{
    let mut returner: Vec<i16>= Vec::new();
    while i!=0 {
        returner.push(get_lsb(i as i32) as i16);
        i= get_without_lsb(i as i32) as i16;
    }
    returner
}
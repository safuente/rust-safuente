/// Function: add_five
/// 
/// # Arguments (num: u32)
/// 
/// # Example
/// '''
/// let x = 5;
/// let y = add_five(x);
/// '''
/// 
/**
 * 
 */
pub fn add_five(num: u32) -> u32 {
    num + 5 // adds five
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x: u32 = 100;
        let y = add_five(x);

        // print x and y
        println!("x and y are from test {} {}", x, y);
        assert_eq!(y, 105);
    }
}

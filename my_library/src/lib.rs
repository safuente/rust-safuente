/// Adds two 64-bit unsigned integers and returns the result.
///
/// # Arguments
///
/// * `left` - The first 64-bit unsigned integer.
/// * `right` - The second 64-bit unsigned integer.
///
/// # Returns
///
/// A 64-bit unsigned integer representing the sum of `left` and `right`.
///
/// # Example
///
/// ```
/// # use my_library::add;
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// This function is used to perform simple addition of two unsigned integer values.
///
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

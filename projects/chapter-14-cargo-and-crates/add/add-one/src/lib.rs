/// Function that adds 1 to the number provided.
///
/// # Examples
/// ```
/// let num = 116;
/// let answer = add_one::add_one(num);
/// assert_eq!(117, answer)
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
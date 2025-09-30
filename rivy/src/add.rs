pub fn add(left: u64, right: u64) -> u64 {
    let e = left + right;             // compute the sum
    println!("value: {}", e);         // print it
    e                                  // return the value
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
use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn open_file(input: &str) -> String {
    return fs::read_to_string(input).expect("Unable to read file");
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

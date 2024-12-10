// Modul utils
pub fn format_number(num: i32) -> String {
    format!("The result is: {}", num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(10), "The result is: 10");
    }
}
pub mod math;
pub mod utils;

// Fungsi utama yang memanfaatkan modul math dan utils
pub fn process_numbers(a: i32, b: i32) -> String {
    let sum = math::add(a, b);
    let formatted = utils::format_number(sum);
    formatted
}

#[cfg(test)]
mod tests {
    use super::*; // Mengimpor semua dari modul lib.rs

    #[test]
    fn test_process_numbers_unit() {
        // Unit test untuk fungsi process_numbers
        let result = process_numbers(2, 3);
        assert_eq!(result, "The result is: 5");
    }
}   
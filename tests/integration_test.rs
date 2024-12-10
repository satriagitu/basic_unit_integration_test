use basic_unit_integration_test; // Mengimpor crate utama

#[test]
fn test_process_numbers_integration() {
    // Menguji interaksi antara math::add dan utils::format_number
    let result = basic_unit_integration_test::process_numbers(3, 4);
    assert_eq!(result, "The result is: 7");
}

#[test]
fn test_math_and_utils_integration() {
    // Menguji beberapa fungsi dari modul math dan utils secara bersamaan
    let sum = basic_unit_integration_test::math::add(5, 10);
    let formatted = basic_unit_integration_test::utils::format_number(sum);
    assert_eq!(formatted, "The result is: 15");

    let product = basic_unit_integration_test::math::multiply(3, 5);
    let formatted_product = basic_unit_integration_test::utils::format_number(product);
    assert_eq!(formatted_product, "The result is: 15");
}

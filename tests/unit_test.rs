use basic_unit_integration_test::math;
use basic_unit_integration_test::utils;

#[test]
fn test_add_unit() {
    // Unit test untuk fungsi math::add
    assert_eq!(math::add(2, 3), 5);
    assert_eq!(math::add(-1, -1), -2);
}

#[test]
fn test_multiply_unit() {
    // Unit test untuk fungsi math::multiply
    assert_eq!(math::multiply(2, 3), 6);
    assert_eq!(math::multiply(-1, 5), -5);
}

#[test]
fn test_format_number_unit() {
    // Unit test untuk fungsi utils::format_number
    assert_eq!(utils::format_number(10), "The result is: 10");
    assert_eq!(utils::format_number(-5), "The result is: -5");
}

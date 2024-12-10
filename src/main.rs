use basic_unit_integration_test::process_numbers; // Import fungsi dari lib.rs
fn main() {
    let a = 3;
    let b = 4;

    // Memanggil fungsi process_numbers dari lib.rs
    let result = process_numbers(a, b);

    println!("{}", result); // Output: "The result is: 7"
}
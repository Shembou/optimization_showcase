unsafe extern "C" {
    fn soap_add(a: i32, b: i32) -> i32;
    fn soap_subtract(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        let sum = soap_add(5, 3);
        println!("5 + 3 = {}", sum);

        let diff = soap_subtract(10, 4);
        println!("10 - 4 = {}", diff);
    }
}

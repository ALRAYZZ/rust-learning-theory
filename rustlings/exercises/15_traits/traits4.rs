trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
// We need to use trait bounds to ensure that both parameters implement the Licensed trait.
// And to solve the unknown size issue, we can use generics,
// so that the compiler knows the exact types at compile time.
// Compiler generates concrete implementations for each type used. And when the different
// types are passed, Rust can see their sizes and handle them correctly.
fn compare_license_types<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}

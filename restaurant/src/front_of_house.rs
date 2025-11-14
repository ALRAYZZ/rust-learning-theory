pub mod hosting;


mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

}

pub fn open_house() {
    // Example using the hosting module
    hosting::add_to_waitlist();
}

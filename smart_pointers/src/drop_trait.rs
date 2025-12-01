struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn run() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    // This is the std method to explicitly call drop on a value and avoid the double free error
    drop(c);
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
}
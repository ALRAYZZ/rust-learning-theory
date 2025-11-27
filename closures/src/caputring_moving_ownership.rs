fn run() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // implicitly does: let list_ref = &list;
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);

    // call the closure that only borrows list and we can still use list afterward
    // closures implement the Fn, FnMut, or FnOnce traits based on how they capture variables
    // so we can call them as a function
    only_borrows();
    println!("After calling closure: {:?}", list);

    // If closure took ownership of list with move keyword
    let take_ownership = move || println!("From closure taking ownership: {:?}", list);


    let mut list2 = vec![4, 5, 6];
    println!("Before defining mutable closure: {:?}", list2);

    // this time the closure mutably borrows list2 and can modify it
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After calling mutable closure: {:?}", list2);
}
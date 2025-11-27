// Iterators are lazy. They do not perform any computation until they are consumed.
// Its so we avoid doing the common C for loop over and over again.
// like for (i = 0; i < v1.len(); i++) type syntax


fn run() {
    let v1 = vec![1, 2, 3];

    // This creates an iterator over references to the elements in v1
    // but is not doing anything with it yet
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got val: {val}");
    }

}

// Methods that consume the iterator like sum, collect, map, filter, etc
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // In order to be able to use the sum method, we need convert the v1 into an iterator
    // and then consume the iterator "Sums the elements of an iterator"
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Methods that produce other iterators
fn iterator_map() {
    let v1 = vec![1, 2, 3];

    // The map method takes a closure that defines how to transform each element
    // It returns a new iterator that produces the transformed elements (in this case, each element plus 1)
    // Also we need to add .collect() at the end to consume the iterator
    // and collect the results into a new collection
    // else we would get a warning that the iterator is not used
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // We can chain multiple iterator adaptors together
    // For example, we can filter the elements to only include those greater than 2
    // and then map them to multiply by 2
    // but, we always need to call a consuming adaptor at the end to get the results
    assert_eq!(v2, vec![2, 3, 4]);
}
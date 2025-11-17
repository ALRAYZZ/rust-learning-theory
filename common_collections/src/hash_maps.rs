fn run() {
    // Creating hash maps
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Accessing values in a hash map using get method which returns an Option and then calling
    // copied to get an owned value instead of a reference and unwrap_or to provide a default value
    // if the key does not exist
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterating over key-value pairs in a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash map and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // Ownership of field_name and field_value is moved to the hash map
    // field_name and field_value are no longer valid here as their ownership has been moved to
    // the hash map this is NOT A HASH MAP FEATURE BUT THE DATA TYPE WE ARE USING

    // So integers have the Copy trait so they are copied into the hash map so we can still use them
    // Strings do not have the Copy trait so their ownership is moved into the hash map

    // Overwriting a value in a hash map
    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25); // This will overwrite the previous

    println!("{:?}", scores2); // This will print {"Blue": 25}

    // Adding a Key and a Value Only If the Key Isn'T Present called ENTRY API
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    // or_insert method to insert a value if the key does not exist
    scores3.entry(String::from("Yellow")).or_insert(50);

    // This will not insert a new value because "Blue" key already exists
    scores3.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores3); // This will print {"Blue": 10, "Yellow": 50}


    // Updating a Value based on the old value
    // Like counting the number of occurrences of words in a text
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Split the text into words and iterate over them
    // For each word, we use the entry API to get the count and increment it
    for word in text.split_whitespace() {

        let count = map.entry(word).or_insert(0);
        *count += 1;
    }


}
mod tuple_struct;
mod rectangles;
mod methods;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    // IMMUTABLE INSTANCE
    // Rust doesn’t allow us to mark only certain fields as mutable
    let user1 = User {
        active: true,
        username: String::from("usernametest"),
        email: String::from("email"),
        sign_in_count: 1,
    };
    // Not allowed, because user1 is immutable by default
    //user1.email = String::from("anotheremail");

    // MUTABLE INSTANCE
    let mut user2 = User {
        active: true,
        username: String::from("usernametest2"), // struct owns the String
        email: String::from("email2"),
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail2");

    // Creating instances from another instance
    let user3 = User {
        active: user2.active, // Implements Copy trait so this is fine
        username: user2.username, // Moves ownership from user2 to user3
        email: String::from("email3"), // New value
        sign_in_count: user2.sign_in_count, // Implements Copy trait so this is fine
    };

    // USING STRUCT UPDATE SYNTAX
    let user4 = User {
        email: String::from("email4"),
        ..user3  // fill in the rest from user3
    };

    // Problem, user2 got moved to user3.username so user2.username is no longer valid
    //println!("{}", {user2.username});


    tuple_struct::run();
    rectangles::run();
    methods::run();
}

// THIS CAN  BE SIMPLIFIED
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// SIMPLIFIED VERSION - because the parameter names match the field names we can do this
fn build_user_simplified(email: String, username: String) -> User {
    User {
        active: true,
        username,  // matching field name
        email, // matching field name
        sign_in_count: 1,
    }
}
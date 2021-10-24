pub fn run() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // error!

    let var_a = String::from("Howdy");
    let var_b = var_a;

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn dangle() -> String {
//     let s = String::from("hello");

//     return s;
// }

// fn change(to: &mut String) {
//     to.push_str("hello");
// }

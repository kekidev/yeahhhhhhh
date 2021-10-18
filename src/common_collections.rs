use std::collections::HashMap;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn run() {
    let b = 10;
    let v = vec![1,2,3,4,5];
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue"))
    ];

    let hello = "Здравствуйте";
    //println!("{:?}", hello.as_bytes());

    /*
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    } */

    let mut scores = HashMap::new();
    scores.insert(String::from("Key"), 1999);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_,_> = 
        teams.into_iter().zip(initial_scores.iter()).collect();

    // HASHMAP AND OWNERSHIP
    let field_name = String::from("Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    /* At here field_name and field_value is invalid
     * because Hashmap took ownership from it
     * */
    map.insert(field_name, field_value); 

    // println!("{}", field_name);  ERROR

    for (k, v) in &map {
        // for loop
    }

    // OVERWRITING
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // INSERTING A VALUE IF THE KEY HAS NO VALUE
    scores.entry(String::from("Yellow")).or_insert(50);

    let a1 = "hello world fucked up world";

    let mut m1 = HashMap::new();

    for word in a1.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}

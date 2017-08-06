use std::collections::HashMap;

pub fn seq_test() {
    let rand_arr = [3, 5, 1, 4, 80];
    println!("rand_arr[2] is {}", rand_arr[2]);
    println!("rand_arr length is {}", rand_arr.len());
    println!("rand_arr[1..3] slice is {:?}", &rand_arr[1..3]);

    println!("{}\n", "a".repeat(10));
    // VECTORS
    let mut vec1 = vec![1, 6, 8, 5, 12, 11, 76, 90];
    println!("vec1[2] is {}", vec1[2]);

    // for v in &vec1 {
    //     println!("Vec member -> {}", v);
    // }

    vec1.push(65);
    println!("vec1 after push -> {:?}", vec1);

    vec1.pop();
    println!("vec1 after pop -> {:?}", vec1);

    let vec_el1:&i32 = &vec1[2];
    let vec_el2: Option<&i32> = vec1.get(3);
    println!("get elem of vectors: '2' -> {}, '3' -> {:?}", vec_el1, vec_el2);
    println!("{}\n", "v".repeat(10));
    // TUPLES
    let pr_tup = ("Vitalii", "Igor", "Leo");
    let pr_tup2: (&str, &str, i8, i32) = ("Vitalii", "Drevenchuk", 29, 1988);
    println!("Your name is {0} and age is {1}", pr_tup2.0, pr_tup2.2);
    println!("tup1 -> {:?}", pr_tup);
    println!("tup2 -> {:?}\n", pr_tup2);

    // HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("HashMap1 -> {:?}\n", scores);

    // Another way to create HashMap
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("HashMap2 -> {:?}\n", scores2);
}

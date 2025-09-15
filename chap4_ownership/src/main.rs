fn main() {
    let mut s = String::from("hello");

    change(&mut s);

//    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s;

//    println!("{}, {}", r1, r2);
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
    let r2 = &mut s;

    let reference_to_nothing = dangle();

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// } // ここで、sはスコープを抜け、ドロップされる。そのメモリは消去される。

fn dangle() -> String {
    let s = String::from("hello");

    s
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消去される。

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

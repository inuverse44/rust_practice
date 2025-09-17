// struct User {
//     username: String, 
//     email: String, 
//     sign_in_count: u64, 
//     active: bool,
// }

// // instance
// // let user1 = User {
// //     email: String::from("someone@example.com"), 
// //     username: String::from("someusername123"), 
// //     active: true, 
// //     sign_in_count: 1, 
// // };

// // インスタンス全体が可変である。一部だけ可変、それ以外は不変のようにできない
// let mut user1 = User {
//     email: String::from("someone@example.com"), 
//     username: String::from("someusername123"), 
//     active: true, 
//     sign_in_count: 1, 
// };

// let user2 = User {
//     email: String::from("another@example.com"), 
//     username: String::from("anotherusername123"), 
//     // active: user1.active, 
//     // sign_in_count: user1.sign_in_count, 
//     ..user1 // 略記。明示的にセットされていない箇所をuser1で補完する
// };

// use1.email = String::from("anotheremail@example.com");
// fn build_user(email: String, username: String) -> User {
//     User {
//         // email: email, 
//         // username: username, 
//         email, // 略記
//         username, // 略記
//         active: true, 
//         sign_in_count: 1,
//     }
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// // ColorとPointは異なる型になる

// let black = Color(0, 0, 0);
// let origin = Point(0, 0, 0);

#[derive(Debug)] // 構造体をprintlnでdebugモードで出力するため
struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {
    // 構造体の情報を読み込みたいだけだから、所有権はいらない。参照で良い
    fn area(&self) -> u32 { // メソッドには必ずインスタンスを表すselfという引数がつく
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // tapleでリファクタリング
    let rect1 = Rectangle { width: 30, height: 50};
    let rect1 = Rectangle { width: 10, height: 40};
    let rect1 = Rectangle { width: 60, height: 45};

    println!(
        //"The area of the rectangle is {:#?} square pixels.", 
        "The area of the rectangle is {} square pixels.", 
        // area(width1, height1)
        // area(rect1)
        // area(&rect1)
        // rect1
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2))
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3))
}

// // 1. fn area(width: u32, height: u32) -> u32{
// // 2. fn area(dimensions: (u32, u32)) -> u32 {
// fn area(rectangle: &Rectangle) -> u32 { // 引数が1つになってうれぴー
//     // 1. width * height
//     // 2. dimensions.0 * dimensions.1 // 0が幅で1が高さを覚えておかないといけない
//     rectangle.width * rectangle.height // 意味付けも一目でわかる
// }

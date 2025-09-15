use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // --snip--
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutはmutable
        // String型のインスタンスを返す
        // ::newはnewがStringの関連関数であることを表す

        io::stdin() // std::io::Stdin型のインスタンスを返す
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // Stringインスタンスのtrimメソッドは文字列の前後の空白をすべて削除する
        // read_lineの処理が終了するためにはEnter --> \nが追加される。これをtrimで削除する。
        // u32 --> unsigned 32 bit 小さな正の数に向いている
        // 失敗する可能性があるparse --> Result型を返す
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,         // ResultsはOk, Err列挙子を持っている
            Err(_) => continue,     // underscoreはすべての値を受けつケル
        }; // Rustではshadowができる
        //    .expect("Please type a number!"); // この機能はある型から別の型に変換するときよく使う


        println!("You guessed: {}", guess);

        // 正解がでたらゲームを終了する
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}

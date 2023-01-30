fn main() {
    println!("the first letter of the english alphabet is {} and the last letter is {}.", 'A', 'Z');
    let a_number;
    let a_word = "Ten";
    a_number = 10;
    println!("This number is {}", a_number);
    println!("the word is {}", a_word);

    //shadowing
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("the number is {}", shadow_num)

}


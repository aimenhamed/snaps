use std::io;

fn main() {
    println!("Enter ! when finished.");

    let mut word = String::new();

    loop {
        println!("Snaps or sentence?");
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed");

        let res = match response.trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };
        if res == "!" {
            break;
        }
        word.push_str(&handle_selection(res));
    }

    println!("The word is: {}", word);
}

fn handle_selection(str: String) -> String {
    // match str.chars().nth(0).unwrap().is_numeric() {
        // true => handle_snaps(),
        // false => handle_sentence(),
    // }
    let word: String = match str.as_str() {
        "sentence" => handle_sentence(),
        "snaps" => handle_snaps(),
        _ => String::new(),
    };
    return word;
}

fn handle_sentence() -> String {
    println!("Please enter the sentence:");
    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed");

    let res: String = match sentence.trim().parse() {
        Ok(str) => str,
        Err(_) => return String::new(),
    };

    let letter = res.as_str().chars().nth(0).unwrap();

    return letter.to_string();
}

fn handle_snaps() -> String {
    println!("Please enter how many snaps:");
    let vowels = ["a", "e", "i", "o", "u"];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed");

    let i: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => return String::new(),
    };

    return vowels[i-1].to_string();

}

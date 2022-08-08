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
    match str.chars().nth(0).unwrap().is_numeric() {
        true => handle_snaps(str),
        false => handle_sentence(str),
    }
}

fn handle_sentence(str: String) -> String {
    let letter = str.chars().nth(0).unwrap();
    return letter.to_string();
}

fn handle_snaps(str: String) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let i: usize = match str.trim().parse() {
        Ok(num) => num,
        Err(_) => return String::new(),
    };

    return vowels[i-1].to_string();

}

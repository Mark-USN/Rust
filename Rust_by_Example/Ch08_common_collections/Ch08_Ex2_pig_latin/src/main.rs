fn main() {
    let gettysburg_address = String::from("Four score and seven years ago our fathers brought forth on this continent a new nation conceived in Liberty and dedicated to the proposition that all men are created equal");
    let empty = String::new();
    let space = String::from(" ");
    let consonants = String::from("Gbhtwtr");

    println!("{gettysburg_address}");
    println!("{}", convert_to_pig(& gettysburg_address));

    println!("{empty}");
    println!("{}", convert_to_pig(& empty));

    println!("{space}");
    println!("{}", convert_to_pig(& space));

    println!("{consonants}");
    println!("{}", convert_to_pig(& consonants));
}


fn convert_to_pig(s: &str) -> String {
    let mut pig = String::new();
    let words = s.split_whitespace();

    for w in words {
        let (first, rest) = w.split_at(1);
        // if the first letter is a vowel, just add "way" to the end.
        if is_vowel(&first.chars().next().unwrap()) {
            pig.push_str(first);
            pig.push_str(rest);
            pig.push_str("way");
        } else {
            // The first letter is a consonant so remove all consonants upto
            // the first vowel, place them at the end of the word and add "ay"

            // Need to modify 'first' and 'rest' so convert them to mutable strings.
            let mut first = first.to_string();
            let mut rest = rest.to_string();
            // Can't both use rest in defining for loop and modify it within 
            // the loop, so clone it 
            let rest_chars = rest.clone();

            for c in rest_chars.chars() {
                if !is_vowel(&c) {
                    first.push(c);
                    rest.remove(0);
                } else {
                    break;
                }           
            }
            pig.push_str(&rest);
            pig.push_str(&first);
            pig.push_str("ay");
        }
        pig.push_str(" ");
    }
    pig

}

fn is_vowel(c: & char) -> bool {
    let vowels = String::from("aAeEiIoOuU");
    if c.is_ascii_alphabetic() {
        for v in vowels.chars() {
            if v == *c {
                return true;
            }
        }
    }

    false
}
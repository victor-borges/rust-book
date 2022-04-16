fn main() {
    let original_string = "Ten. Million. Questions. Let's celebrate all we've done together. Whose pig is this? It's his'.";
    let mut new_words = Vec::new();

    let mut words = Vec::new();
    let mut last = 0;
    for (index, matched) in original_string.match_indices(is_word_separator) {
        if last != index {

            words.push(&original_string[last..index]);
        }
        words.push(matched);
        last = index + matched.len();
    }
    if last < original_string.len() {
        words.push(&original_string[last..]);
    }

    'words: for word in words {
        let mut chars = word.chars().peekable();

        if let Some(c) = chars.peek() {
            if !c.is_alphabetic() {
                new_words.push(word.to_string());
                continue
            }
        }

        let is_uppercase = match chars.peek() {
            Some(c) => c.is_uppercase(),
            None => continue
        };

        while let Some(c) = chars.next() {
            if c.is_vowel() {
                new_words.push(format!("{word}yay"));
                continue 'words;
            } else {
                let mut new_word = c.to_lowercase().to_string();
                while let Some(next) = chars.next() {
                    if next.is_consonant() {
                        new_word.push_str(&next.to_lowercase().to_string());
                    } else {
                        let vowel = if is_uppercase {
                            next.to_uppercase().to_string()
                        } else {
                            next.to_string()
                        };

                        let remainder = chars.to_owned().collect::<String>();
                        new_words.push(format!("{vowel}{remainder}{new_word}ay"));
                        continue 'words;
                    }
                }
            }
        }
    }

    let new_string = new_words.concat();
    println!("{new_string}");
}

fn is_word_separator(c: char) -> bool {
    c.is_whitespace() || (c.is_ascii_punctuation() && c != '\'') || c.is_control()
}

trait RomanceChar {
    fn is_vowel(&self) -> bool;
    fn is_consonant(&self) -> bool;
}

impl RomanceChar for char {
    fn is_vowel(&self) -> bool {
        matches!(self, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }

    fn is_consonant(&self) -> bool {
        self.is_ascii_alphabetic() && !self.is_vowel()
    }
}

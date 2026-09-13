//Copyright (C) 2026  Senne98
//Lisence: https://github.com/Senne98/Cycle/blob/main/LICENSE

pub trait CharVecToString {
    fn to_string_if_valid_f64(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

impl CharVecToString for Vec<char> {
    fn to_string(&self) -> String {
        self.into_iter().collect()
    }

    fn to_string_if_valid_f64(&self) -> Option<String> {
        let text: String = self.to_string();
        if text.parse::<f64>().is_err() {
            return None;
        }

        return Some(text);
    }
}

pub trait CharIsLetter {
    fn is_letter(&self) -> bool;
}

impl CharIsLetter for char {
    fn is_letter(&self) -> bool {
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'].contains(self)
    }
}

pub trait RemoveSpaces {
    fn remove_whitespaces(&self) -> String;
}

impl RemoveSpaces for String {
    fn remove_whitespaces(&self) -> String {
        self.replace(" ", "")
    }
}

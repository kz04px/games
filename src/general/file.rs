const LETTERS: usize = 52;

#[must_use]
const fn letter(value: usize) -> u8 {
    if value < 26 {
        b'a' + value as u8
    } else {
        b'A' + (value - 26) as u8
    }
}

#[must_use]
const fn value(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some(c as usize - 'a' as usize),
        'A'..='Z' => Some(26 + c as usize - 'A' as usize),
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct File<const WIDTH: usize>(pub u8);

impl<const WIDTH: usize> File<WIDTH> {
    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        if word.is_empty() {
            return Err("Wrong string length");
        }

        let mut index: usize = 0;
        for c in word.chars() {
            let value = value(c).ok_or("char out of range")?;
            index = index
                .checked_mul(LETTERS)
                .and_then(|shifted| shifted.checked_add(value + 1))
                .ok_or("char out of range")?;
        }

        let index = index - 1;
        if index >= WIDTH || index > u8::MAX as usize {
            Err("char out of range")
        } else {
            Ok(Self(index as u8))
        }
    }

    pub fn from_char(c: char) -> Result<Self, &'static str> {
        Self::from_string(c.encode_utf8(&mut [0; 4]))
    }

    #[must_use]
    pub const fn get_index(&self) -> usize {
        self.0 as usize
    }
}

impl<Any, const WIDTH: usize> std::ops::Index<File<WIDTH>> for [Any; WIDTH] {
    type Output = Any;

    fn index(&self, f: File<WIDTH>) -> &Self::Output {
        &self[f.get_index()]
    }
}

impl<Any, const WIDTH: usize> std::ops::IndexMut<File<WIDTH>> for [Any; WIDTH] {
    fn index_mut(&mut self, f: File<WIDTH>) -> &mut Self::Output {
        &mut self[f.get_index()]
    }
}

impl<const WIDTH: usize> std::fmt::Display for File<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut letters = [0u8; 4];
        let mut first = letters.len();
        let mut remaining = self.get_index() + 1;

        while remaining > 0 {
            remaining -= 1;
            first -= 1;
            letters[first] = letter(remaining % LETTERS);
            remaining /= LETTERS;
        }

        for letter in &letters[first..] {
            write!(f, "{}", *letter as char)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::File;

    #[test]
    fn indexing() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(array[File::<8>(0)], 1);
        assert_eq!(array[File::<8>(1)], 2);
        assert_eq!(array[File::<8>(2)], 3);
        assert_eq!(array[File::<8>(3)], 4);
        assert_eq!(array[File::<8>(4)], 5);
        assert_eq!(array[File::<8>(5)], 6);
        assert_eq!(array[File::<8>(6)], 7);
        assert_eq!(array[File::<8>(7)], 8);
    }

    #[test]
    fn from_char() {
        assert!(File::<8>::from_char('a').is_ok());
        assert!(File::<8>::from_char('b').is_ok());
        assert!(File::<8>::from_char('h').is_ok());
        assert!(File::<8>::from_char('i').is_err());
        assert_eq!(File::<8>::from_char('a'), Ok(File(0)));
        assert_eq!(File::<8>::from_char('b'), Ok(File(1)));
        assert_eq!(File::<8>::from_char('h'), Ok(File(7)));
        assert_eq!(File::<8>::from_char('i'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('`'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('#'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('0'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('9'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('!'), Err("char out of range"));
    }

    #[test]
    fn to_string() {
        for c in 'a'..='h' {
            if let Ok(f) = File::<8>::from_char(c) {
                assert_eq!(f.to_string(), c.to_string());
            } else {
                panic!("Uh oh")
            }
        }
        for c in 'i'..='z' {
            assert!(File::<8>::from_char(c).is_err());
        }
    }

    #[test]
    fn index() {
        assert_eq!(File::<8>::from_string("a").unwrap().get_index(), 0);
        assert_eq!(File::<8>::from_string("b").unwrap().get_index(), 1);
        assert_eq!(File::<8>::from_string("c").unwrap().get_index(), 2);
    }

    #[test]
    fn files_past_z() {
        let tests = [
            (0, "a"),
            (1, "b"),
            (25, "z"),
            (26, "A"),
            (27, "B"),
            (51, "Z"),
            (52, "aa"),
            (53, "ab"),
            (63, "al"),
            (77, "az"),
            (78, "aA"),
            (103, "aZ"),
            (104, "ba"),
        ];

        for (index, text) in tests {
            assert_eq!(File::<200>::from_string(text), Ok(File(index)), "{text}");
            assert_eq!(File::<200>(index).to_string(), text, "{index}");
        }
    }

    #[test]
    fn every_file_of_a_64_wide_board_has_its_own_name() {
        let mut seen = std::collections::HashSet::new();

        for index in 0..64 {
            let text = File::<64>(index).to_string();
            assert_eq!(File::<64>::from_string(&text), Ok(File(index)), "{text}");
            assert_eq!(text.len(), if index < 52 { 1 } else { 2 }, "{text}");
            assert!(seen.insert(text.clone()), "two files called {text}");
        }

        assert_eq!(seen.len(), 64);
    }

    #[test]
    fn out_of_range() {
        assert!(File::<64>::from_string("al").is_ok());
        assert_eq!(File::<64>::from_string("am"), Err("char out of range"));
        assert_eq!(File::<8>::from_string("A"), Err("char out of range"));
        assert_eq!(File::<8>::from_string("aa"), Err("char out of range"));
        assert_eq!(
            File::<64>::from_string("abcdefghijklmnop"),
            Err("char out of range")
        );

        for word in ["", "a1", "1", "-", "é", "aé", "{", "[", " a"] {
            assert!(File::<64>::from_string(word).is_err(), "{word}");
        }
    }

    #[test]
    fn upper_case_is_out_of_range_on_a_narrow_board() {
        for c in 'A'..='Z' {
            assert!(File::<8>::from_char(c).is_err(), "{c}");
            assert!(File::<15>::from_char(c).is_err(), "{c}");
            assert!(File::<26>::from_char(c).is_err(), "{c}");
        }
    }
}

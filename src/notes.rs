use regex::Regex;
use std::fmt;
use structopt::lazy_static::lazy_static;

const NOTE_RE: &str = r"([A-G])([#b]?)(-?\d{0,2})";

#[derive(Debug, PartialEq)]
pub enum Accidental {
    Sharp,
    Flat,
}

#[derive(Debug, PartialEq)]
pub enum NoteLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
pub struct Note {
    letter: NoteLetter,
    acc: Option<Accidental>,
    n: Option<i8>,
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sharp => write!(f, "{}", self.to_char()),
            Flat => write!(f, "{}", self.to_char()),
        }
    }
}

impl Accidental {
    fn to_char(&self) -> char {
        use Accidental::*;
        match self {
            Sharp => '#',
            Flat => 'b',
        }
    }

    fn from_char(s: char) -> Accidental {
        match s {
            '#' => Accidental::Sharp,
            'b' => Accidental::Flat,
            _ => panic!("Invalid Accidental"),
        }
    }
}

impl fmt::Display for NoteLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl NoteLetter {
    fn to_char(&self) -> char {
        format!("{:?}", self).chars().nth(0).unwrap()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Note {
    pub fn new(letter: NoteLetter) -> Note {
        Note {
            letter,
            acc: None,
            n: None,
        }
    }

    pub fn to_string(&self) -> String {
        let Note { letter, acc, n } = self;
        use NoteLetter::*;

        let (letter_sign, acc_sign) = match (letter, acc) {
            (letter, Some(acc)) if *acc == Accidental::Sharp && letter == &B => {
                ("C".to_string(), "".to_string())
            }
            (letter, Some(acc)) if *acc == Accidental::Flat && letter == &C => {
                ("B".to_string(), "".to_string())
            }
            (letter, Some(acc)) if *acc == Accidental::Sharp && letter == &E => {
                ("F".to_string(), "".to_string())
            }
            (letter, Some(acc)) if *acc == Accidental::Flat && letter == &F => {
                ("E".to_string(), "".to_string())
            }
            (letter, Some(acc)) => (letter.to_string(), acc.to_string()),
            (letter, _) => (letter.to_string(), "".to_string()),
        };

        let n_sign = match n {
            Some(n) => n.to_string(),
            _ => "".to_string(),
        };

        format!("{}{}{}", letter_sign, acc_sign, n_sign)
    }

    pub fn from_str(s: &str) -> Self {
        // lazy_static! {
        //     static ref RE: Regex = Regex::new("...").unwrap();
        // }

        Note::new(NoteLetter::A)
    }
}

#[cfg(test)]
mod tests {
    use super::Accidental::*;
    use super::Note;
    use super::NoteLetter::*;
    use super::NOTE_RE;
    use regex::Captures;
    use regex::Regex;

    fn assert_cap(cap: Captures, [letter, acc, n]: [&str; 3]) {
        assert_eq!(cap.get(1).unwrap().as_str(), letter);
        assert_eq!(cap.get(2).unwrap().as_str(), acc);
        assert_eq!(cap.get(3).unwrap().as_str(), n);
    }

    #[test]
    fn test_from_str() {
        let re = Regex::new(NOTE_RE).unwrap();

        let cap = re.captures("A#10").unwrap();
        assert_cap(cap, ["A", "#", "10"]);

        let cap = re.captures("A#-10").unwrap();
        assert_cap(cap, ["A", "#", "-10"]);

        let cap = re.captures("Ab-4").unwrap();
        assert_cap(cap, ["A", "b", "-4"]);

        let cap = re.captures("Gb").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "G");
        assert_eq!(cap.get(2).unwrap().as_str(), "b");
        assert!(cap.get(3).is_none());

        let cap = re.captures("G").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "G");
        assert!(cap.get(2).is_none());

        let cap = re.captures("G4").unwrap();
        assert_eq!(cap.get(1).unwrap().as_str(), "G");
        assert_eq!(cap.get(2).unwrap().as_str(), "4");

        let cap = re.captures("H#").unwrap();
        assert!(cap.get(0).is_none());
    }

    #[test]
    fn test_note_to_string() {
        // simple single note from new
        assert_eq!(Note::new(A).to_string(), "A");
        // Basic with acc and num
        assert_eq!(
            Note {
                letter: A,
                acc: Some(Sharp),
                n: Some(2)
            }
            .to_string(),
            "A#2"
        );
        // basic acc with negative num
        assert_eq!(
            Note {
                letter: C,
                acc: Some(Sharp),
                n: Some(-1)
            }
            .to_string(),
            "C#-1"
        );
        // Basic with flat without num
        assert_eq!(
            Note {
                letter: A,
                acc: Some(Flat),
                n: None
            }
            .to_string(),
            "Ab"
        );
        // skipping acc
        assert_eq!(
            Note {
                letter: B,
                acc: None,
                n: Some(2)
            }
            .to_string(),
            "B2"
        );
        // testing half step to f (sharp)
        assert_eq!(
            Note {
                letter: E,
                acc: Some(Sharp),
                n: None
            }
            .to_string(),
            "F"
        );
        // testing half step to E (flat)
        assert_eq!(
            Note {
                letter: F,
                acc: Some(Flat),
                n: Some(2)
            }
            .to_string(),
            "E2"
        );
        // testing half step to B (sharp)
        assert_eq!(
            Note {
                letter: B,
                acc: Some(Sharp),
                n: None
            }
            .to_string(),
            "C"
        );
        // testing half step to C (flat) and negative num
        assert_eq!(
            Note {
                letter: C,
                acc: Some(Flat),
                n: Some(-1)
            }
            .to_string(),
            "B-1"
        );
    }
}

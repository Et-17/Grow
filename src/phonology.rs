use std::fmt::Display;

pub struct Word {
    pub syllables: Vec<Syllable>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for syllable in &self.syllables {
            write!(f, "{}", syllable)?;
        }
        return Ok(());
    }
}

pub struct Syllable {
    pub phonemes: Vec<Phoneme>,
    pub stressed: bool
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stressed {
            write!(f, "ˈ")?;
        }
        for phoneme in &self.phonemes {
            write!(f, "{}", phoneme)?;
        }
        Ok(())
    }
}

pub struct Phoneme {
    pub value: String
}

impl Display for Phoneme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
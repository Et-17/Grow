mod phonology;

use phonology::*;

fn main() {
    let internal_transcription = Word {
        syllables: vec![
            Syllable {
                stressed: false,
                phonemes: vec![
                    Phoneme {value: "l".to_string()},
                    Phoneme {value: "ɪ".to_string()},
                    Phoneme {value: "ŋ".to_string()}
                ]
            },
            Syllable {
                stressed: true,
                phonemes: vec![
                    Phoneme {value: "ɡ".to_string()},
                    Phoneme {value: "w".to_string()},
                    Phoneme {value: "ɪ".to_string()},
                    Phoneme {value: "s".to_string()},
                    Phoneme {value: "t".to_string()}
                ]
            },
            Syllable {
                stressed: false,
                phonemes: vec![
                    Phoneme {value: "ɪ".to_string()},
                    Phoneme {value: "k".to_string()},
                    Phoneme {value: "s".to_string()}
                ]
            }
        ]
    };
    println!("Intended: lɪŋˈɡwɪstɪks");
    println!("Internal: {}", internal_transcription);
}

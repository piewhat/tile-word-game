use bevy::{platform::collections::HashSet, prelude::*};

// region:    --- Types

#[derive(Resource)]
pub struct Dictionary {
    pub words: HashSet<String>,
}

#[derive(Resource, Default)]
pub struct GameProgress {
    pub score: u32,
}

// endregion: --- Types

impl FromWorld for Dictionary {
    fn from_world(_world: &mut World) -> Self {
        let mut words = HashSet::default();

        // Try to load from assets file
        if let Ok(content) = std::fs::read_to_string("assets/wordlist-20210729.txt") {
            for word in content.split_whitespace() {
                let word = word.trim_matches(|c| c == '"' || c == ',').to_uppercase();
                if !word.is_empty() {
                    words.insert(word);
                }
            }
        }

        // Fallback starter dictionary (uppercase matches tile characters)
        if words.is_empty() {
            let word_list = [
                "BEVY", "RUST", "GAME", "CODE", "TILE", "SLOT", "WORD", "PLAY", "TYPE", "SEND",
                "FAST", "COOL", "SHARP", "BLAST", "PIXEL", "INPUT", "EVENT", "GRID", "DATA",
                "NODE",
            ];

            for word in word_list {
                words.insert(word.to_string());
            }
        }

        Self { words }
    }
}

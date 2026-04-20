use bevy::prelude::*;

#[derive(Event)]
pub struct SubmitWord {
    pub word: String,
}

#[derive(Event)]
pub struct WordAccepted {
    pub word: String,
    pub score: u32,
}

#[derive(Event)]
pub struct WordRejected {
    pub word: String,
}

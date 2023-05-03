use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

// impl Score {
//     fn new(value: u32) -> Self {
//         Self { value: value }
//     }
// }

impl Default for Score {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}
use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

use super::resources::*;
use crate::events::GameOver;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        screen_print!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        screen_print!("High Scores: {:?}", high_scores);
    }
}

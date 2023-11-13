// Import common module
mod common;

use player_rank_cli::player_rank_lib::*;

#[test]
fn get_first_question_2p(){
    // Create a player rank
    let players = common::test_players(26);
    let mut questions = Questions::new();
    let mut player_rank = PlayerRank::new(&players, &mut questions);

    // Get first question
    let (question, status) = player_rank.get_next_question();

    // Validate question/status existence
    assert!(question.is_some());
    assert!(status.is_none());
}

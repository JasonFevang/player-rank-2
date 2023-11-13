// Import common module
mod common;

use player_rank_cli::player_rank_lib::*;

#[test]
fn get_first_question() {
    // Create a player rank
    let players = common::test_players(26);
    let mut questions = Questions::new();
    let mut player_rank = PlayerRank::new(&players, &mut questions, Some(0));

    // Get first question
    let (question, status) = player_rank.get_next_question();

    // Validate correct question
    assert_eq!(
        question,
        Some(Question {
            player1: String::from("Samuel"),
            pos1: Position::Atk,
            player2: String::from("William"),
            pos2: Position::Atk
        })
    );

    // Should start in the attack stage
    assert_eq!(status, Some(QuestionStatus::StartingStage(Stage::first())));
}

#[test]
#[ignore]
fn repeatedly_skip() {
    // Create a player rank
    let players = common::test_players(4);
    let mut questions = Questions::new();
    let mut player_rank = PlayerRank::new(&players, &mut questions, Some(0));

    // Get the first 20 questions
    for _ in 0..20 {
        let (question, status) = player_rank.get_next_question();
        player_rank.give_response(1.0);
        println!("q: '{:?}', s: '{:?}'", question, status);
    }
}

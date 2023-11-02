use anyhow::Result;
use crate::player_rank_lib;

pub fn parse_player_file(_player_file: &std::path::PathBuf) -> player_rank_lib::Players {
    let mut players = player_rank_lib::Players::new();
    players.players.push(player_rank_lib::Player {
        name: String::from("Jason"),
        goalie: true,
        avail1: true,
        avail2: true,
    });
    players
}

pub fn parse_question_file(_question_file: &std::path::PathBuf) -> player_rank_lib::Questions {
    let mut questions = player_rank_lib::Questions::new();

    // Add a dummy question
    questions.questions.push(player_rank_lib::AnsweredQuestion {
        question: player_rank_lib::Question {
            player1: String::from("Jason"),
            pos1: player_rank_lib::Position::Atk,
            player2: String::from("Max"),
            pos2: player_rank_lib::Position::Def,
        },
        response: 1.2,
    });
    questions
}

pub fn write_question_file(
    _file: &std::path::PathBuf,
    _questions: &player_rank_lib::Questions,
) -> Result<()> {
    Ok(())
}

pub fn write_rank_file(_file: &std::path::PathBuf, _ranks: &player_rank_lib::Ranks) -> Result<()> {
    Ok(())
}

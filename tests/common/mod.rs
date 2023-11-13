use player_rank_cli::player_rank_lib::{Player, Players};

pub fn test_players(num_players: usize) -> Players {
    let names = vec![
        "Alice",
        "Bob",
        "Charlotte",
        "David",
        "Emily",
        "Frank",
        "Grace",
        "Henry",
        "Isabella",
        "Jack",
        "Kate",
        "Liam",
        "Mia",
        "Noah",
        "Olivia",
        "Peter",
        "Quinn",
        "Rachel",
        "Samuel",
        "Taylor",
        "Ulysses",
        "Victoria",
        "William",
        "Xander",
        "Yasmine",
        "Zachary",
    ];

    assert!(num_players <= names.len());

    let mut players = Players::new();

    // Define a closure for adding players
    let mut add_player = |name: String, goalie: bool| {
        players.players.push(Player {
            name,
            goalie,
            avail1: false,
            avail2: false,
        });
    };

    for name in names.iter().take(num_players) {
        add_player(String::from(*name), false);
    }

    players
}

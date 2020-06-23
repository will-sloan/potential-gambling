mod poker;

fn main() {
    let mut game = poker::Game::new_game();
    print_out(&game);
    poker::shuffle_deck(&mut game.deck);
    print_out(&game);
    let mut player1 = poker::Player {
        cards: Vec::new(),
        chips: 500,
        ip: String::from("Will"),
        folded: false,
        hand: 0,
    };
    // let mut player2 = poker::Player {
    //     cards: Vec::new(),
    //     chips: 500,
    //     ip: String::from("Tom"),
    //     folded: false,
    //     hand: 0,
    // };
    // let mut player3 = poker::Player {
    //     cards: Vec::new(),
    //     chips: 500,
    //     ip: String::from("Bear"),
    //     folded: false,
    //     hand: 0,
    // };

    //game.players = vec![player1, player2, player3];
    game.players = vec![player1];

    game.deal_to_players();

    // for player in game.players.iter() {
    //     println!("{:#?}", player);
    // }
    println!("Printing Initial Deal");
    print_player(&game.players);
    println!("\nchecking value of cards");
    game.check_cards();
    print_player(&game.players);
    println!("\nShowing the flop");
    game.do_flop();
    print_cards(&game.flop);
    println!("\nCalling check cards again");
    game.check_cards();
    print_player(&game.players);
    println!("\nShowing next two cards, updating scores, then showingflop and cards");
    game.flip_one();
    game.flip_one();
    game.check_cards();
    print_cards(&game.flop);
    print_player(&game.players);
}

fn print_out(game: &poker::Game) {
    for (i, v) in game.deck.iter().enumerate() {
        println!("{:?} {:?}", i, v);
    }
}

fn print_cards(cards: &Vec<poker::Card>) {
    for card in cards.iter() {
        println!("{:#?}", card);
    }
}

fn print_player(players: &Vec<poker::Player>) {
    for player in players.iter() {
        println!("{:#?}", player);
    }
}

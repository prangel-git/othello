use othello::*;

/// Two alphabeta prunning players playing othello
fn main() {
    // Making a weighted board for testing
    let mut weights = WeightedBoard::new();
    weights.set_reward(0, 10);
    weights.set_reward(1, 11);
    weights.set_reward(2, 12);
    weights.set_reward(3, 13);
    weights.set_reward(4, 14);
    weights.set_reward(5, 15);
    weights.set_reward(6, 16);
    weights.set_reward(7, 17);
    weights.set_reward(8, 18);

    for i in 0..8 {
        for j in 0..8 {
            print!("|{:02}|", weights.get_weight(i, j));
        }
        print! {"\n"};
    }

    // Testing weighted board during play.

    let mut board = Board::initial_state();

    let reward = |env: &Board, agent: &AgentId| weights.reward(env, agent);

    let mut dark = AlphabetaAgent::new(AgentId::Black, &reward, 1);
    let mut light = AlphabetaAgent::new(AgentId::White, &reward, 1);

    let log = play(&mut board, &mut dark, &mut light);

    for (agent, mv) in log {
        println!("Player: {}, moved {}", agent, mv);
    }

    println!("Last Board \n{}", board);

    println!(
        "Final Score: Black {}, White {}",
        board.count_black(),
        board.count_white()
    );

    let winner = board.winner();

    match winner {
        Some(w) => println!("\nPlayer {:?} wins.", w),
        None => println!("\nThe game ended in a draw."),
    }
}

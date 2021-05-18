use othello::*;

/// Two alphabeta prunning players playing othello
fn main() {
    // Making a weighted board for testing
    let weights = WeightedBoard::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);

    println!("Weighted board \n {:}", weights);

    // Testing weighted board during play.

    let mut board = Board::initial_state();

    let reward = Box::new(|env: &Board, agent: &AgentId| weights.reward(env, agent));

    let mut dark = AlphabetaAgent::new(AgentId::Black, &reward, 5);
    let mut light = AlphabetaAgent::new(AgentId::White, &reward, 5);

    let log = play(&mut board, &mut dark, &mut light);

    for (agent, mv) in log {
        println!("Player: {}, moved {}", agent, mv);
    }

    println!("Last Board \n{}", board);

    println!(
        "Final Score: Black {}, White {}",
        weights.reward(&board, &AgentId::Black),
        weights.reward(&board, &AgentId::White)
    );

    let winner = board.winner();

    match winner {
        Some(w) => println!("\nPlayer {:?} wins.", w),
        None => println!("\nThe game ended in a draw."),
    }
}

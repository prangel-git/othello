use othello::*;

/// Two montecarlo players playing othello
fn main() {
    let mut board = Board::initial_state();

    let mut dark = MctsUctAgent::new(AgentId::Black, 2f64, 1);
    let mut light = MctsUctAgent::new(AgentId::White, 2f64, 1);

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

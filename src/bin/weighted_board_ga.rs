use othello::*;

fn matching(x: &WeightedBoard, y: &WeightedBoard) -> f64 {
    let reward_x: Box<dyn Fn(&Board, &AgentId) -> f64> =
        Box::new(|env, agent| WeightedBoard::reward(x, env, agent));
    let reward_y: Box<dyn Fn(&Board, &AgentId) -> f64> =
        Box::new(|env, agent| WeightedBoard::reward(y, env, agent));

    let mut dark = AlphabetaAgent::new(AgentId::Black, &reward_x, 1);
    let mut light = AlphabetaAgent::new(AgentId::White, &reward_y, 1);

    let mut board = Board::initial_state();

    play(&mut board, &mut dark, &mut light);

    match board.winner() {
        Some(AgentId::Black) => -1f64,
        Some(AgentId::White) => 1f64,
        None => 0f64,
    }
}

fn main() {
    let params = AlgorithmParams {
        rounds: 1000,
        max_population: 8,
        mutation_rate: 0.1,
        co_rate: 0.5,
    };

    let matching_b: Box<dyn Fn(&WeightedBoard, &WeightedBoard) -> f64> = Box::new(matching);

    let last_population = ga_tournament_selection(&Vec::new(), &params, &matching_b);

    println!("Last population:");
    for weight in &last_population {
        println!("weight: {:?}", weight);
    }

    println!("Best board: \n {:}", &last_population[0]);
}

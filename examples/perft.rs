use games::{
    ataxx::AtaxxPosition,
    connect4::Connect4Position,
    droptaxx::DroptaxxPosition,
    gamerules::{GameRules, default_perft},
    isolation::IsolationPosition,
};
use std::time::Instant;

fn run_table<G: GameRules>(name: &str, max_depth: i32) {
    println!("{name}");
    println!(
        "{:>5} {:>12} {:>16} {:>17} {:>9}",
        "depth", "nodes", "generic /ms", "specialised /ms", "speedup"
    );

    for depth in 1..=max_depth {
        let mut pos = G::startpos();

        let start = Instant::now();
        let generic_nodes = default_perft(&mut pos, depth);
        let generic = start.elapsed();

        let start = Instant::now();
        let specialised_nodes = pos.perft(depth);
        let specialised = start.elapsed();

        assert_eq!(generic_nodes, specialised_nodes);

        let speedup = generic.as_secs_f64() / specialised.as_secs_f64();
        println!(
            "{:>5} {:>12} {:>16} {:>17} {:>8.2}x",
            depth,
            generic_nodes,
            generic.as_millis(),
            specialised.as_millis(),
            speedup
        );
    }

    println!();
}

fn main() {
    run_table::<AtaxxPosition<7, 7>>("ataxx", 7);
    run_table::<Connect4Position>("connect4", 10);
    run_table::<DroptaxxPosition>("droptaxx", 6);
    run_table::<IsolationPosition>("isolation", 4);
}

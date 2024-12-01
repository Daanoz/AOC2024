use clap::Parser;

mod solutions;

#[derive(Parser, Debug)]
#[command(name = "AOC 2024")]
#[command(author = "Daan Sieben")]
#[command(version = "1.0")]
#[command(about, long_about = None)]
struct Args {
    /// AOC Session id; if not set uses env var AOC_SESSION
    #[arg(long)]
    aoc_session: Option<String>,
    /// Puzzle day to run
    #[arg(short, long)]
    day: Option<u32>,
}

#[cfg(feature = "render")]
#[aoc_core::aoc(year = 2024)]
fn main() {
    aoc_bevy_render::run(solutions::get_collection());
}

#[cfg(not(feature = "render"))]
#[aoc_core::aoc(year = 2024)]
fn main() {
    let args = Args::parse();
    if let Some(session_id) = args.aoc_session {
        aoc_core::set_session(session_id);
    }
    solutions::run(args.day);
}

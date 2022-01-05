use game::run;

mod game;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        eprintln!("Exiting...");
        std::process::exit(1);
    }
}

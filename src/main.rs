mod cli;
mod naga;
mod server;
mod wgsl_error;

fn main() {
    let last = std::env::args().next_back();

    let exit_code = if let Some(last) = last {
        if last == "--server" {
            server::run();
            0
        } else {
            cli::run()
        }
    } else {
        cli::run()
    };

    std::process::exit(exit_code);
}

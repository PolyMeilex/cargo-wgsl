mod cli;
mod lsp;
mod naga;

fn main() {
    let last = std::env::args().last();

    let exit_code = if let Some(last) = last {
        if last == "--server" {
            // server::run();
            0
        } else if last == "--lsp-server" {
            lsp::run().ok();
            0
        } else {
            cli::run()
        }
    } else {
        cli::run()
    };

    std::process::exit(exit_code);
}

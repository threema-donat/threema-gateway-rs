use std::process;

use docopt::Docopt;
use threema_gateway::ApiBuilder;

const USAGE: &str = "
Usage: lookup_credits [options] <from> <secret>

Options:
    -h, --help    Show this help
";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|docopt| docopt.parse())
        .unwrap_or_else(|e| e.exit());

    // Command line arguments
    let from = args.get_str("<from>");
    let secret = args.get_str("<secret>");

    println!("Looking up credits");

    // Look up ID
    let api = ApiBuilder::new(from, secret).into_simple();
    match api.lookup_credits().await {
        Err(e) => {
            println!("Could not look up credits: {e}");
            process::exit(1);
        }
        Ok(credits) => println!("You have {credits} credits remaining"),
    }
}

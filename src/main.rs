use clap::Parser;
use dotenv::dotenv;
use human_panic::setup_panic;

// CLI Config handled through the use of the clap crate
#[derive(Parser, Debug)]
#[command(name="Handler", author="Ishan Joshi", version, about="A Simple CLI to handle my site", long_about = None)]

// The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=true, help="The option you want to use")]
    option: String,

    #[arg(required=false)]
    hash: Option<String>,

    #[arg(required=false, short, long, help="The file you want to use")]
    file: Option<String>
}

fn get_args()-> Args {
    Args::parse()
}

//? Mod Definitions
mod handle;
mod cli;
mod utils;
mod connections;
mod db;

#[tokio::main]
async fn main() {
    // For automatically loading the .env file
    dotenv().ok();
    // For automatically setting up the panic handler
    setup_panic!();
    // Print a small splash screen
    utils::print_splash_screen();

    let args = get_args();
    let option = args.option;
    let hash;
    // We check if the user has actually given the hash as an argument
    if args.hash.is_some() {
        hash = args.hash.unwrap();
    } else {
        // I don't like to impose arguments on the user, so we ask for the hash if it is not given
        hash = cli::get_text("Page Hash", "Enter The Hash of the page you want to edit");
    }
    // We match the option given by the user and call the appropriate function
    // Match functions rock in Rust
    // We use the handle module to handle the different options
    match option.as_str() {
        "set" => handle::set(&hash).await,
        "new" => handle::new(&hash, args.file).await,
        "list" => handle::list(&hash).await,
        "pages" => handle::pages(&hash).await,
        "code" => handle::code(&hash).await,
        "go" => handle::go(&hash).await,
        "speed" => handle::speed(&hash).await,
        _ => println!("Invalid Option"),
    }
}

use clap::Parser;
use dotenv::dotenv;
use human_panic::setup_panic;

// CLI Config handled through the use of the clap crate
#[derive(Parser, Debug)]
#[command(name="Handler", author="Ishan Joshi", version, about="A Simple CLI to handle my site", long_about = None)]

// The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required = true, help = "The option you want to use")]
    option: String,

    #[arg(required = false)]
    iden: String,

    #[arg(required = false, short, long, help = "The file you want to use")]
    file: Option<String>,
}

fn get_args() -> Args {
    Args::parse()
}

//? Mod Definitions
mod cli;
mod db;
mod file;
mod handle;
mod utils;

#[tokio::main]
async fn main() {
    file::change_to_config_path();
    // For automatically loading the .env file
    dotenv().ok();
    // For automatically setting up the panic handler
    setup_panic!();
    // Print a small splash screen
    utils::print_splash_screen();

    let args = get_args();
    let option = args.option;
    // We check if the user has actually given the hash as an argument
    let iden = args.iden;
    // We match the option given by the user and call the appropriate function
    // Match functions rock in Rust
    // We use the handle module to handle the different options
    match option.as_str() {
        "set" => handle::set(&iden).await,
        "new" => handle::new(&iden, args.file).await,
        "list" => handle::list(&iden).await,
        "pages" => handle::pages(&iden).await,
        "code" => handle::code(&iden).await,
        "go" => handle::go(&iden).await,
        "speed" => handle::speed(&iden).await,
        _ => println!("Invalid Option"),
    }
}

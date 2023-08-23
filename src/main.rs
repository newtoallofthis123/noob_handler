use clap::Parser;
use dotenv::dotenv;

// CLI Config handled through the use of the clap crate
#[derive(Parser, Debug)]
#[command(name="Handler", author="Ishan Joshi", version, about="A Simple CLI to handle my site", long_about = None)]

// The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=true)]
    option: String,

    #[arg(required=false)]
    hash: Option<String>,
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
    bunt::println!("{$blue} _   _             _     _   _                 _ _           {/$}");
    bunt::println!("{$yellow}| \\ | | ___   ___ | |__ | | | | __ _ _ __   __| | | ___ _ __ {/$}");
    bunt::println!("{$red}|  \\| |/ _ \\ / _ \\| '_ \\| |_| |/ _` | '_ \\ / _` | |/ _ \\ '__|{/$}");
    bunt::println!("{$yellow}| |\\  | (_) | (_) | |_) |  _  | (_| | | | | (_| | |  __/ |   {/$}");
    bunt::println!("{$blue}|_| \\_|\\___/ \\___/|_.__/|_| |_|\\__,_|_| |_|\\__,_|_|\\___|_|   {/$}");
    println!("\n");
    let args = get_args();
    let option = args.option;
    let hash;
    if args.hash.is_some() {
        hash = args.hash.unwrap();
    } else {
        hash = cli::get_text("Page Hash", "Enter The Hash of the page you want to edit");
    }
    match option.as_str() {
        "new" => handle::new(&hash).await,
        "list" => handle::list(&hash).await,
        "pages" => handle::pages(&hash).await,
        "code" => handle::code(&hash).await,
        "go" => handle::go(&hash).await,
        _ => println!("Invalid Option"),
    }
}

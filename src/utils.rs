use rand::Rng;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub fn get_mongo_url()->String{
    std::env::var("MONGODB_URL").expect("MONGODB_URL not set")
}

pub fn title_to_hash(title: &str)->String{
    title.to_lowercase().replace(' ', "-").to_lowercase()
}

pub fn random_hash()->String{
    let mut rng = rand::thread_rng();
    let rand_string: String = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
        .take(6)
        .collect();
    rand_string
}

pub fn match_lang(lang: &str)->String{
    match lang{
        "python" => String::from(".py"),
        "rust" => String::from(".rs"),
        "javascript" => String::from(".js"),
        "typescript" => String::from(".ts"),
        "c" => String::from(".c"),
        "r" => String::from(".r"),
        "jsx" => String::from(".jsx"),
        "tsx" => String::from(".tsx"),
        "c++" => String::from(".cpp"),
        "c#" => String::from(".cs"),
        "go" => String::from(".go"),
        "java" => String::from(".java"),
        // So weird, that the rust case is not working
        // TODO: Open Issue on rust-lang/rust repo
        "rustlang" => String::from(".rs"),
        "kotlin" => String::from(".kt"),
        "ruby" => String::from(".rb"),
        "php" => String::from(".php"),
        "html" => String::from(".html"),
        "css" => String::from(".css"),
        _ => String::from(".txt"),
    }
}

pub fn copy(msg: &str){
    let mut board: ClipboardContext = ClipboardProvider::new().unwrap();
    board.set_contents(msg.to_owned()).unwrap();
}

pub fn print_splash_screen(){
    bunt::println!("{$blue} _   _             _     _   _                 _ _           {/$}");
    bunt::println!("{$yellow}| \\ | | ___   ___ | |__ | | | | __ _ _ __   __| | | ___ _ __ {/$}");
    bunt::println!("{$red}|  \\| |/ _ \\ / _ \\| '_ \\| |_| |/ _` | '_ \\ / _` | |/ _ \\ '__|{/$}");
    bunt::println!("{$yellow}| |\\  | (_) | (_) | |_) |  _  | (_| | | | | (_| | |  __/ |   {/$}");
    bunt::println!("{$blue}|_| \\_|\\___/ \\___/|_.__/|_| |_|\\__,_|_| |_|\\__,_|_|\\___|_|   {/$}");
    println!("\n");
}

use rand::Rng;

pub fn get_mongo_url()->String{
    let mongo_url = std::env::var("MONGODB_URL").expect("MONGODB_URL not set");
    mongo_url
}

pub fn title_to_hash(title: &str)->String{
    let hash = title.to_lowercase().replace(" ", "-").to_lowercase();
    hash
}

pub fn random_hash()->String{
    let mut rng = rand::thread_rng();
    let rand_string: String = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
        .take(8)
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
        "c++" => String::from(".cpp"),
        "c#" => String::from(".cs"),
        "go" => String::from(".go"),
        "java" => String::from(".java"),
        "kotlin" => String::from(".kt"),
        "ruby" => String::from(".rb"),
        "php" => String::from(".php"),
        "html" => String::from(".html"),
        "css" => String::from(".css"),
        _ => String::from(".txt"),
    }
}
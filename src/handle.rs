use crate::{cli, db::{self, Page, Code, Specials}, utils};
use std::io::Write;

pub async fn pages(hash:&str){
    bunt::println!("Currently in Page: {$cyan}{}{/$}", hash);
    let page = db::get_page(&hash).await;
    let option = cli::get_option(vec!["Show", "Edit", "Delete"]);
    match option.as_str() {
        "Show" => {
            cli::display_page(&page);
        },
        "Edit" => {
            let edited_page = cli::ask_page(&page);
            bunt::println!("Edited Page: {$green}{}{/$}", edited_page.name);
            cli::display_page(&edited_page);
            let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
            if confirmation {
                db::update_page(hash, &edited_page).await;
                bunt::println!("Updated {$green}{}{/$}", edited_page.name);
                utils::copy(format!("https://noobscience.rocks/quips/{}", edited_page.hash).as_str());
                bunt::println!("Copied URL to clipboard");
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        "Delete" => {
            let confirmation = cli::get_confirmation("Are you sure you want to delete this page?");
            if confirmation {
                db::delete_page(hash).await;
                bunt::println!("Deleted {$green}{}{/$}", page.name);
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        _ => println!("Invalid Option"),
    }
}

pub async fn code(hash: &str){
    bunt::println!("Currently in Code: {$cyan}{}{/$}", hash);
    let code_doc = db::get_code(&hash).await;
    let option = cli::get_option(vec!["Show", "Edit", "Delete"]);
    match option.as_str() {
        "Show" => {
            cli::display_code(&code_doc);
        },
        "Edit" => {
            let edited_code = cli::ask_code(&code_doc);
            bunt::println!("Edited Code: {$green}{}{/$}", edited_code.title);
            let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
            if confirmation {
                db::update_code(hash, &edited_code).await;
                bunt::println!("Added {$green}{}{/$}", edited_code.title);
                utils::copy(format!("https://noobscience.rocks/code/{}", edited_code.hash).as_str());
                bunt::println!("Copied URL to clipboard");
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        "Delete" => {
            let confirmation = cli::get_confirmation("Are you sure you want to delete this page?");
            if confirmation {
                db::delete_code(hash).await;
                bunt::println!("Deleted {$green}{}{/$}", code_doc.title);
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        _ => println!("Invalid Option"),
    }   
}

pub async fn go(hash: &str){
    bunt::println!("Currently in Go: {$cyan}{}{/$}", hash);
    let go_doc = db::get_go(&hash).await;
    let option = cli::get_option(vec!["Show", "Edit", "Delete"]);
    match option.as_str() {
        "Show" => {
            cli::display_go(&go_doc);
        },
        "Edit" => {
            let edited_go = cli::ask_go(&go_doc);
            bunt::println!("Edited Go: {$green}{}{/$}", edited_go.slug);
            let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
            if confirmation {
                db::update_go(hash, &edited_go).await;
                bunt::println!("Edited {$green}{}{/$}", edited_go.slug);
                utils::copy(format!("https://noobscience.rocks/go/{}", edited_go.slug).as_str());
                bunt::println!("Copied URL to clipboard");
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        "Delete" => {
            let confirmation = cli::get_confirmation("Are you sure you want to delete this page?");
            if confirmation {
                db::delete_go(hash).await;
                bunt::println!("Deleted {$green}{}{/$}", go_doc.slug);
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        _ => println!("Invalid Option"),
    }   
}

pub async fn new_page(){
    bunt::println!("Initializing New Page");
    let page:Page = Page{
        _id: mongodb::bson::oid::ObjectId::new(),
        hash: String::from(""),
        name: String::from(""),
        content: String::from(""),
        date: String::from(""),
        author: String::from(""),
    };
    let mut edited_page = cli::ask_page(&page);
    bunt::println!("Edited Page: {$green}{}{/$}", edited_page.name);
    edited_page.hash = crate::utils::title_to_hash(&edited_page.name);
    cli::display_page(&edited_page);
    let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
    if confirmation {
        db::insert_page(&edited_page).await;
        bunt::println!("Updated {$green}{}{/$}", edited_page.name);
        utils::copy(format!("https://noobscience.rocks/quips/{}", edited_page.hash).as_str());
        bunt::println!("Copied URL to clipboard");
    }
    else{
        bunt::println!("{$red}Aborted{/$}");
    }
}

pub async fn new_code(){
    bunt::println!("Initializing New Code");
    let code:Code = Code{
        _id: mongodb::bson::oid::ObjectId::new(),
        hash: String::from(""),
        title: String::from(""),
        content: String::from(""),
        lang: String::from(""),
        author: String::from(""),
    };
    let mut edited_code = cli::ask_code(&code);
    bunt::println!("Initialized Page: {$green}{}{/$}", edited_code.title);
    edited_code.hash = crate::utils::random_hash();
    let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
    if confirmation {
        db::insert_code(&edited_code).await;
        bunt::println!("Added {$green}{}{/$}", edited_code.title);
        utils::copy(format!("https://noobscience.rocks/code/{}", edited_code.hash).as_str());
        bunt::println!("Copied URL to clipboard");
    }
    else{
        bunt::println!("{$red}Aborted{/$}");
    }
}

pub async fn new_go(){
    bunt::println!("Initializing New Go");
    let edited_go = cli::ask_new_go();
    bunt::println!("Initialized Go: {$green}{}{/$}", edited_go.slug);
    let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
    if confirmation {
        db::insert_go(&edited_go).await;
        bunt::println!("Added {$green}{}{/$}", edited_go.slug);
        utils::copy(format!("https://noobscience.rocks/go/{}", edited_go.slug).as_str());
        bunt::println!("Copied URL to clipboard");
    }
    else{
        bunt::println!("{$red}Aborted{/$}");
    }
}

pub async fn new(doc:&str){
    match doc {
        "pages" => {
            new_page().await;            
        },
        "code" => {
            new_code().await;
        }
        "go" => {
            new_go().await;
        }
        _ => println!("Invalid Option"),
    }
}

pub async fn list_pages(){
    let pages_vec = db::get_all_pages().await;
    let title_list:Vec<String> = pages_vec.iter().map(|page| page.name.clone()).collect();
    //convert vec<String> to vec<&str>
    let title_list:Vec<&str> = title_list.iter().map(|title| title.as_str()).collect();
    let option = cli::get_option(title_list);
    let page = pages_vec.iter().find(|page| page.name == option).unwrap();
    pages(page.hash.as_str()).await;
}

pub async fn list_codes(){
    let pages_vec = db::get_all_codes().await;
    let title_list:Vec<String> = pages_vec.iter().map(|page| page.title.clone()).collect();
    //convert vec<String> to vec<&str>
    let title_list:Vec<&str> = title_list.iter().map(|title| title.as_str()).collect();
    let option = cli::get_option(title_list);
    let page = pages_vec.iter().find(|page| page.title == option).unwrap();
    code(page.hash.as_str()).await;
}

pub async fn list(collection:&str){
    match collection {
        "pages" => {
            list_pages().await;
        },
        "code" => {
            list_codes().await;
        },
        "go" => {
            bunt::println!("Not Implemented Yet");
        }
        _ => println!("Invalid Option"),
    }
}

pub async fn set(thing:&str){
    let mut special:Specials = db::get_specials().await;
    match thing{
        "current" => {
            let current = cli::get_editor("Current", special.current);
            special.current = current;
        },
        "title" => {
            let title = cli::get_text("Title", "What are you up to?");
            special.title = title;
        },
        "description" => {
            let description = cli::get_editor("Description", special.description);
            special.description = description;
            let date = cli::get_text("Date", "When did you start this?");
            special.date = date;
        },
        _ => println!("Invalid Option"),
    }
    let confirmation = cli::get_confirmation("Are you sure you want to commit this page?");
    if confirmation {
        db::set_specials(&special).await;
        bunt::println!("Updated {$green}{}{/$}", thing);
    }
    else{
        bunt::println!("{$red}Aborted{/$}");
    }
}

pub async fn md(hash:&str){
    bunt::println!("New Markdown File");
    let md_url = utils::get_md_url();
    let md_path = format!("{}\\{}.mdx", md_url, hash);
    let mut file = std::fs::File::create(md_path).expect("Unable to create file");
    let title = cli::get_text("Title", "Title of the post");
    let date = cli::get_text("Date", "Date of the post");
    let author = "Ishan".to_string();
    let emoji = cli::get_text("Emoji", "Emoji of the post");
    let category = cli::get_text("Category", "Category of the post");
    let tags = cli::get_text("Tags", "Tags of the post");
    let post_type = cli::get_text("Type", "Type of the post");
    let bg = inquire::Text::new("Bg of the post").with_default("#fff").prompt().unwrap();
    let fg = inquire::Text::new("Fg of the post").with_default("#000").prompt().unwrap();
    let selection_color = inquire::Text::new("Selection Color of the post").with_default("#fff").prompt().unwrap();
    let selection_bg = inquire::Text::new("Code Bg of the post").with_default("#000").prompt().unwrap();
    let full_img = inquire::Text::new("Full Image").with_default("false").prompt().unwrap();
    let full_img = full_img.parse::<bool>().unwrap();
    let full_img = full_img.to_string();

    let content = format!("---\ntitle: {}\ndate: {}\nauthor: {}\nemoji: {}\nlayout: '../../../layouts/blog_post.astro'\ncategory: {}\ntags: {}\ntype: {}\nbg: {}\nfg: {}\nselection_color: {}\nselection_bg: {}\nfull_img: {}\n---\n", title, date, author, emoji, category, tags, post_type, bg, fg, selection_color, selection_bg, full_img);
    file.write_all(content.as_bytes()).expect("Unable to write data");
    bunt::println!("Created {$green}{}{/$}", title);
}

//check website speed
use reqwest::Client;
use std::time::Instant;

pub async fn speed(url: &str) -> () {
    let final_url:String;
    if url.starts_with("http://") || url.starts_with("https://") {
        bunt::println!("Checking Speed of {$cyan}{}{/$}", url);
        final_url = url.to_string();
    } else {
        bunt::println!("Checking Speed of {$cyan}{}{/$}", format!("https://{}", url));
        final_url = format!("https://{}", url).to_string();
    }
    let client = Client::new();
    let start = Instant::now();
    let res = client.get(final_url).send().await.unwrap();
    let duration = start.elapsed();
    let content_length = res.content_length().unwrap_or(0);
    let status = res.status();
    if duration.as_secs() > 0 {
        bunt::println!("Response Time: {$yellow}{}s{/$}", duration.as_secs());
    } else {
        bunt::println!("Response Time: {$green}{}ms{/$}", duration.as_millis());
    }
    if content_length < 10 {
        bunt::println!("Page Size: {$yellow}{} bytes{/$}", content_length);
    } else {
        bunt::println!("Page Size: {$green}{} bytes{/$}", content_length);
    }
    let duration_secs = duration.as_secs_f64();
    let duration_rounded = (duration_secs * 100.0).round() / 100.0;
    println!("Page Load Time: {:.2}s", duration_rounded);
    if status.is_success() {
        bunt::println!("Status: {$green}{}{/$}", status);
    } else {
        bunt::println!("Status: {$red}{}{/$}", status);
    }
}
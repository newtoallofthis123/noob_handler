use crate::{cli, db::{self, Page, Code}};

pub async fn pages(hash:&str){
    cli::print_title("Pages");
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
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        _ => println!("Invalid Option"),
    }
}

pub async fn new(doc:&str){
    match doc {
        "page" => {
            new_page().await;            
        },
        "code" => {
            new_code().await;
        }
        _ => println!("Invalid Option"),
    }
}

pub async fn new_page(){
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
    }
    else{
        bunt::println!("{$red}Aborted{/$}");
    }
}

pub async fn code(hash: &str){
    cli::print_title("Code");
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
            }
            else{
                bunt::println!("{$red}Aborted{/$}");
            }
        },
        _ => println!("Invalid Option"),
    }   
}

pub async fn new_code(){
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
    }
    else{
        bunt::println!("{$red}Aborted{/$}");
    }
}
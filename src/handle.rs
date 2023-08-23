use crate::{cli, db::{self, Page, Code}, utils};

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

pub async fn new(doc:&str){
    match doc {
        "page" => {
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
        utils::copy(format!("https://noobscience.rocks/page/{}", edited_page.hash).as_str());
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
    let mut edited_go = cli::ask_new_go();
    bunt::println!("Initialized Go: {$green}{}{/$}", edited_go.slug);
    edited_go.slug = crate::utils::random_hash();
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
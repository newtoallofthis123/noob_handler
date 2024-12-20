use crate::{
    db::{Code, Go, Page},
    utils::{match_lang, random_hash},
};

pub fn get_text(msg: &str, help_msg: &str) -> String {
    let question = inquire::Text::new(msg);
    question.clone().with_help_message(help_msg);

    question.prompt().unwrap()
}

pub fn get_option(options: Vec<&str>) -> String {
    let question = inquire::Select::new("Select an option", options).with_page_size(20);

    question.prompt().unwrap().to_string()
}

pub fn get_confirmation(msg: &str) -> bool {
    let question = inquire::Confirm::new(msg).with_default(true);
    question.prompt().unwrap()
}

pub fn get_editor(msg: &str, pre_content: String) -> String {
    let content = inquire::Editor::new(msg)
        .with_file_extension(".md")
        .with_editor_command(std::ffi::OsStr::new("micro"))
        .with_predefined_text(pre_content.as_str())
        .prompt()
        .unwrap();
    content
}

pub fn ask_page(page: &Page) -> Page {
    println!("{}", format!("Editing {}", page.name).as_str());
    let name = inquire::Text::new("Name")
        .with_default(page.name.as_str())
        .prompt()
        .unwrap();
    let content = inquire::Editor::new("Content")
        .with_file_extension(".md")
        .with_editor_command(std::ffi::OsStr::new("micro"))
        .with_predefined_text(page.content.as_str())
        .prompt()
        .unwrap();
    let author = inquire::Text::new("Author")
        .with_default(page.author.as_str())
        .prompt()
        .unwrap();
    let date = inquire::Text::new("Date")
        .with_default(page.date.as_str())
        .prompt()
        .unwrap();

    Page {
        _id: page._id,
        hash: page.hash.clone(),
        name,
        content,
        author,
        date,
    }
}

pub fn display_page(page: &Page) {
    bunt::println!("Showing Page: {$blue}{$underline}{}{/$}{/$}", page.name);
    bunt::println!("With Name: {$green}{}{/$}", page.name);
    bunt::println!("Written on {$yellow}{}{/$}", page.date);
    bunt::println!("Author: {$cyan}{}{/$}", page.author);
    bunt::println!("------------------------------");
    bunt::println!("{}", page.content);
}

pub fn display_code(code: &Code) {
    bunt::println!(
        "Showing Code Snippet: {$blue}{$underline}{}{/$}{/$}",
        code.title
    );
    bunt::println!("Written by {$yellow}{}{/$}", code.author);
    bunt::println!("Language: {$cyan}{}{/$}", code.lang);
    bunt::println!("------------------------------");
    bunt::println!(
        "Opening Code In Editor with language {}...Changing it won't change the actual code",
        code.lang
    );
    let _content = inquire::Editor::new("Content")
        .with_file_extension(match_lang(&code.lang).as_str())
        .with_editor_command(std::ffi::OsStr::new("vim"))
        .with_predefined_text(code.content.as_str())
        .prompt()
        .unwrap();
}

pub fn display_go(go: &Go) {
    bunt::println!(
        "Showing Go with hash: {$blue}{$underline}{}{/$}{/$}",
        go.slug
    );
    bunt::println!("With URL: {$green}{}{/$}", go.url);
}

pub fn ask_code(code: &Code) -> Code {
    println!("{}", format!("Editing {}", code.title).as_str());
    let title = inquire::Text::new("Title")
        .with_default(code.title.as_str())
        .prompt()
        .unwrap();
    let lang = inquire::Text::new("Lang")
        .with_default(code.lang.as_str())
        .prompt()
        .unwrap();
    let content: String = inquire::Editor::new("Content")
        .with_file_extension(&match_lang(&lang))
        .with_editor_command(std::ffi::OsStr::new("vim"))
        .with_predefined_text(code.content.as_str())
        .prompt()
        .unwrap();
    let author = inquire::Text::new("Author")
        .with_default(code.author.as_str())
        .prompt()
        .unwrap();

    Code {
        _id: code._id,
        hash: code.hash.clone(),
        title,
        content,
        author,
        lang,
    }
}

pub fn ask_go(go: &Go) -> Go {
    println!("{}", format!("Editing {}", go.slug).as_str());
    let slug = inquire::Text::new("Slug")
        .with_default(go.slug.as_str())
        .prompt()
        .unwrap();
    let url = inquire::Text::new("URL")
        .with_default(go.url.as_str())
        .prompt()
        .unwrap();

    Go {
        _id: go._id,
        slug,
        url,
    }
}

pub fn ask_new_go() -> Go {
    let slug = inquire::Text::new("Slug")
        .with_default(random_hash().as_str())
        .prompt()
        .unwrap();
    let url = inquire::Text::new("URL").prompt().unwrap();

    Go {
        _id: mongodb::bson::oid::ObjectId::new(),
        slug,
        url,
    }
}

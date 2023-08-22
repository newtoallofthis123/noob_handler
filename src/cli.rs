use crate::{db::{Page, Code}, utils::match_lang};

pub fn get_text(msg: &str, help_msg: &str)->String{
    let question = inquire::Text::new(msg);
    question.clone().with_help_message(help_msg);
    let answer = question.prompt().unwrap();
    answer
}

pub fn get_option(options: Vec<&str>)->String{
    let question = inquire::Select::new("Select an option", options);
    let answer = question.prompt().unwrap().to_string();
    answer
}

pub fn get_confirmation(msg: &str)->bool{
    let question = inquire::Confirm::new(msg);
    let answer = question.prompt().unwrap();
    answer
}

pub fn print_title(title: &str){
    bunt::println!("In {$bold}{$blue}{$underline}{}{/$}{/$}{/$}", title);
}

pub fn ask_page(page: &Page)->Page{
    println!("{}", format!("Editing {}", page.name).as_str());
    let name = inquire::Text::new("Name").with_default(page.name.as_str()).prompt().unwrap();
    let content = inquire::Editor::new("Content").with_file_extension(".md").with_editor_command(std::ffi::OsStr::new("vim")).with_predefined_text(page.content.as_str()).prompt().unwrap();
    let author = inquire::Text::new("Author").with_default(page.author.as_str()).prompt().unwrap();
    let date = inquire::Text::new("Date").with_default(page.date.as_str()).prompt().unwrap();
    
    let page = Page{
        _id: page._id.clone(),
        hash: page.hash.clone(),
        name,
        content,
        author,
        date,
    };
    page
}

pub fn display_page(page: &Page){
    bunt::println!("##############################");
    bunt::println!("Showing Page: {$blue}{$underline}{}{/$}{/$}", page.name);
    bunt::println!("With Name: {$green}{}{/$}", page.name);
    bunt::println!("Written on {$yellow}{}{/$}", page.date);
    bunt::println!("Author: {$cyan}{}{/$}", page.author);
    bunt::println!("------------------------------");
    bunt::println!("{}", page.content);
    bunt::println!("##############################");
}

pub fn display_code(code: &Code){
    bunt::println!("##############################");
    bunt::println!("Showing Code Snippet: {$blue}{$underline}{}{/$}{/$}", code.title);
    bunt::println!("Written by {$yellow}{}{/$}", code.author);
    bunt::println!("Language: {$cyan}{}{/$}", code.lang);
    bunt::println!("------------------------------");
    bunt::println!("Opening Code In Editor with language {}...Changing it won't change the actual code", code.lang);
    let _content = inquire::Editor::new("Content").with_file_extension(match_lang(&code.lang).as_str()).with_editor_command(std::ffi::OsStr::new("vim")).with_predefined_text(code.content.as_str()).prompt().unwrap();
    bunt::println!("##############################");
}

pub fn ask_code(code: &Code)->Code{
    println!("{}", format!("Editing {}", code.title).as_str());
    let title = inquire::Text::new("Title").with_default(code.title.as_str()).prompt().unwrap();
    let lang = inquire::Text::new("Lang").with_default(code.lang.as_str()).prompt().unwrap();
    let content = inquire::Editor::new("Content").with_file_extension(&match_lang(&lang)).with_editor_command(std::ffi::OsStr::new("vim")).with_predefined_text(code.content.as_str()).prompt().unwrap();
    let author = inquire::Text::new("Author").with_default(code.author.as_str()).prompt().unwrap();
    
    let code_doc = Code{
        _id: code._id.clone(),
        hash: code.hash.clone(),
        title,
        content,
        author,
        lang,
    };
    code_doc
}
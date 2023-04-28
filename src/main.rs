mod lexer;
mod tokens;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, file_path, ..] = &args.as_slice() else {
        panic!("Please provide a file name");
    };

    let file_contents = match std::fs::read_to_string(&file_path) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Could not open the file at \"{}\"", &file_path);
            eprintln!("Error: {}", e);
            match std::env::current_dir() {
                Ok(dir) => eprintln!("Current working directory is {}", dir.display()),
                Err(err) => {
                    eprintln!("Could not get current working directory.");
                    eprintln!("Error: {}", err);
                }
            }
            std::process::exit(1);
        }
    };

    let lexed = lexer::lex(file_contents.as_str());
    println!("{:#?}", lexed);
}

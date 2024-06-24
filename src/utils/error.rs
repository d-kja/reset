use owo_colors::OwoColorize;

pub fn process_exit(error: Box<dyn std::error::Error>) {
    let message = error.to_string();
    println!("{}", message.red().bold());
}

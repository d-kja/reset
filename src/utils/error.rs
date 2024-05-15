pub fn process_exit(message: &str) {
    println!("{}", message.red().bold());
    process::exit(1);
}

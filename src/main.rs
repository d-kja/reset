use owo_colors::OwoColorize;
use std::{
    env,
    process::{self, Command},
};

#[derive(Debug)]
enum Commands {
    Repo,
    Docker,
    Invalid,
}

// #[derive(Debug)]
// enum Action {
//     Reset,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    let _current_path = args.next().expect("unable to read the current path");

    let command = args
        .next()
        .expect("command is required, use the -h flag as a guide");

    let command = match &command[..] {
        "-r" => Commands::Repo,
        "repo" => Commands::Repo,
        "-d" => Commands::Docker,
        "docker" => Commands::Docker,
        "-h" => {
            println!(
                "\n\r{}\n\r {} {}\n\r {} {}",
                "Available Commands:".bold().bright_cyan(),
                "- repo".italic(),
                "(-r)".dimmed(),
                "- docker".italic(),
                "(-d)".dimmed()
            );
            println!(
                "\n\r{}\n\r {}: {}",
                "Extra:".bright_magenta().italic(),
                "--prisma".italic(),
                "migrate the current schema".dimmed().italic()
            );
            // println!("\n\rAvailable Actions:\n\r - reset (-r)\n\r");

            process_exit("\nChoose one of the options above, i.e. \"repo reset\", you can also use the short version \"r r\" ");
            Commands::Invalid
        }
        _ => {
            process_exit("\ninvalid command, try -h to see the available options");
            Commands::Invalid
        }
    };

    // # TODO: action feature
    //
    // let action = args
    //     .next()
    //     .expect("action is required, use the -h flag as a guide");
    //
    // let action = match &action[..] {
    //     "r" => Action::Reset,
    //     "reset" => Action::Reset,
    //     _ => panic!("invalid command, try -h to "),
    // };

    match command {
        Commands::Repo => {
            if cfg!(target_os = "windows") {
                panic!("Windows, really? Go download WSL")
            } else {
                println!("{}\r\n", "Searching for the current branch".dimmed());
                let branch = Command::new("git")
                    .args(["branch", "--show-current"])
                    .output()
                    .expect("Unable to retrieve the current git branch");
                let branch = String::from_utf8(branch.stdout).unwrap() as String;
                let branch = branch.replace("\n", "");

                println!("{}\r\n", "Fetching any update from github".magenta());
                Command::new("git")
                    .args(["fetch"])
                    .status()
                    .expect("Unable to fetch with git");

                println!(
                    "{} {} {}\r\n",
                    "Pulling any update from the".purple(),
                    branch.cyan(),
                    "branch".cyan()
                );
                Command::new("git")
                    .args(["pull", "origin", &branch])
                    .status()
                    .expect("Unable to pull with git");
            };

            Ok(())
        }
        Commands::Docker => {
            if cfg!(target_os = "windows") {
                panic!("Windows, really? Go download WSL")
            } else {
                println!("{}", "Taking the container down\r\n".cyan());
                Command::new("docker")
                    .args(["compose", "down"])
                    .status()
                    .expect("Unable to take the container down");

                println!(
                    "\r\n{} {}\r\n",
                    "Creating a new instance".cyan(),
                    "of the container using compose".dimmed()
                );
                Command::new("docker")
                    .args(["compose", "up", "-d"])
                    .status()
                    .expect("Unable to launch a new detached container");

                let migration = args.next();

                if let Some(value) = migration {
                    match value.as_str() {
                        "--prisma" => {
                            println!(
                                "\r\n{} {}",
                                "Migrating the schema".bright_purple(),
                                "and generating the types".dimmed()
                            );
                            Command::new("bunx")
                                .args(["prisma", "migrate", "dev"])
                                .status()
                                .expect("Unable to run bunx with the prisma binary");
                        }
                        _ => (),
                    }
                }
            };

            Ok(())
        }
        Commands::Invalid => panic!("Something went wrong"),
    }
}

fn process_exit(message: &str) {
    println!("{}", message.red().bold());
    process::exit(1);
}

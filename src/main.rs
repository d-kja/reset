use owo_colors::OwoColorize;
use std::{
    env::{self, Args},
    process::{self, Command},
    thread,
    time::Duration,
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
    let mut args_for_flags = env::args();

    let _current_path = args.next().expect("unable to read the current path");

    let command = args.next();

    if let None = command {
        welcome_message();
    }

    let command = command.expect("command is required, use the -h flag as a guide");

    let command = match &command[..] {
        "repo" | "r" => Commands::Repo,
        "docker" | "d" => Commands::Docker,
        "h" | "help" => {
            println!(
                "\r\n{}\r\n {} {}\r\n {} {}",
                "Available Commands:".bold().bright_cyan(),
                "- repo".italic(),
                "(r)".dimmed(),
                "- docker".italic(),
                "(d)".dimmed()
            );
            println!(
                "\r\n{}\r\n {}: {}",
                "Extra:".bright_magenta().italic(),
                "--prisma [timeout u64]".italic(),
                "migrate the current schema".dimmed().italic()
            );
            // println!("\n\rAvailable Actions:\n\r - reset (-r)\n\r");

            process_exit(format!("\r\nChoose one of the options above, i.e. \"repo reset\", you can also use the short version\r\n").as_str());
            Commands::Invalid
        }
        _ => {
            process_exit("\r\nInvalid command, try using help (h) to see the available options");
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
            println!("{}", "Searching for the current branch".dimmed());
            let branch = Command::new("git")
                .args(["branch", "--show-current"])
                .output()
                .expect("Unable to retrieve the current git branch");
            let branch = String::from_utf8(branch.stdout).unwrap() as String;
            let branch = branch.replace("\n", "");

            println!("\r\n{}", "Fetching any update from github".magenta());
            Command::new("git")
                .args(["fetch"])
                .status()
                .expect("Unable to fetch with git");

            println!(
                "\r\n{} {} {}",
                "Pulling any update from the".purple(),
                branch.cyan(),
                "branch".cyan()
            );
            Command::new("git")
                .args(["pull", "origin", &branch])
                .status()
                .expect("Unable to pull with git");

            for (idx, flag) in args.enumerate() {
                let next_flag_idx = idx + 3;
                handle_flags(flag, args_for_flags.nth(next_flag_idx));
            }

            Ok(())
        }
        Commands::Docker => {
            println!("{}", "Taking the container down".cyan());
            Command::new("docker")
                .args(["compose", "down"])
                .status()
                .expect("Unable to take the container down");

            println!(
                "\r\n{} {}",
                "Creating a new instance".cyan(),
                "of the container using compose".dimmed()
            );
            Command::new("docker")
                .args(["compose", "up", "-d"])
                .status()
                .expect("Unable to launch a new detached container");

            for (idx, flag) in args.enumerate() {
                let next_flag_idx = idx + 3;
                handle_flags(flag, args_for_flags.nth(next_flag_idx));
            }

            Ok(())
        }
        Commands::Invalid => panic!("Something went wrong"),
    }
}

fn handle_flags(flag: String, param: Option<String>) {
    match flag.as_str() {
        "--prisma" => {
            let timeout = param
                // a safe duration considering how long it takes to spin up a mysql docker
                .unwrap_or(String::from("15"))
                .parse::<u64>()
                .unwrap();

            println!(
                "\r\n{} {}",
                "Migrating the schema".bright_purple(),
                "and generating the types".dimmed()
            );

            // waiting for the database to load
            thread::sleep(Duration::from_secs(timeout));

            Command::new("bunx")
                .args(["prisma", "migrate", "dev"])
                .status()
                .expect("Unable to run bunx with the prisma binary");
        }
        _ => (),
    }
}

fn process_exit(message: &str) {
    println!("{}", message.red().bold());
    process::exit(1);
}

fn welcome_message() {
    let header = r#"
            . .      . .      . .      . .     . .    .       . .      .     .        
         .+'|=|`+..+'|=|`+..+'|=|`+..+'|=|`+.+'|=|`+.=|`+. .+'|=|`+..+'|     |`+.     
         |  | |  ||  | `+.||  | `+.||  | `+.|.+' |  | `+.| |  | `+.||  |     |  |     
         |  |'. '.|  |=|`. |  | .   |  |=|`.     |  |      |  |     |  |     |  |     
         |  | |  ||  | `.| `+.|=|`+.|  | `.|     |  |      |  |     |  |     |  |     
         |  | |  ||  |    ..    |  ||  |    .    |  |      |  |    .|  |    .|  |     
         |  | |  ||  | .+'||`+. |  ||  | .+'|    |  |      |  | .+'||  | .+'||  |     
         `+.| |.+'`+.|=|.+'`+.|=|.+'`+.|=|.+'    |.+'      `+.|=|.+'`+.|=|.+'|.+'     
                                                                                      
    +----------------------------------------------------------------------------------+
    "#;

    let content = format!(
        " {} \r\n {} \r\n\r\n\r\n {} \r\n\r\n {} ",
        "                 A simple CLI created to reduce the amount of repetition"
            .italic()
            .dimmed(),
        "                             on daily tasks of a developer"
            .italic()
            .dimmed(),
        "             To learn more about this binary, you can run either of the following:"
            .purple()
            .bold(),
        "                       [binary-name] help      |      [binary-name] h"
            .white()
            .italic()
    );
    let footer = format!("      .::{:.^76}::.", "");

    println!(
        "\r\n\r\n{}\r\n\r\n{}\r\n\r\n{}\r\n\r\n",
        header.bold().bright_purple(),
        content,
        footer.bold().bright_purple()
    );
    process::exit(1);
}

use std::{env, process::Command};

#[derive(Debug)]
enum Commands {
    Repo,
    Docker,
}

#[derive(Debug)]
enum Action {
    Reset,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    let current_path = args.next().expect("unable to read the current path");

    let command = args
        .next()
        .expect("command is required, use the -h flag as a guide");

    let command = match &command[..] {
        "-r" => Commands::Repo,
        "--repo" => Commands::Repo,
        "-d" => Commands::Docker,
        "--docker" => Commands::Docker,
        "-h" => {
            println!("\n\rAvailable Commands:\n\r - repo (r)\n\r - docker (d)");
            println!("\n\rAvailable Actions:\n\r - reset (r)\n\r");

            panic!("Choose one of the options above, i.e. \"repo reset\", you can also use the short version \"r r\" ");
        }
        _ => panic!("invalid command, try -h to see the available options"),
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
                panic!("Windows, really? Go download WSL...")
            } else {
                let branch = Command::new("git")
                    .args(["branch", "--show-current"])
                    .output()
                    .expect("Unable to retrieve the current git branch");
                let branch = String::from_utf8(branch.stdout).unwrap() as String;
                let branch = branch.replace("\n", "");

                Command::new("git")
                    .args(["fetch"])
                    .status()
                    .expect("Unable to fetch with git");

                Command::new("git")
                    .args(["pull", "origin", &branch])
                    .status()
                    .expect("Unable to pull with git");
            };

            Ok(())
        }
        Commands::Docker => {
            if cfg!(target_os = "windows") {
                panic!("Windows, really? Go download WSL...")
            } else {
                Command::new("docker")
                    .args(["compose", "down"])
                    .status()
                    .expect("Unable to take the container down");

                Command::new("docker")
                    .args(["compose", "up", "-d"])
                    .status()
                    .expect("Unable to launch a new detached container");

                let migration = args.next();

                if let Some(value) = migration {
                    match value.as_str() {
                        "--prisma" => {
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
    }
}

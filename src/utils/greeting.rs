use owo_colors::OwoColorize;
use std::process;

const LOGO: &str = r#"
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

pub fn welcome_message() {
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
    let footer = format!("      .::{:.^75}::.", "");

    println!(
        "\r\n\r\n{}\r\n\r\n{}\r\n\r\n{}\r\n\r\n",
        LOGO.bold().bright_purple(),
        content,
        footer.bold().bright_purple()
    );
    process::exit(1);
}

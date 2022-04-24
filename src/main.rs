use std::{thread, time,io:: Read,fs, process::{exit, Command}, env};
use colored::Colorize;
use toml;
use dirs;
use clap::StructOpt;
use ctrlc;

mod create_config;
mod cli;

fn main() {
    let args = cli::Args::parse();
    let config_dir = dirs::config_dir().unwrap();
    let app_config = config_dir.join("gamemode");
    let toml_location = &app_config.join("config.toml");
    let success_prefix = "[   OK   ] ";
    let failed_prefix = "[   FAILED   ] ";
    let wait_prefix = "[   WAIT   ] ";


    if !config_dir.exists() {
        // create config_dir
        println!("{}{}{}",wait_prefix.yellow(),"Creating config directory at ", config_dir.display());
        match std::fs::create_dir_all(&config_dir) {
            Ok(_) => println!("{}{}",success_prefix.green(),"Successfully created config directory"),
            Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to create config directory: ", e); 
            exit(1);}
        }
    } else {
        println!("{}{}{}",success_prefix.green(),"Config directory detected at ", config_dir.display());
        if !app_config.exists() {
            // create app_config
            println!("{}{}{}",wait_prefix.yellow(),"Creating app config directory at ", config_dir.join(&app_config).display());
            match std::fs::create_dir_all(config_dir.join(&app_config)) {
                Ok(_) => println!("{}{}",success_prefix.green(),"Successfully created app config directory"),
                Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to create app config directory: {}", e); 
                exit(1)}
            }
        } else {
            println!("{}{}{}",success_prefix.green(),"App config directory detected at ", config_dir.join(&app_config).display());
        }
    }
    if !toml_location.exists() {
        // create toml_location
        create_config::create(toml_location);
        println!("{}{}{}",wait_prefix.yellow(),"Creating config file at ", toml_location.display());
        exit(0);
    } 

    println!("{}{}{}",success_prefix.green(),"Config file detected at ", toml_location.display());
    // read toml_location as string
    let mut config_file = String::new();
    match fs::File::open(toml_location) {
        Ok(mut file) => {
            match file.read_to_string(&mut config_file) {
                Ok(_) => println!("{}{}",success_prefix.green(),"Successfully read config file"),
                Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to read config file: ", e);
                exit(1);}
            }
        }
        Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to read config file: ", e);
        exit(1);}
    }
    let config: toml::Value = match toml::from_str(&config_file) {
        Ok(value) => value,
        Err(e) => {
            println!("{}{}{}",failed_prefix.red(),"Failed to parse config file: ", e);
            exit(1)}
    };

    let terminal = match config.get("terminal") {
        Some(value) => match value.as_str() {
            Some(string) => string,
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: terminal is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: terminal is not set");
            exit(1)
        }
    };

    let launch_argument = match config.get("launch_argument") {
        Some(value) => match value.as_str() {
            Some(string) => string,
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: launch_argument is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: launch_argument is not set");
            exit(1)
        }
    };

    let launch_argument = if env::consts::OS == "windows" {
        "/".to_string() + launch_argument
    } else {
        "-".to_string() + launch_argument
    };

    let shell:String = match config.get("shell") {
        Some(value) => match value.as_str() {
            Some(string) => string.to_string(),
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: shell is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: shell is not set");
            exit(1)
        }
    };

    let shell_launch_argument = match config.get("shell_launch_argument") {
        Some(value) => match value.as_str() {
            Some(string) => string,
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: shell_launch_argument is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: shell_launch_argument is not set");
            exit(1)
        }
    };
    let shell_launch_argument = if env::consts::OS == "linux" {
        "-".to_string() + shell_launch_argument
    } else {
        shell_launch_argument.to_string()
    };    

    let custom_command = match config.get("custom_command") {
        Some(value) => match value.as_str() {
            Some(string) => string,
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: custom_command is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: custom_command is not set");
            exit(1)
        }
    };

    let temp_file = match config.get("temp_file") {
        Some(value) => match value.as_str() {
            Some(string) => string,
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: temp_file is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: temp_file is not set");
            exit(1)
        }
    };
    
    let execute_nbfc = match config.get("execute_nbfc") {
        Some(value) => match value.as_str() {
            Some(string) => string,
            None => {
                println!("{}{}",failed_prefix.red(),"Failed to parse config file: execute_nbfc is not a string");
                exit(1)
            }
        },
        None => {
            println!("{}{}",failed_prefix.red(),"Failed to parse config file: execute_nbfc is not set");
            exit(1)
        }
    };

    if execute_nbfc == "true" && args.nbfc != true {
        let mut _command_fan_max = Command::new(&shell)
            .arg(&shell_launch_argument)
            .arg("nbfc set -s 100")
            .spawn().expect("Error running nbfc");
    }
    if args.cmd != true {
        print!("\n{}{}",success_prefix.green(), "Command has been run | ");
    } else {
        print!("\n{}{}","[   OFF   ] ".red(), "Command is not running | ");
    }
    if args.nbfc != true {
        print!("{}{}",success_prefix.green(), "Fan speed changed | ");
    } else {
        print!("{}{}","[   OFF   ] ".red(), "Fan speed is not changed | ");
    }
    if env::consts::OS == "linux"{
        println!("{}{}",success_prefix.green(), "Keeping hard drive alive ");
    } else {
        println!("{}{}","[   OFF   ] ".red(), "Not writing to hard drive ");
    }

    if env::consts::OS == "linux" {
        let mut _command_harddisk = Command::new(&shell)
            .arg(&shell_launch_argument)
            .arg(format!("while true; do  echo a > {}; sleep 5; done",temp_file))
            .spawn().expect("Error running hard disk write");
        if args.cmd != true {
        let mut _custom_command = Command::new(terminal);
        _custom_command.arg(&launch_argument);
        _custom_command.arg(custom_command);
        match _custom_command.output() {
            Ok(output) => {
                if output.status.success() {
                    println!("{}{}",success_prefix.green(),"Reverting the settings...");
                    _command_harddisk.kill().expect("Failed to kill _command_harddisk process");
                    if execute_nbfc == "true" && args.nbfc != true {
                        let mut _command_fan_max = Command::new(shell)
                            .arg(&shell_launch_argument)
                            .arg("nbfc set -a")
                            .spawn().expect("Error running nbfc");}
                } else {
                    println!("{}{}",failed_prefix.red(),"Failed to run the command");
                    exit(1)
                }
            }
            Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to run the command: ", e);
            exit(1);}
        }
        } else {
            ctrlc::set_handler(move || {
                println!("{}{}",success_prefix.green(),"Reverting the settings...");
                let mut _command_fan_max = Command::new(&shell)
                                    .arg(&shell_launch_argument)
                                    .arg("nbfc set -a")
                                    .spawn().expect("Error running nbfc");
                _command_harddisk.kill().expect("Failed to kill _command_harddisk process");
                exit(0);
            })
            .expect("Error setting Ctrl-C handler");
            loop{
                thread::sleep(time::Duration::from_secs(600));
            }
        }
    }
    }
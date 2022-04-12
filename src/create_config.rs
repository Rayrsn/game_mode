use std::{env, path::PathBuf, io::Write, process::exit};
use colored::Colorize;

pub fn create(toml_location: &PathBuf) {
    let success_prefix = "[   OK   ] ";
    let failed_prefix = "[   FAILED   ] ";

    match std::fs::File::create(toml_location) {
        Ok(mut file) => {
            let mut config_file = String::new();
            config_file.push_str("# Game Mode Config File\n");
            config_file.push_str("# This file is used to store your game mode settings.\n");
            config_file.push_str("# You can edit this file to change the settings.\n");
            config_file.push_str("\n");
            config_file.push_str("# Terminal used to Launch the program (I suggest you use konsole since it has tab functionality, if you're on linux of course)\n");
            if env::consts::OS == "windows" {
                config_file.push_str("terminal = \"powershell\"\n");
            } else {
                config_file.push_str("terminal = \"konsole\"\n");
            }
            config_file.push_str("\n");
            config_file.push_str("# The argument used to launch a new command. (90% of the time it's 'e', but it can differ in different programs\n");
            config_file.push_str("# for example, in cmd it's 'K'\n");
            config_file.push_str("launch_argument = \"e\"\n");
            config_file.push_str("\n");
            config_file.push_str("# python script location *EDIT THIS*\n");
            config_file.push_str("python_script_location = \"\"\n");
            config_file.push_str("\n");
            config_file.push_str("# Execute NBFC? *EDIT THIS* (Note Book Fan Control (https://github.com/hirschmann/nbfc) is a software for controlling the speed of your laptop fans.)\n");
            config_file.push_str("execute_nbfc = \"false\"\n");
            config_file.push_str("\n");
            config_file.push_str("# Temp file location *EDIT THIS*. (where should the program create a temp file in order to disable hard disk sleep.) (Linux Only)\n");
            config_file.push_str("temp_file = \"\"\n");
            config_file.push_str("\n");
            config_file.push_str("# The shell used to launch the hard disk command. (Linux Only)\n");
            config_file.push_str("shell = \"bash\"\n");
            config_file.push_str("\n");
            config_file.push_str("# The argument used to launch a new command in the specified shell. (Linux Only)\n");
            config_file.push_str("shell_launch_argument = \"c\"\n");

            // write to toml_location
            match file.write(config_file.as_bytes()) {
                Ok(_) => println!("{}{}{}{}",success_prefix.green(),"Successfully created sample config file, Please check the config file at ", toml_location.to_str().unwrap().blue()," and rerun the program."),
                Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to create sample config file: ", e); 
                exit(1);}
            }
        }
        Err(e) => {println!("{}{}{}",failed_prefix.red(),"Failed to create config file: ", e); 
        exit(1);}
    }
}
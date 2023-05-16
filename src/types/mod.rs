pub mod configuration;
pub mod events;
pub mod process;
pub mod service;
pub mod status;

pub const UNIX_SOCKET_PATH: &str = "//tmp/superviseur.sock";

pub const SUPERFILE: &str = "Superfile.hcl";
pub const SUPERFILE_TOML: &str = "Superfile.toml";

pub const BANNER: &str = r#"
         _____                             _                     
        / ___/__  ______  ___  ______   __(_)_______  __  _______
        \__ \/ / / / __ \/ _ \/ ___/ | / / / ___/ _ \/ / / / ___/
       ___/ / /_/ / /_/ /  __/ /   | |/ / (__  )  __/ /_/ / /    
      /____/\__,_/ .___/\___/_/    |___/_/____/\___/\__,_/_/     
                /_/                                              
      "#;

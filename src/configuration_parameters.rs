use clap::{App, Arg};

/// Read Command Line Arguments
pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    file_path: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let file_path = matches
            .value_of("file_path")
            .expect("Error getting `file_path` value.")
            .to_string();
        ConfigurationParameters { file_path }
    }
}

impl ConfigurationParameters {
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about(
            "IPFS Smart Contract:Upload a file to IPFS and then store the CID in a smart contract.",
        )
        .version("1.0.0")
        .arg(
            Arg::with_name("file_path")
                .short("-f")
                .long("file-path")
                .value_name("File Path")
                .help("Path to the file to be uploaded.")
                .required(true),
        )
        .get_matches()
}

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    #[serde(default)]
    context: Option<String>,
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct Step {
    name: String,
    #[serde(default)]
    context: Option<String>,
    request: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Requested Feature ---");

    let file = File::open("example_configs/config1.yaml")?;
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader)?;

    println!("name: {}", config.name);
    if let Some(context) = &config.context {
        println!("context: {}", context);
    }
    println!("-----------------------------");

    for (index, step) in config.steps.iter().enumerate() {
        let step_number = index + 1;
        println!("step number: {}", step_number);
        println!("- name: {}", step.name);
        if let Some(context) = &step.context {
            println!("- context: {}", context);
        }
        println!("- request: {}", step.request);
        println!("-----------------------------");
    }

    Ok(())
}

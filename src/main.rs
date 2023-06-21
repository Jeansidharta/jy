use std::{
    error::Error,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("Could not read stdin: {}", err))?;

    let yaml_value: serde_yaml::Value =
        serde_json::from_str(&input).map_err(|err| format!("Could not parse json: {}", err))?;
    std::io::stdout()
        // Unwrap here because there should be no problem converting yaml to a string
        .write(serde_yaml::to_string(&yaml_value).unwrap().as_bytes())
        .map_err(|err| format!("Could not write to stdout : {}", err))?;

    Ok(())
}

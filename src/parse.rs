use regex::Regex;

enum Command {
    Get {key: String},
    Remove {key: String},
    Add {key: String, value: String},
}


impl Command {
    fn parse(command: String) -> Result<Self, &'static str> {
        let get_regex = Regex::new(r"^get\s+(\S+)$").unwrap();
		let set_regex = Regex::new(r#"^set\s+(\S+)\s+"(.+)"$"#).unwrap();
		let delete_regex = Regex::new(r"^delete\s+(\S+)$").unwrap();
		
		if let Some(captures) = get_regex.captures(command.as_str()) {
			return Ok(Command::Get{key: captures.get(1).unwrap().as_str().to_string()});
		}
		Err("Invalid command")
    }
}


#[cfg(test)]
mod test {
	use crate::parse::Command;

	#[test]
	fn test_parse_command() {
		let cmd = Command::parse("get some_key".to_string()).unwrap();
		match cmd {
			Command::Get{key} => assert_eq!(key, "some_key".to_string()),
			_ => {}
		};
	}
}

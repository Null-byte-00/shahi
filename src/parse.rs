use regex::Regex;

pub enum Command {
    Get {key: String},
    Delete {key: String},
    Set {key: String, value: String},
}


impl Command {
    pub fn parse(command: String) -> Result<Self, &'static str> {
        let get_regex = Regex::new(r"^get\s+(\S+)$").unwrap();
		let set_regex = Regex::new(r#"^set\s+(\S+)\s+"(.+)"$"#).unwrap();
		let delete_regex = Regex::new(r"^delete\s+(\S+)$").unwrap();
		
		if let Some(captures) = get_regex.captures(command.as_str()) {
			return Ok(Command::Get{key: captures.get(1).unwrap().as_str().to_string()});
		} else if let Some(captures) = set_regex.captures(command.as_str()) {
            return Ok(Command::Set{key: captures.get(1).unwrap().as_str().to_string(),
                                    value: captures.get(2).unwrap().as_str().to_string()});
        } else if let Some(captures) = delete_regex.captures(command.as_str()) {
            return Ok(Command::Delete{key: captures.get(1).unwrap().as_str().to_string()});
        }
		Err("Invalid command")
    }
}


#[cfg(test)]
mod test {
	use crate::parse::Command;

	#[test]
	fn test_parse_command() {
		let cmd_get = Command::parse("get some_key".to_string()).unwrap();
		match cmd_get {
			Command::Get{key} => assert_eq!(key, "some_key".to_string()),
			_ => {}
		};
        let cmd_set = Command::parse("set some_key \"some value\"".to_string()).unwrap();
        match cmd_set {
            Command::Set{key, value} => {
                assert_eq!(value, "some value".to_string());
                assert_eq!(key, "some_key".to_string());
            },
            _ => {}
        };
        let cmd_delete = Command::parse("delete some_key".to_string()).unwrap();
        match cmd_delete {
            Command::Delete{key} => assert_eq!(key, "some_key".to_string()),
            _ => {}
        };
	}
}

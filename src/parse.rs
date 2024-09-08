enum Command {
    Get {key: String},
    Remove {key: String},
    Add {key: String, value: String},
}


impl Command {
    fn parse(command: String) -> Result<Self, &'static str> {
        let GET = "get".to_string();
        let ADD = "add".to_string();

        match command {
            GET => return Ok(Command::Get {key:"Key".to_string()}),
            ADD => return Ok(Command::Remove {key:"key".to_string()}),
            _ => return Err("Invalid command")
        };
    }
}

use super::parse;
use super::core;

pub struct Storage {
    data: core::DataTable,
}


impl Storage {
    pub fn new() -> Self {
        Self {
            data: core::DataTable::new(),
        }
    }

    pub fn run_command(&mut self, command: String) -> Result<String, &'static str> {
        let parsed_command_result = parse::Command::parse(command);
        let parsed_command = match parsed_command_result {
            Ok(cmd) => cmd,
            Err(error) => return Ok("Invalid command".to_string()),
        };

        match parsed_command {
            parse::Command::Get{key} => {
                if !self.data.key_exists(key.clone()) {
                    return Ok("Key does not exist".to_string());
                }
                return Ok(self.data.get_value(key.clone()).unwrap());
            },
            parse::Command::Set{key, value} => {
                if self.data.key_exists(key.clone()) {
                    self.data.set_value(key.clone(), value.clone());
                    return Ok("Ok".to_string());
                }
                self.data.add_member(key.clone(), value.clone());
                return Ok("Ok".to_string());
            },
            parse::Command::Delete{key} => {
                if self.data.key_exists(key.clone()) {
                    self.data.remove_member(key.clone());
                    return Ok("Ok".to_string());
                }
                return Ok("Key does not exist".to_string());
            },
        };
    }

}


#[cfg(test)]
mod test {
    use crate::commands::Storage;

    #[test]
    fn test_run_command() {
        let mut storage = Storage::new();
        let result_add = storage.run_command("set key1 \"value1\"".to_string()).unwrap();
        assert_eq!(result_add, "Ok".to_string());
        let result_get = storage.run_command("get key1".to_string()).unwrap();
        assert_eq!(result_get, "value1".to_string());
        let result_delete = storage.run_command("delete key1".to_string()).unwrap();
        assert_eq!(result_delete, "Ok".to_string());
        let result_get2 = storage.run_command("get key1".to_string()).unwrap();
        assert_eq!(result_get2, "Key does not exist".to_string());
    }
}

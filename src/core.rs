
pub struct DataTable {
    keys: Vec<String>,
    values: Vec<String>,
}


impl DataTable {
    fn new() -> Self {
        Self {keys: vec![], values: vec![]}
    }

    fn key_exists(&self, key: String) -> bool {
        return self.keys.iter().any(|k| *k == key);
    }

    fn get_value(&self, key: String) -> Result<String, &'static str> {
        if !self.key_exists(key.clone()) {
            return Err("Key does not exist");
        }

        let idx: usize = self.keys.iter().position(|k| *k == key).unwrap();
        return Ok(self.values.get(idx).unwrap().clone());
    }

    fn set_value(&mut self, key: String, value: String) -> Result<(), &'static str> {
        if !self.key_exists(key.clone()) {
            return Err("Key does not exist");
        }
        
        let idx: usize = self.keys.iter().position(|k| *k == key).unwrap();
        self.values[idx] = value;
        Ok(())
    }

    fn add_member(&mut self,key: String,value: String) -> Result<(), &'static str> {
        if self.key_exists(key.clone()) {
            return Err("Key already exists");
        }
        self.keys.push(key);
        self.values.push(value);
        Ok(())
    }

    fn remove_member(&mut self ,key: String) -> Result<(),&'static str> {
        if !self.key_exists(key.clone()) {
            return Err("Key does not exist");
        }

        let idx: usize = self.keys.iter().position(|k| *k == key).unwrap();
        self.keys.remove(idx);
        Ok(())
    }
}



#[cfg(test)]
mod test {
    use crate::core::DataTable;


    #[test]
    fn test_data_table() {
        let mut data_table = DataTable {
            keys: vec!["hello".to_string(), "haha".to_string()],
            values: vec!["hohoh".to_string(), "heeheee".to_string()],
        };

        // data_table.keys.push("new_key".to_string());
        // data_table.values.push("new_value".to_string());
        data_table.add_member("new_key".to_string(), "new_value".to_string());
    }


    #[test]
    fn test_key_exists() {
        let  data_table = DataTable {
            keys: vec!["hello".to_string(), "haha".to_string()],
            values: vec!["hohoh".to_string(), "heeheee".to_string()],
        };

        assert_eq!(data_table.key_exists("hello".to_string()), true);
        assert_eq!(data_table.key_exists("non_existing_key".to_string()), false);

    }

    #[test]
    fn test_get_value() {
        let data_table = DataTable {
            keys: vec!["hello".to_string(), "haha".to_string()],
            values: vec!["hohoh".to_string(), "heeheee".to_string()],
        };
        
        let val = data_table.get_value("hello".to_string()).unwrap();
        assert_eq!(val.to_string(), "hohoh");

    }

    #[test]
    fn test_remove_value() {
        let mut data_table = DataTable {
            keys: vec!["hello".to_string(), "haha".to_string()],
            values: vec!["hohoh".to_string(), "heeheee".to_string()],
        };
        

        assert_eq!(data_table.key_exists("hello".to_string()), true);
        data_table.remove_member("hello".to_string());
        assert_eq!(data_table.key_exists("hello".to_string()), false);

    }

    #[test]
    fn test_set_value () {
        let mut data_table = DataTable::new();
        data_table.add_member("k1".to_string(), "v1".to_string());
        data_table.set_value("k1".to_string(), "v2".to_string());
        assert_eq!(data_table.get_value("k1".to_string()).unwrap(), "v2".to_string());
    }

    #[test]
    fn test_constructor() {
        let mut data_table = DataTable::new();
        data_table.add_member("first_key".to_string(), "first_value".to_string());
        assert_eq!(data_table.get_value("first_key".to_string()).unwrap(), "first_value");
    }
}

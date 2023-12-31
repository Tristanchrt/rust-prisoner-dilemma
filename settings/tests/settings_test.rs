use settings::Settings;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_key_get_configuration() {

        let mut dict = HashMap::new();
        dict.insert(String::from("KEY1"), String::from("VALUE1"));
        
        // Get key
        let value1 = Settings::get_configuration_value(&dict, "KEY1");
        assert_eq!(value1, String::from("VALUE1"));
        
        // No key
        let value2 = Settings::get_configuration_value(&dict, "KEY2");
        assert_eq!(value2, String::new());
    }   
   
    #[test]
    fn setting_creation() {

        let file_name = "./tests/fixture_settings.json";

        let settings = Settings::load(file_name);
        
        assert_eq!(settings.host, "1");
        assert_eq!(settings.port, "2");
        assert_ne!(settings.client_max, "3");
       
    }   

}
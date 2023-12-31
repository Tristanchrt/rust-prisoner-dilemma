use settings::{Log};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_show_warn() {
        let message = String::from("This is a warning message");
        Log::show("WARN", message.clone());
        // You can check the output visually in the test output.
    }

    #[test]
    fn test_log_show_debug() {
        let message = String::from("This is a debug message");
        Log::show("DEBUG", message.clone());
        // You can check the output visually in the test output.
    }

    #[test]
    fn test_log_show_info() {
        let message = String::from("This is an info message");
        Log::show("INFO", message.clone());
        // You can check the output visually in the test output.
    }

    #[test]
    fn test_log_show_error() {
        let message = String::from("This is an error message");
        Log::show("ERROR", message.clone());
        // You can check the output visually in the test output.
    }

    #[test]
    fn test_log_show_other() {
        let message = String::from("This is another message");
        Log::show("OTHER", message.clone());
        // You can check the output visually in the test output.
    }
}
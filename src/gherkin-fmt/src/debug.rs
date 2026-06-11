pub fn log(enabled: bool, message: impl AsRef<str>) {
    if enabled {
        eprintln!("DEBUG: {}", message.as_ref());
    }
}

pub fn log_rule(enabled: bool, rule_name: &str) {
    log(enabled, format!("Executing {rule_name}"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn log_rule_formats_executing_message() {
        let rule_name = "preserve lines before `Feature` block";
        let message = format!("Executing {rule_name}");

        assert_eq!(message, "Executing preserve lines before `Feature` block");
    }
}
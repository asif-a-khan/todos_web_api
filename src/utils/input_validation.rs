use validator::ValidationErrors;

// Helper function to format validation errors
pub fn handle_validation_errors(errors: ValidationErrors) -> String {
    println!("Validation errors: {:#?}", errors);
    let formatted_errors: Vec<String> = errors
        .field_errors()
        .into_iter()
        .map(|(field, errors)| {
            let error_messages: Vec<_> = errors
                .iter()
                .filter_map(|err| err.message.clone().map(|msg| msg.into_owned())) // Handle Optional message
                .collect();
            format!("{}: {}", field, error_messages.join(", "))
        })
        .collect();
    formatted_errors.join(", ")
}
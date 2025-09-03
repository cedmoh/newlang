use crate::eval::Value;

const PLACEHOLDER: &str = "{}";

/// Format strings using a template and parameters.
/// The template should contain `{}` placeholders for each parameter.
///
/// # Examples
/// ```
/// let formatted = format(vec![Value::String("Hello, {}!".to_string()), Value::String("World".to_string())]);
/// ```
pub fn format(params: Vec<Value>) -> Value {
    let mut param_iter = params.into_iter();

    let mut template = match param_iter.next() {
        Some(Value::String(template)) => template,
        Some(_) => panic!("Expected the first parameter to be a string template."),
        None => panic!("Expected template parameter."),
    };

    let params = param_iter
        .map(|p| match p {
            Value::String(s) => s.clone(),
            _ => panic!("Expected string parameters for formatting."),
        })
        .collect::<Vec<String>>();

    for param in params.iter() {
        let placeholder_index = template.find(PLACEHOLDER).unwrap();
        template.replace_range(
            placeholder_index..placeholder_index + PLACEHOLDER.len(),
            param,
        );
    }

    Value::String(template)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let formatted = format(vec![
            Value::String("Hello, {}! Today is {}.".to_string()),
            Value::String("Alice".to_string()),
            Value::String("Monday".to_string()),
        ]);
        assert_eq!(
            formatted,
            Value::String("Hello, Alice! Today is Monday.".to_string())
        );
    }

    #[test]
    #[should_panic(expected = "Expected the first parameter to be a string template.")]
    fn test_format_no_template() {
        format(vec![Value::Number(42.0)]);
    }

    #[test]
    #[should_panic(expected = "Expected template parameter.")]
    fn test_format_empty_params() {
        format(vec![]);
    }

    #[test]
    #[should_panic(expected = "Expected string parameters for formatting.")]
    fn test_format_non_string_param() {
        format(vec![
            Value::String("Hello, {}!".to_string()),
            Value::Number(42.0),
        ]);
    }
}

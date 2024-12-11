#[iccdk::query]
fn calculate(a: i32, b: i32, operator: String) -> String {
    let result = match operator.as_str() {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b), // Poprawienie mnożenia
        "/" => {
            if b != 0 {
                Some(a / b)
            } else {
                None
            }
        }
        "%" => {
            if b != 0 {
                Some(a % b)
            } else {
                None
            }
        }
        _ => None, // Poprawienie domyślnego przypadku
    };

    match result {
        Some(value) => format!("Result: {}", value),
        None => "Wrong operator or division by zero".to_string(),
    }
}

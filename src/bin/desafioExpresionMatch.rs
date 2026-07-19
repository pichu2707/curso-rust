fn describe_number(num: i32) -> String {
    // Escribe el código aquí
    match num {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        _ => "many".to_string(),
    }
}

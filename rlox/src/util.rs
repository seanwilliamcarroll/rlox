pub fn error(line: usize, message: &String) {
    report(line, &"".to_owned(), message);
}

pub fn report(line: usize, where_at: &String, message: &String) {
    println!("[line {}] Error{}: {}", line, where_at, message)
}

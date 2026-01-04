pub fn create(path: &str, _pass: &str) {
    std::fs::write(path, b"encrypted-wallet").unwrap();
}

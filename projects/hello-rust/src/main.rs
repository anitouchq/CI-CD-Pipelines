fn main() {
    println!("Hello from Rust in Docker! 🦀🐳");
    println!("This image is built and published by GitHub Actions.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn greeting_is_not_empty() {
        let greeting = "Hello from Rust in Docker!";
        assert!(!greeting.is_empty());
    }
}

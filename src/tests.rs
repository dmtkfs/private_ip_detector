#[cfg(test)]
mod tests {
    use super::*; // Import everything from `main.rs` (i.e., `is_private` function)

    #[test]
    fn test_private_ips() {
        assert!(is_private(&"192.168.1.1".parse().unwrap()));  // Should be private
        assert!(is_private(&"10.0.0.1".parse().unwrap()));     // Should be private
        assert!(is_private(&"172.16.0.1".parse().unwrap()));   // Should be private
        assert!(!is_private(&"8.8.8.8".parse().unwrap()));     // Should be public
    }
}

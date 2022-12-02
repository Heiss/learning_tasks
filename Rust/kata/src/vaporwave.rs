pub fn vaporcode(s: &str) -> String {
    s.chars()
        .filter_map(|c| match c.is_whitespace() {
            false => Some(c.to_uppercase().to_string()),
            true => None,
        })
        .collect::<Vec<String>>()
        .join("  ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            vaporcode("Lets go to the movies"),
            "L  E  T  S  G  O  T  O  T  H  E  M  O  V  I  E  S".to_string()
        );
        assert_eq!(
            vaporcode("Why isn't my code working?"),
            "W  H  Y  I  S  N  '  T  M  Y  C  O  D  E  W  O  R  K  I  N  G  ?".to_string()
        );
    }
}

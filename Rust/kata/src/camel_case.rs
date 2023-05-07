use std::sync::mpsc::sync_channel;
use std::thread;

trait ToCamelCase {
    fn to_camel_case(&self) -> String;
}

impl ToCamelCase for str {
    fn to_camel_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let (tx, rx) = sync_channel(3);

        let this = self.to_string();

        thread::spawn(move || {
            for slice in this.split(|c: char| c == '-' || c == '_') {
                tx.send(slice.to_string()).unwrap();
            }
            drop(tx);
        });

        let mut result = String::new();
        while let Ok(mut msg) = rx.recv() {
            let mut first_char = msg.remove(0);
            (!result.is_empty()).then(|| first_char.make_ascii_uppercase());
            result.push(first_char);
            result.push_str(&msg);
        }

        result
    }
}

fn to_camel_case(text: &str) -> String {
    text.to_camel_case()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("", "");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}

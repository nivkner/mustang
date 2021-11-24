use std::process::Command;

fn main() {
    match Command::new("/no-binary-by-this-name-should-exist").output() {
        Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
        Ok(output) => unreachable!("{:?}", output),
    }
}

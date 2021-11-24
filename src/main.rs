use std::process::Command;

fn main() {
    for i in 1..=25 {
        let day = format!("day{:0>2}", i);
        let cmd = Command::new("cargo")
            .args(&["run", "--bin", &day])
            .output()
            .unwrap();

        if cmd.status.success() {
            let output = String::from_utf8(cmd.stdout).unwrap();
            println!("Problem: {}\n{}", day, output);
        } else {
            println!("Problem {} failed to run.", day);
        }

    }
}

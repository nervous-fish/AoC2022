use std::process::Command;

fn main() {
    (1..25).for_each(|day| {
        println!("----------- Day {} ---------", day);

        let day = format!("day{:02}", day);

        let cmd = Command::new("cargo")
            .args(["run", "-r", "--bin", &day])
            .output()
            .unwrap();

        let output = String::from_utf8(cmd.stdout).unwrap();

        println!(
            "{}",
            if output.is_empty() {
                "not solved"
            } else {
                output.trim()
            }
        )
    });
}

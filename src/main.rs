use std::env;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sub: &str = match args.get(1) {
        Some(s) => s,
        None => {
            eprintln!("Please provide the subscription id or name as first argument");
            exit(0);
        }
    };

    let list = Command::new("az")
        .args(&[
            "group",
            "list",
            "--subscription",
            sub,
            "--query",
            "[*].name",
            "-o",
            "tsv",
        ])
        .output()
        .expect("Couldn't run list command");

    if !list.status.success() {
        eprintln!("List command failed with {:?}", list.status);
        eprintln!("stderr:\n{}", String::from_utf8_lossy(&list.stderr));
        exit(1);
    }

    let list_str = String::from_utf8_lossy(&list.stdout);

    let mut lines = list_str.lines();
    if lines.next() == None {
        println!("No resource group in this subscription");
        exit(0);
    }

    for line in list_str.lines() {
        let delete_cmd = Command::new("az")
            .args(&[
                "group",
                "delete",
                "--resource-group",
                line,
                "--no-wait",
                "-y",
            ])
            .output()
            .expect("Couldn't run delete command");
        match delete_cmd.status.success() {
            true => println!("Delete request sent for group {}", line),
            false => eprintln!(
                "Delete request not sent for group {}\nstderr:\n{}",
                line,
                String::from_utf8_lossy(&delete_cmd.stderr)
            ),
        }
    }
}

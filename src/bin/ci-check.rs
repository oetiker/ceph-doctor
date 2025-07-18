use std::process::{exit, Command};

fn main() {
    println!("🔍 Running CI checks locally...\n");

    let checks = vec![
        (
            "Format Check",
            "cargo",
            vec!["fmt", "--all", "--", "--check"],
        ),
        (
            "Clippy Check",
            "cargo",
            vec![
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        (
            "Test Check",
            "cargo",
            vec!["test", "--all-targets", "--all-features"],
        ),
    ];

    let mut failed = false;

    for (name, cmd, args) in checks {
        print!("⏳ Running {name}... ");

        let output = Command::new(cmd)
            .args(&args)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("✅ PASSED");
        } else {
            println!("❌ FAILED");
            println!("Error output:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            failed = true;
        }
    }

    if failed {
        println!("\n❌ Some checks failed. Please fix the issues above.");
        exit(1);
    } else {
        println!("\n🎉 All checks passed! Ready for CI.");
    }
}

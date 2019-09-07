use colored::*;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::{Command, ExitStatus};
use std::time::Duration;
use toml::Value;
use wait_timeout::ChildExt;

const TIMEOUT: Duration = Duration::from_secs(60);

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    eprint!("{}", opts.usage(&brief));
}

enum TestError {
    Failed,
    Timeout,
    CompilationFailed,
}

/// Execute a command with specifics arguments
fn exec_command(cmd: &str, args: &[&str]) -> ExitStatus {
    let mut cmd = Command::new(cmd);
    cmd.args(args);
    println!("{} {:?}", "EXECUTING".blue().bold(), cmd);
    cmd.status().expect("failed to execute process")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("g", "graphical", "launch qemu with console");
    opts.optflag("", "nocapture", "show output even if test succeed");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, opts);
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let tests: Vec<String> = {
        let mut file = File::open("./Cargo.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let value = contents.parse::<Value>().unwrap();
        let btree = match &value["features"] {
            Value::Table(btree) => btree,
            _ => panic!("not a btree"),
        };
        let all_tests: Vec<String> = btree
            .into_iter()
            .filter_map(|f| {
                if f.0.starts_with("test-") || f.0.starts_with("native-test-") {
                    Some(f.0.clone())
                } else {
                    None
                }
            })
            .collect();
        if !matches.free.is_empty() {
            for test in matches.free.iter() {
                if !all_tests.iter().find(|&x| x == test).is_some() {
                    eprintln!("invalid test name{}", test);
                    eprintln!("possible tests are: {:?}", all_tests);
                    std::process::exit(1);
                }
            }
            matches.free.clone()
        } else {
            all_tests
        }
    };
    println!("running {} tests", tests.len());
    let all_result: Vec<Result<(), TestError>> = tests
        .iter()
        .map(|feature| {
            let native = if feature.starts_with("native-test-") {
                true
            } else {
                false
            };
            println!(
                "test: {} native_mode: {}",
                (*feature).clone().magenta().bold(),
                native
            );

            let exit_status = exec_command(
                "make",
                &[
                    "-C",
                    if native { "../" } else { "./" },
                    "DEBUG=yes",
                    &format!(
                        "cargo_flags=--features {},test,{}",
                        feature,
                        if matches.opt_present("g") {
                            ""
                        } else {
                            "serial-eprintln,exit-on-panic"
                        }
                    ),
                ],
            );
            if !exit_status.success() {
                println!("{}", "Compilation Failed".red().bold());
                return Err(TestError::CompilationFailed);
            }

            if native && feature.contains("hard-drive") {
                // Compiling generate C programm
                exec_command(
                    "gcc",
                    &["src/tests/generate.c", "-o", "generate", "--verbose"],
                );

                // Generating a Rainbow disk of 16mo
                exec_command("./generate", &["../rainbow_disk.img", "16777216"]);

                // Clean executable
                exec_command("rm", &["generate", "-v"]);
            }

            let output_file = format!(
                "{}/test-output/{}",
                env!("PWD"),
                format!("{}-output", feature)
            );
            let mut child = {
                let mut qemu_command = Command::new("qemu-system-x86_64");
                qemu_command
                    .args(&["--enable-kvm", "-cpu", "IvyBridge", "-m", "256"])
                    .args(&["-serial", &format!("file:{}", output_file)])
                    .args(&["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"])
                    .args(if matches.opt_present("g") {
                        [].iter()
                    } else {
                        ["-display", "none"].iter()
                    });

                match native {
                    true => {
                        if feature.contains("hard-drive") {
                            qemu_command
                                .args(&["-drive", "format=raw,file=../image_disk.img"])
                                .args(&["-drive", "format=raw,file=../rainbow_disk.img"])
                        } else {
                            qemu_command.args(&["-drive", "format=raw,file=../image_disk.img"])
                        }
                    }
                    false => qemu_command.args(&["-kernel", "build/kernel.elf"]),
                };
                println!("{}: {:?}", "EXECUTING".blue().bold(), qemu_command);
                qemu_command.spawn().expect("failed to execute process")
            };

            let show_output = || {
                let mut output = String::new();
                File::open(output_file)
                    .unwrap()
                    .read_to_string(&mut output)
                    .unwrap();
                println!("{}: {}", "OUTPUT".blue().bold(), output);
            };

            match child.wait_timeout(TIMEOUT) {
                Err(e) => panic!("Internal error: {}", e),
                Ok(Some(exit_status)) => {
                    let exit_status = exit_status.code().unwrap() >> 1;
                    if exit_status != 0 {
                        show_output();
                        println!("{}", "Failed".red().bold());
                        Err(TestError::Failed)
                    } else {
                        if matches.opt_present("nocapture") {
                            show_output();
                        }
                        println!("{}", "Ok".green().bold());
                        Ok(())
                    }
                }
                Ok(None) => {
                    child.kill().expect("cant kill");
                    show_output();
                    println!("{}", "TIMEOUT".red().bold());
                    Err(TestError::Timeout)
                }
            }
        })
        .collect();
    let total_succeed = all_result.iter().filter(|r| r.is_ok()).count();
    let total_failed = all_result.iter().filter(|r| r.is_err()).count();
    println!(
        "test result: {} {} passed; {} failed",
        if total_succeed == tests.len() {
            "SUCCEED".green().bold()
        } else {
            "FAILED".red().bold()
        },
        total_succeed,
        total_failed
    );
}

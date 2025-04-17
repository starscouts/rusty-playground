use std::collections::HashMap;
use std::env;
use rustfetch::colors::*;
use sysinfo::{System, SystemExt, UserExt, ProcessExt, CpuExt, Pid, Process, ProcessStatus};

fn convert_seconds(seconds: &u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;
    let mut text = String::from("");

    if days > 0 {
        text += &*(match days {
            1 => days.to_string() + " day, ",
            _ => days.to_string() + " days, "
        });
    }

    if hours > 0 {
        text += &*(match hours {
            1 => hours.to_string() + " hour, ",
            _ => hours.to_string() + " hours, "
        });
    }

    if minutes > 0 {
        text += &*(match minutes {
            1 => minutes.to_string() + " minute, ",
            _ => minutes.to_string() + " minutes, "
        });
    }

    text
        .strip_suffix(", ")
        .unwrap().to_owned()
}

fn format_size(size: u64) -> String {
    // Note: this is how I've been doing it in JS and PHP,
    // it can probably be done differently in Rust.
    if size > 1048576 {
        format!("{} MiB", (size as f64 / 1048576.0).round())
    } else if size > 1024 {
        format!("{} KiB", (size as f64 / 1024.0).round())
    } else {
        format!("{} B", size)
    }
}

fn get_version_string(shell_path: &str) -> Option<String> {
    let output = std::process::Command::new(shell_path)
        .arg("--version")
        .output();
    String::from_utf8(output.ok().unwrap().stdout)
        .ok()
}

fn get_matching_processes(processes: &HashMap<Pid, Process>, status: ProcessStatus, name: &str) -> String {
    let count = processes.iter()
        .map(|process| process.1.status())
        .filter(|status_code| status_code.to_owned() == status)
        .count();

    if count > 0 {
        format!("{count} {name}, ")
    } else {
        "".to_owned()
    }
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let computer_name = sys.host_name()
        .unwrap_or("(unknown)".to_owned());

    let user_name = sysinfo::get_current_pid()
        .ok()
        .and_then(|pid| sys.process(pid))
        .and_then(|process| process.user_id())
        .and_then(|user_id| sys.get_user_by_id(user_id))
        .map(|user| user.name())
        .unwrap_or("(unknown)");

    let os_name = match env::consts::OS {
        "linux" => "Linux",
        "macos" => "macOS",
        "ios" => "iOS",
        "freebsd" => "FreeBSD",
        "dragonfly" => "DragonflyBSD",
        "netbsd" => "NetBSD",
        "openbsd" => "OpenBSD",
        "solaris" => "Solaris the very cursed OS",
        "android" => "Android",
        "windows" => "Windows but you are going to install Linux right now",
        _ => env::consts::OS
    };

    let os_version = sys.os_version()
        .unwrap_or("".to_owned());

    let os_architecture = match env::consts::ARCH {
        "aarch64" => "arm64",
        _ => env::consts::ARCH
    };

    let kernel_name = sys.name()
        .unwrap_or(os_name.to_owned());

    let kernel_version = sys.kernel_version()
        .unwrap_or(os_version.clone());

    let uptime = convert_seconds(&sys.uptime());

    let shell = env::var("SHELL")
        .ok()
        .and_then(|shell_path| get_version_string(&shell_path))
        .map(|version| version.trim().to_owned())
        .unwrap_or("Unknown".to_owned());

    let cpu = sys.cpus()[0].brand();

    let process_list = sys.processes();
    let processes = format!("{}{}{}{}{}{}{}{}{}{}{}",
        get_matching_processes(process_list, ProcessStatus::Run, "running"),
        get_matching_processes(process_list, ProcessStatus::Idle, "idle"),
        get_matching_processes(process_list, ProcessStatus::Sleep, "in sleep"),
        get_matching_processes(process_list, ProcessStatus::Stop, "stopped"),
        get_matching_processes(process_list, ProcessStatus::Zombie, "zombie"),
        get_matching_processes(process_list, ProcessStatus::Tracing, "tracing"),
        get_matching_processes(process_list, ProcessStatus::Dead, "dead"),
        get_matching_processes(process_list, ProcessStatus::Wakekill, "in wake kill"),
        get_matching_processes(process_list, ProcessStatus::Parked, "parked"),
        get_matching_processes(process_list, ProcessStatus::LockBlocked, "blocked on lock"),
        get_matching_processes(process_list, ProcessStatus::UninterruptibleDiskSleep, "in disk sleep")
    )
        .strip_suffix(", ")
        .unwrap().to_owned();

    let memory = format!("{}/{}", format_size(sys.used_memory()), format_size(sys.total_memory()));

    println!("{bold}{blue}{user_name}@{computer_name}{reset}",
        bold = FORMAT_BOLD,
        blue = COLOR_FG_BLUE,
        reset = FORMAT_RESET);
    println!("{blue}OS{reset}: {os_name} {os_version} {os_architecture}",
         blue = COLOR_FG_BLUE,
         reset = FORMAT_RESET);
    println!("{blue}Kernel{reset}: {kernel_name} {kernel_version}",
         blue = COLOR_FG_BLUE,
         reset = FORMAT_RESET);
    println!("{blue}Uptime{reset}: {uptime}",
         blue = COLOR_FG_BLUE,
         reset = FORMAT_RESET);
    println!("{blue}Processes{reset}: {processes}",
             blue = COLOR_FG_BLUE,
             reset = FORMAT_RESET);
    println!("{blue}Shell{reset}: {shell}",
         blue = COLOR_FG_BLUE,
         reset = FORMAT_RESET);
    println!("{blue}CPU{reset}: {cpu}",
         blue = COLOR_FG_BLUE,
         reset = FORMAT_RESET);
    println!("{blue}Memory{reset}: {memory}",
         blue = COLOR_FG_BLUE,
         reset = FORMAT_RESET);
}

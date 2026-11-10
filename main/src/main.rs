use std::process::Command;
use std::thread;
use std::time;

fn main() {
    let device = "Keychron  Keychron Link "; // YES THE SPACE AT THE END MATTERS I DONT KNOW WHY ITS DUMB
    let preset = "DVN";

    let sleep_time = time::Duration::from_secs(15);
    let windows_to_search_for: [&str; 2] = ["Sober", "Roblox"];

    loop {
        thread::sleep(sleep_time);
        if roblox_open(&windows_to_search_for) {
            enable_preset(device, preset);
        } else {
            continue;
        }

        while roblox_open(&windows_to_search_for) {
            thread::sleep(sleep_time);
        }

        if !roblox_open(&windows_to_search_for) {
            disable_control(device);
        }
    }
}

// Gets the window ID's of all currently opened windows
fn get_window_ids() -> Vec<String> {
    let command_output = Command::new("kdotool").arg("search").output().unwrap();

    let output = String::from_utf8(command_output.stdout).expect("String was not a valid UTF 8");

    output.lines().map(String::from).collect()
}

fn enable_preset(device: &str, preset: &str) {
    let _ = Command::new("input-remapper-control")
        .arg("--command")
        .arg("start")
        .arg("--device")
        .arg(device)
        .arg("--preset")
        .arg(preset)
        .output();
    println!("The preset {} has been enabled on device {}", preset, device);
}

fn disable_control(device: &str) {
    let _ = Command::new("input-remapper-control")
        .arg("--command")
        .arg("stop")
        .arg("--device")
        .arg(device)
        .output();
    println!("Stopped all control of device {}", device);
}

fn roblox_open(windows_to_search_for: & [&str; 2]) -> bool {
    let window_ids = get_window_ids();

    for id in window_ids {
        let window_name = String::from_utf8(
            Command::new("kdotool")
                .arg("getwindowname")
                .arg(id)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        if windows_to_search_for.contains(&window_name.as_str()) {
            println!("{} was open", window_name);
            return true;
        }
    }

    false
}

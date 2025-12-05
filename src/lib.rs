use std::fs;
use std::path::Path;
use std::sync::OnceLock;

static INPUT: OnceLock<String> = OnceLock::new();

fn curl_input(day: u32) -> String {
    let cookie_value = fs::read_to_string("aoc_session_cookie")
        .expect("No session cookie file \"aoc_session_cookie\"");
    let output = std::process::Command::new("curl")
        .args([
            "-H".to_string(),
            format!("Cookie: {}", cookie_value.trim()),
            format!("https://adventofcode.com/2025/day/{}/input", day),
        ])
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("{}\n", String::from_utf8(output.stderr).unwrap())
    }
    String::from_utf8(output.stdout).unwrap()
}

pub fn input(day: u32) -> &'static str {
    let path = format!("input/{:02}", day);
    fs::create_dir_all(Path::new(&path).parent().unwrap()).expect("Cannot create input directory.");
    INPUT
        .get_or_init(|| {
            fs::read_to_string(&path).unwrap_or_else(|_| {
                let input = curl_input(day);
                fs::write(&path, &input).unwrap();
                input
            })
        })
        .as_str()
}

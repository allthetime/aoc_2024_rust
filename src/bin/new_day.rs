use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: new_day <day_number>");
        return;
    }

    let day_number = &args[1];
    let folder_name = format!("src/day{:0>3}", day_number);

    // Check if the directory already exists
    if fs::metadata(&folder_name).is_ok() {
        eprintln!("Directory {} already exists", folder_name);
        return;
    }

    if let Err(e) = fs::create_dir(&folder_name) {
        eprintln!("Failed to create directory {}: {}", folder_name, e);
        return;
    }

    // Create mod.rs file from template
    let template_path = "src/templates/mod.rs";
    let new_mod_path = format!("{}/mod.rs", folder_name);
    if let Err(e) = fs::copy(template_path, &new_mod_path) {
        eprintln!("Failed to copy template to {}: {}", new_mod_path, e);
        return;
    }

    // Replace placeholders in mod.rs
    let mut mod_content = fs::read_to_string(&new_mod_path).expect("Failed to read mod.rs");
    mod_content = mod_content.replace("{DAY_NUMBER}", day_number);
    mod_content = mod_content.replace("{ DAY_NUMBER }", day_number);
    fs::write(&new_mod_path, mod_content).expect("Failed to write mod.rs");

    // Run get_data binary
    let status = Command::new("cargo")
        .args(&["run", "--bin", "get_data", "--", day_number])
        .status()
        .expect("Failed to run get_data binary");

    if !status.success() {
        eprintln!("get_data binary failed");
    }

    // Add the new day to main.rs
    let main_rs_path = "src/main.rs";
    let mut main_rs_content = fs::read_to_string(main_rs_path).expect("Failed to read main.rs");
    let new_day_mod = format!("mod day{:0>3};", day_number);
    let new_day_run = format!("day{:0>3}::solve,", day_number);

    if !main_rs_content.contains(&new_day_mod) {
        main_rs_content = main_rs_content.replace(
            "// {NEW_DAY_MOD}",
            &format!("{}\n// {{NEW_DAY_MOD}}", new_day_mod),
        );
    }

    if !main_rs_content.contains(&new_day_run) {
        main_rs_content = main_rs_content.replace(
            "        // {NEW_DAY_RUN}",
            &format!("{}\n        // {{NEW_DAY_RUN}}", new_day_run),
        );
    }

    fs::write(main_rs_path, main_rs_content).expect("Failed to write main.rs");
}

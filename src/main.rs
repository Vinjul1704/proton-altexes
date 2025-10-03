use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::fs::*;
use std::path::Path;

use eframe::egui;


fn main() -> eframe::Result {
    
    let args: Vec<String> = env::args().collect();

    // Argument after "waitforexeandrun" is the EXE path
    let exe_index = args.clone().into_iter().position(|arg| arg == "waitforexitandrun").expect("Error parsing %command%") + 1;

    // Construct config path for EXE
    let config_path_str = format!("{}.altexes", args[exe_index].clone());
    let config_path = Path::new(&config_path_str);

    // Read alternative EXEs from config
    // Vector of path as String, and bool to indicate if the file exists
    let mut altexes: Vec<(String, bool)> = Vec::new();
    if config_path.exists() {
        for (_line_index, line) in read_to_string(config_path).unwrap().lines().enumerate() {
            if !line.is_empty() {
                let exists = Path::new(&line).exists();
                altexes.push((line.to_string(), exists));
            }
        }
    }


    // GUI
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_simple_native("proton-altexes", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Proton alternative EXE launcher");
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Run Default").clicked() {
                    // Reconstruct launch command
                    let launch_command = args[1].clone();
                    let mut launch_args: Vec<String> = Vec::new();

                    for (arg_index, arg) in args.clone().into_iter().enumerate() {
                        if arg_index < 2 {
                            continue;
                        }

                        launch_args.push(arg);
                    }

                    // Run command
                    Command::new(launch_command).args(launch_args).process_group(0).spawn().expect("Failed to run command.");

                    // Close GUI
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.button("Add EXE").clicked() && let Some(path) = rfd::FileDialog::new().pick_file() {
                    let picked_path = Some(path.display().to_string()).expect("Error picking path");
                    let _ = (&mut altexes).push((picked_path, true));
                    write_config(Path::new(&config_path_str), altexes.clone());
                }
            });
            ui.separator();
            for (altexe_index, altexe) in altexes.clone().into_iter().enumerate() {
                ui.horizontal(|ui| {
                    if altexe.clone().1 {
                        if ui.button("Run").clicked() {
                            // Get some args and paths
                            let launch_command = args[1].clone();
                            let mut launch_args: Vec<String> = Vec::new();

                            let exe_path = altexe.clone().0;
                            let exe_dir = Path::new(&exe_path).parent().expect("Couldn't get EXE dir");

                            let orig_exe_path = &args.clone()[exe_index];
                            let orig_exe_dir = Path::new(&orig_exe_path).parent().expect("Couldn't get EXE dir");

                            // Fix up LD_LIBRARY_PATH
                            if let Some(path_env) = env::var_os("LD_LIBRARY_PATH") {
                                let mut paths = env::split_paths(&path_env).collect::<Vec<_>>();
                                if let Some(path_index) = paths.iter().position(|x| x == orig_exe_dir) {
                                    paths.remove(path_index);
                                }
                                paths.push(exe_dir.to_path_buf());
                                let path_env_new = env::join_paths(paths).expect("Failed to construct LD_LIBRARY_PATH");
                                unsafe { env::set_var("LD_LIBRARY_PATH", &path_env_new); }
                            }

                            // Fix up launch command
                            for (arg_index, arg) in args.clone().into_iter().enumerate() {
                                if arg_index < 2 {
                                    continue;
                                }

                                if arg_index == exe_index {
                                    launch_args.push(exe_path.clone());
                                } else {
                                    launch_args.push(arg);
                                }
                            }

                            // Run command
                            Command::new(launch_command).args(launch_args).current_dir(exe_dir).process_group(0).spawn().expect("Failed to run command.");

                            // Close GUI
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                    else
                    {
                        ui.add_enabled(false, egui::Button::new("Run"));
                    }
                    if ui.button("Remove").clicked() {
                        (&mut altexes).remove(altexe_index);
                        write_config(Path::new(&config_path_str), altexes.clone());
                    }
                    ui.label(altexe.clone().0);
                });
            }
        });
    })
}


fn write_config(config_path: &Path, altexes: Vec<(String, bool)>) {
    let mut config_content = String::new();

    for altexe in altexes {
        config_content.push_str(altexe.0.as_str());
        config_content.push('\n');
    }

    write(config_path, config_content).expect("Couldn't write config");
}
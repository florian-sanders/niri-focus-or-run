use niri_ipc::{Action, Request, Window, socket::Socket};
use std::{
    env::args,
    error::Error,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    let program = args().nth(1).ok_or("Please provide a program name")?;
    let app_id = args().nth(2);
    let mut socket = Socket::connect()?;
    let app_id_to_focus = match &app_id {
        Some(app_id) => &app_id,
        None => &program,
    };
    let matching_windows = find_windows(&mut socket, &app_id_to_focus)?;

    if !matching_windows.is_empty() {
        focus_next_window(&mut socket, &matching_windows)?;
    } else {
        Command::new(&program)
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    Ok(())
}

fn find_windows(socket: &mut Socket, app_id_to_focus: &str) -> Result<Vec<Window>, Box<dyn Error>> {
    let reply = socket.send(Request::Windows)??;
    let windows = match reply {
        niri_ipc::Response::Windows(windows) => windows,
        _ => return Err("Niri failed to provide the window list".into()),
    };
    let matching_windows: Vec<Window> = windows
        .into_iter()
        .filter(|window| {
            window
                .app_id
                .as_deref()
                .is_some_and(|app_id| app_id.contains(app_id_to_focus))
        })
        .collect();
    Ok(matching_windows)
}

fn focus_next_window(socket: &mut Socket, windows: &[Window]) -> Result<(), Box<dyn Error>> {
    let focused_window_position = windows.iter().position(|window| window.is_focused);
    let index_to_focus = match focused_window_position {
        Some(index) => (index + 1) % windows.len(),
        None => 0,
    };
    let window_to_focus = windows
        .get(index_to_focus)
        .ok_or("Failed to find window to focus")?;

    socket.send(Request::Action(Action::FocusWindow {
        id: window_to_focus.id,
    }))??;

    Ok(())
}

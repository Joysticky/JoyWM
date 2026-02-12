/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![allow(irrefutable_let_patterns)]

mod handlers;

mod grabs;
mod input;
mod state;
mod winit;

use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
pub use state::JoyWM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let mut event_loop: EventLoop<JoyWM> = EventLoop::try_new()?;

    let display: Display<JoyWM> = Display::new()?;

    let mut state = JoyWM::new(&mut event_loop, display);

    // Open a Wayland/X11 window for our nested compositor
    crate::winit::init_winit(&mut event_loop, &mut state)?;

    // Set WAYLAND_DISPLAY to our socket name, so child processes connect to JoyWM rather
    // than the host compositor
    std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);

    // Spawn a test client, that will run under JoyWM
    spawn_client();

    event_loop.run(None, &mut state, move |_| {
        // JoyWM is running
    })?;

    Ok(())
}

fn init_logging() {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }
}

fn spawn_client() {
    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let arg = args.next();

    match (flag.as_deref(), arg) {
        (Some("-c") | Some("--command"), Some(command)) => {
            std::process::Command::new(command).spawn().ok();
        }
        _ => {
            std::process::Command::new("weston-terminal").spawn().ok();
        }
    }
}

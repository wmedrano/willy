use std::{
    ffi::{c_void, CStr},
    io::{stdout, Stdout},
};

use anyhow::Result;
use app::App;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use flashkick::Scm;
use ratatui::prelude::CrosstermBackend;
use scm_obj_cache::ScmObjCache;
use terminal_backend::{event_to_scm, iter_crossterm_events};

pub mod app;
pub mod buffer;
pub mod scm_obj_cache;
pub mod terminal_backend;
pub mod widgets;

fn main() -> Result<()> {
    // Boot Guile by loading scheme/main.scm.
    let mut args: Vec<*const i8> = vec![
        CStr::from_bytes_with_nul(b"-l\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"scheme/main.scm\0")
            .unwrap()
            .as_ptr(),
    ];
    unsafe {
        flashkick::ffi::scm_boot_guile(
            args.len() as i32,
            args.as_mut_ptr() as *mut *mut i8,
            Some(inner_main),
            std::ptr::null_mut(),
        );
    }
    Ok(())
}

pub extern "C" fn inner_main(_: *mut c_void, argc: i32, argv: *mut *mut i8) {
    unsafe {
        flashkick::ffi::scm_c_define_gsubr(
            CStr::from_bytes_with_nul(b"run-willy\0").unwrap().as_ptr(),
            1,
            0,
            0,
            scm_run_willy as _,
        );
        flashkick::ffi::scm_shell(argc, argv);
    }
}

extern "C" fn scm_run_willy(event_handler: flashkick::Scm) -> flashkick::Scm {
    match run_willy(event_handler) {
        Ok(()) => flashkick::Scm::EOL,
        Err(err) => unsafe {
            let err_sym = ScmObjCache::singleton().symbols.error;
            let msg = Scm::new_string(&err.to_string());
            let args = Scm::with_reversed_list(std::iter::once(msg));
            flashkick::ffi::scm_throw(err_sym.0, args.0);
        },
    }
}

fn run_willy(event_handler: flashkick::Scm) -> Result<()> {
    // Setup
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // Run
    let result = run_willy_with_terminal(CrosstermBackend::new(stdout()), event_handler);

    // Cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    result
}

fn run_willy_with_terminal(
    terminal: CrosstermBackend<Stdout>,
    event_handler: flashkick::Scm,
) -> Result<()> {
    let mut willy = App::new(terminal)?;
    loop {
        willy.render()?;
        let events = iter_crossterm_events().inspect(|e| unsafe {
            if let Ok(e) = e {
                let scm_event = event_to_scm(e);
                flashkick::ffi::scm_call_1(event_handler.0, scm_event.0);
            }
        });
        match willy.handle_events(events)? {
            app::AppControlState::Continue => (),
            app::AppControlState::Exit => return Ok(()),
        };
    }
}

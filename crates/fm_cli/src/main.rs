use std::{io, thread, time::Duration};

use crossbeam_channel::{unbounded, Receiver, Sender};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fm_core::{apply_event, reduce, Action, Effect, Event, State};
use ratatui::{backend::CrosstermBackend, Terminal};

fn spawn_worker(effect_rx: Receiver<Effect>, event_tx: Sender<Event>) {
    thread::spawn(move || {
        while let Ok(effect) = effect_rx.recv() {
            let ev = match effect {
                Effect::ListDir { path, show_hidden, sort } => {
                    match fm_core::fs_ops::list_dir(&path, show_hidden, sort) {
                        Ok(entries) => Event::Listed { cwd: path, entries },
                        Err(e) => Event::Error { message: format!("list_dir failed: {e}") },
                    }
                }
                Effect::DeletePath { path } => match fm_core::fs_ops::delete_path(&path) {
                    Ok(_) => Event::Deleted { path },
                    Err(e) => Event::Error { message: format!("delete failed: {e}") },
                },
                Effect::OpenPath { path } => {
                    fm_platform::open::open_path(&path);
                    Event::Opened { path }
                }
            };
            let _ = event_tx.send(ev);
        }
    });
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cwd = std::env::current_dir()?;
    let mut state = State::new(cwd);

    let (effect_tx, effect_rx) = unbounded::<Effect>();
    let (event_tx, event_rx) = unbounded::<Event>();

    spawn_worker(effect_rx, event_tx);

    // initial load
    for eff in reduce(&mut state, Action::Refresh) {
        let _ = effect_tx.send(eff);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tick = Duration::from_millis(50);

    loop {
        // drain events
        while let Ok(ev) = event_rx.try_recv() {
            apply_event(&mut state, ev);
            // after delete, refresh
            // (cheap: always refresh on Deleted)
            // we can detect by status prefix
            if state.status.starts_with("Deleted:") {
                for eff in reduce(&mut state, Action::Refresh) {
                    let _ = effect_tx.send(eff);
                }
            }
        }

        terminal.draw(|f| fm_tui::ui::draw(f, &state))?;

        if state.should_quit {
            break;
        }

        if let Some(action) = fm_tui::input::read_action(state.mode, tick)? {
            let effects = reduce(&mut state, action);
            for eff in effects {
                let _ = effect_tx.send(eff);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

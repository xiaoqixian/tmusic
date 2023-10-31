// Date: Sun Oct 22 10:03:51 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    layout::{Constraint, Rect},
    backend::{Backend, CrosstermBackend},
    Terminal,
};


use self::{progress_bar::ProgressBar, white_panel::WhitePanel};

use super::playback::PlayerError;

mod component;
mod app;
mod search_box;
mod progress_bar;
mod naked_nested;
mod nested;
mod block;
mod white_panel;
//mod single_widget;
//mod time_sensitive;

use component::{CompState, Component};
use search_box::SearchBox;

#[derive(Debug)]
enum Error {
    PlayerError(PlayerError),
    IOError(std::io::Error)
}

pub fn run() {
    let _ = enable_raw_mode().expect("enable_raw_mode failed");
    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).expect("create terminal failed");

    let res = inner_run(&mut terminal);

    disable_raw_mode().expect("disable_raw_mode failed");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).expect("leave execution failed");
    terminal.show_cursor().expect("show cursor failed");

    if let Err(e) = res {
        panic!("error: {:?}", e);
    }

}

fn inner_run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Error> {
    let size = match terminal.size() {
        Err(e) => return Err(Error::IOError(e)),
        Ok(s) => s
    };

    let mut app = app::new();
    let sb = SearchBox::new(Constraint::Length(3))
        .block_with_title(String::from("搜索栏"));
    //let pb = ProgressBar::new(Constraint::Length(3));
    let pb = WhitePanel::new(Constraint::Length(3)).block();
    let mut panel = naked_nested::NakedNested::new(Constraint::Min(3))
        .direction(tui::layout::Direction::Horizontal);

    let wp1 = WhitePanel::new(Constraint::Percentage(30))
        .block();
    let wp2 = WhitePanel::new(Constraint::Percentage(70))
        .block();

    panel.registrate(wp1);
    panel.registrate(wp2);

    app.registrate(sb);
    app.registrate(pb);
    app.registrate(panel);
    app.set_area(terminal.size().unwrap());
    app.alter_mode(component::CompMode::Enter);
    
    'run: loop {
        app.render(terminal.current_buffer_mut());
        let min_update_duration = app.update_duration()
            .unwrap_or(std::time::Duration::from_secs(100));

        if let Err(e) = terminal.draw(|_| {}) {
            return Err(Error::IOError(e));
        }

        match event::poll(min_update_duration) {
            Ok(_) => match event::read() {
                Err(e) => return Err(Error::IOError(e)),
                Ok(ev) => {
                    if let Event::Resize(width, height) = ev {
                        let _ = terminal.resize(Rect::new(0, 0, width, height));
                        app.set_area(terminal.size().unwrap());
                        continue 'run;
                    }

                    match app.feed_event(ev) {
                        CompState::Exit => break 'run,
                        _ => {}
                    }
                }
            },
            Err(e) => return Err(Error::IOError(e))
        }
    }
    Ok(())
}


#[test]
fn test_ui() {
    run();
}

//#[test]
//fn debug_test() {
    //let stdout = io::stdout();
    //let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).expect("create terminal failed");

    //let _ = inner_run(&mut terminal);
//}

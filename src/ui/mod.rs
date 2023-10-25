// Date: Sun Oct 22 10:03:51 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::io;
use std::rc::Rc;

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


use super::playback::PlayerError;

//mod popup;
//mod nested_layout;
mod component;
//mod app;
//mod search_box;
//mod progress_bar;
//mod jumper;
mod nested;
mod enterable;

//use app::App;
//use search_box::SearchBox;
//use component::{CompMode, Component};

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
}

fn inner_run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Error> {
    let size = match terminal.size() {
        Err(e) => return Err(Error::IOError(e)),
        Ok(s) => s
    };

    //let mut app = App::<Rc<dyn Component>>::new();
    //let sb = Rc::new(SearchBox::new(Constraint::Min(3)));
    //app.registrate(sb);
    
    //app.init(size);

    'run: loop {
        //app.set_area(terminal.size().unwrap());
        //app.render(terminal.current_buffer_mut());
        if let Err(e) = terminal.draw(|_| {}) {
            return Err(Error::IOError(e));
        }

        match event::read() {
            Err(e) => return Err(Error::IOError(e)),
            Ok(ev) => {
                if let Event::Resize(width, height) = ev {
                    let _ = terminal.resize(Rect::new(0, 0, width, height));
                }

                //match app.read_event(ev) {
                    //CompMode::Exit => break 'run,
                    //_ => {}
                //}
            }
        }
    }
        //match event::read() {
            //Err(e) => return Err(Error::IOError(e)),
            //Ok(ev) => match app.read_event(ev) {
                //CompMode::Exit => break 'run,
                //_ => {}
            //}
        //}
    //}

    Ok(())
}


#[test]
fn test_ui() {
    run();
    //let stdout = io::stdout();
    //let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).expect("create terminal failed");

    //let _ = inner_run(&mut terminal);
}

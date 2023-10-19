// Date: Sat Oct 14 23:20:45 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    boxed::Box, 
    collections::HashMap,
    io,
    time::{Duration, Instant}
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph, Borders, BorderType, Gauge, List, ListItem},
    text, Terminal,
};



enum NameOrLayout {
    Name(&'static str),
    Layout(Box<NestedLayout>)
}

struct NestedLayout {
    layout: Layout,
    inner: Vec<NameOrLayout>
}

impl NestedLayout {
    fn new(layout: Layout, inner: Vec<NameOrLayout>) -> Self {
        Self {
            layout,
            inner
        }
    }
    fn run(nested: &Self, rect: Rect, map: &mut HashMap<&'static str, Rect>) {
        let chunks = nested.layout.split(rect);
        nested.inner.iter().enumerate()
            .for_each(|(index, item)| {
                match item {
                    NameOrLayout::Name(name) => {
                        map.insert(name, chunks[index]);
                    },
                    NameOrLayout::Layout(inner_nested) => {
                        Self::run(inner_nested.as_ref(), chunks[index], map);
                    }
                }
            });
    }

    pub fn split(&self, rect: Rect) -> HashMap<&'static str, Rect> {
        let mut hash_map = HashMap::<&'static str, Rect>::new();
        Self::run(&self, rect, &mut hash_map);
        hash_map
    }
}

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }

    fn init_layout(size: Rect) -> HashMap<&'static str, Rect> {
        NestedLayout::new(
            Layout::default().direction(Direction::Vertical)
            .margin(1).constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3)
            ]), vec![NameOrLayout::Name("search_box"), NameOrLayout::Name("progress_bar"), NameOrLayout::Layout(Box::new(NestedLayout::new(
                Layout::default().direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
                ]), vec![NameOrLayout::Name("song_list"), NameOrLayout::Name("display_panel")]
            )))]
        ).split(size)
    }

    fn run<B: Backend>(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;


        let mut target = String::new();
        let mut entered_search_box = false;
        let search_box_block = Block::default().title("搜索栏").borders(Borders::ALL);
        let entered_search_box_block = Block::default().title("搜索栏").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)).border_type(BorderType::Double);

        let tick_rate = Duration::from_secs(1);
        let mut song_progress = 0;
        let mut last_tick = Instant::now();
        let mut song_pulsed = false;

        
        'run: loop {
            terminal.draw(|frame| {
                let layout_map = Self::init_layout(frame.size());

                frame.render_widget(
                    Paragraph::new(vec![text::Spans::from(vec![text::Span::raw(target.as_str())])])
                    .block(if entered_search_box {entered_search_box_block.clone()} else {search_box_block.clone()}),
                    layout_map["search_box"]
                );

                frame.render_widget(
                    Gauge::default()
                        .block(Block::default()
                            .title("寂寞难耐-李宗盛")
                            .borders(Borders::ALL))
                        .gauge_style(Style::default().fg(Color::Yellow))
                        .percent(song_progress),
                    layout_map["progress_bar"]
                );

                frame.render_widget(
                    //Paragraph::new(vec!["1. 寂寞难耐 - 李宗盛", "2. Long Season -  Fishmans", "3. Hotel California - Eagles", "4. Wish You Were Here - Pink Floyd"].into_iter().map(|song| text::Spans::from(vec![text::Span::raw(song)])).collect::<Vec<text::Spans>>())
                    List::new(vec!["1. 寂寞难耐 - 李宗盛", "2. Long Season -  Fishmans", "3. Hotel California - Eagles", "4. Wish You Were Here - Pink Floyd"].into_iter().map(|song| ListItem::new(song)).collect::<Vec<ListItem>>())
                    .block(Block::default().title("歌曲列表").borders(Borders::ALL)),
                    layout_map["song_list"]
                );

                frame.render_widget(
                    Block::default()
                        .title("搜索结果")
                        .borders(Borders::ALL),
                    layout_map["display_panel"] 
                );
            })?;

            if event::poll(tick_rate)? {
                match event::read()? {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('/') => entered_search_box = true,
                        KeyCode::Esc => entered_search_box = false,
                        KeyCode::Backspace => {
                            if entered_search_box {
                                target.pop();
                            }
                        },
                        KeyCode::Char(c) => {
                            if entered_search_box {
                                target.push(c);
                            } else {
                                match c {
                                    'q' => break 'run,
                                    ' ' => song_pulsed = !song_pulsed,
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if song_progress < 100 && !song_pulsed {
                    song_progress += 1;
                }
                last_tick = Instant::now();
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }
}



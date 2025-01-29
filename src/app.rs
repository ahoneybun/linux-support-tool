use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, List, ListItem, ListDirection},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    exit: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.exit = true;
        while self.exit {
            terminal.draw(|frame| App::draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(main_area);

        let title = Line::from("Support Tools")
            .bold()
            .blue()
            .centered();
        let about = Line::from("This provides some quick fixes to common issues")
            .centered();
        let footer = Line::from("Press `q` to quit the tool.")
            .centered();

        let items = ["Item 1", "Item 2", "Item 3"];
        let commands = List::new(items)
            .block(Block::bordered().title("Commands"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        let details = List::new(items)
            .block(Block::bordered().title("Details"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_widget(title, title_area);
        frame.render_widget(about, main_area);
        frame.render_widget(commands, left_area);
        frame.render_widget(details, right_area);
        frame.render_widget(footer, status_area);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q')) => self.quit(),
            (_, KeyCode::Char('e')) => self.echo(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.exit = false;
    }

    /// Other shortcuts
    fn echo(&mut self) {
        println!("Hello World");
    }
}

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use std::io::stdout;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;

    stdout.execute(terminal::Clear(ClearType::All))?;

    stdout.execute(cursor::MoveTo(40, 5))?;
    stdout.execute(SetForegroundColor(Color::DarkYellow))?;
    stdout.execute(Print("Welcome to crossterm, Press any key"))?;
    stdout.execute(ResetColor)?;

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char(c) => {
                        stdout.execute(cursor::MoveTo(40, 15))?;
                        stdout.execute(Print(format!("You pressed {}", c)))?;
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

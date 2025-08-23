use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use std::io::stdout;

pub fn run_typing_test(words: Vec<&str>, duration: u32) -> std::io::Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;

    stdout.execute(terminal::Clear(ClearType::All))?;

    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(Print(words.join(" ")))?;

    let mut typed = String::new();

    loop {
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char(c) => {
                        typed.push(c);

                        stdout.execute(cursor::MoveTo(0, 1))?;
                        stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
                        stdout.execute(Print(format!("You typed {}", typed)))?;
                    }
                    KeyCode::Backspace => {
                        typed.pop();
                        stdout.execute(cursor::MoveTo(0, 1))?;
                        stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
                        stdout.execute(Print(format!("You typed {}", typed)))?;
                    }
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

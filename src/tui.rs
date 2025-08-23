use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use std::io::stdout;

pub fn run_typing_test(words: Vec<&str>, duration: u32) -> std::io::Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(ClearType::All))?;

    let target_text = words.join(" ");
    let mut typed = String::new();

    loop {
        // clear the line
        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(terminal::Clear(ClearType::CurrentLine))?;

        // draw target line with per-char coloring and block caret
        let caret_pos = typed.chars().count();

        for (i, target_ch) in target_text.chars().enumerate() {
            // check if this is the caret position
            let is_caret = i == caret_pos;

            if let Some(typed_ch) = typed.chars().nth(i) {
                if typed_ch == target_ch {
                    // correct char -> white
                    stdout.execute(SetForegroundColor(Color::White))?;
                } else {
                    // wrong char -> red
                    stdout.execute(SetForegroundColor(Color::Red))?;
                }
            } else {
                // not typed yet -> grey
                stdout.execute(SetForegroundColor(Color::DarkGrey))?;

                // add block caret background for current position
                if is_caret {
                    stdout.execute(SetBackgroundColor(Color::White))?;
                    stdout.execute(SetForegroundColor(Color::Black))?;
                }
            }

            stdout.execute(Print(target_ch))?;
            stdout.execute(ResetColor)?; // reset after each char
        }

        // input handling
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => {
                        typed.push(c);
                    }
                    KeyCode::Backspace => {
                        typed.pop();
                    }
                    _ => {}
                }
            }
        }
    }

    // restore cursor before quitting
    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

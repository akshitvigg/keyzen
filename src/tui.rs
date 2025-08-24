use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::stdout;
use std::time::{Duration, Instant};




fn generate_unlimited_words(original_words: &[String], estimated_needed: usize) -> Vec<String> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    
    while result.len() < estimated_needed {
        let batch_size = original_words.len().min(estimated_needed - result.len());
        let mut batch: Vec<String> = original_words
            .choose_multiple(&mut rng, batch_size)
            .cloned()
            .collect();
        result.append(&mut batch);
    }
    
    result
}

fn create_text_lines(words: &[String], words_per_line: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    
    for word in words {
        current_line.push(word.clone());
        if current_line.len() >= words_per_line {
            lines.push(current_line.join(" "));
            current_line.clear();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line.join(" "));
    }
    
    lines
}

fn center_text(text: &str, width: u16) -> u16 {
    let text_len = text.chars().count() as u16;
    if text_len >= width {
        return 0;
    }
    (width - text_len) / 2
}

fn format_time(seconds: u64) -> String {
    if seconds >= 60 {
        let mins = seconds / 60;
        let secs = seconds % 60;
        format!("{}:{:02}", mins, secs)
    } else {
        format!("{}", seconds)
    }
}

fn draw_header_stats(stdout: &mut std::io::Stdout, terminal_width: u16, y_pos: u16, 
                    secs_left: u64, current_wpm: u32, current_accuracy: u32, 
                    start_time: Option<Instant>) -> std::io::Result<()> {
    
    if start_time.is_none() {
        let timer_text = format_time(secs_left);
        let timer_x = center_text(&timer_text, terminal_width);
        stdout.execute(cursor::MoveTo(timer_x, y_pos))?;
        stdout.execute(SetForegroundColor(Color::Yellow))?;
        stdout.execute(Print(&timer_text))?;
        stdout.execute(ResetColor)?;
        return Ok(());
    }

    let timer_text = format_time(secs_left);
    let stats_line = format!("{} │ {} wpm │ {}% acc", timer_text, current_wpm, current_accuracy);
    
    let stats_x = center_text(&stats_line, terminal_width);
    stdout.execute(cursor::MoveTo(stats_x, y_pos))?;
    
    stdout.execute(SetForegroundColor(Color::Yellow))?;
    stdout.execute(Print(&timer_text))?;
    
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(" │ "))?;
    
    stdout.execute(SetForegroundColor(Color::Cyan))?;
    stdout.execute(Print(&format!("{} wpm", current_wpm)))?;
    
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(" │ "))?;
    
    let acc_color = match current_accuracy {
        95..=100 => Color::Green,
        80..=94 => Color::Yellow,
        _ => Color::Red,
    };
    stdout.execute(SetForegroundColor(acc_color))?;
    stdout.execute(Print(&format!("{}% acc", current_accuracy)))?;
    
    stdout.execute(ResetColor)?;
    Ok(())
}

pub fn run_typing_test(words: Vec<String>, duration: u32, lang: &str) -> std::io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    if words.is_empty() {
        eprintln!("No words to type!");
        return Ok(());
    }

    let words_per_line = 10;
    let visible_lines = 3;
    
    let mut typed = String::new();
    let test_duration = Duration::from_secs(duration as u64);
    let mut start_time: Option<Instant> = None;
    let mut correct_chars = 0;
    let mut total_typed_chars = 0;
    let mut current_line_set = 0;
    
    
    let word_pool = words.clone();
    let mut all_text_lines = Vec::new();

    loop {
        let (terminal_width, terminal_height) = terminal::size()?;
        let elapsed = start_time.map(|t| t.elapsed()).unwrap_or(Duration::ZERO);

        if start_time.is_some() && elapsed >= test_duration {
            break;
        }

        let needed_lines = (current_line_set + 1) * visible_lines + 10; // Buffer
        while all_text_lines.len() < needed_lines {
            let more_words = generate_unlimited_words(&word_pool, words_per_line * 20);
            let more_lines = create_text_lines(&more_words, words_per_line);
            all_text_lines.extend(more_lines);
        }

        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(terminal::Clear(ClearType::All))?;

        let center_y = terminal_height / 2;

        let current_wpm = if start_time.is_some() && elapsed.as_secs() > 0 {
            let minutes = elapsed.as_secs_f64() / 60.0;
            (correct_chars as f64 / 5.0 / minutes).round() as u32
        } else {
            0
        };
        
        let current_accuracy = if total_typed_chars > 0 {
            ((correct_chars as f64 / total_typed_chars as f64) * 100.0).round() as u32
        } else {
            100 
        };

        let secs_left = if start_time.is_some() {
            test_duration.saturating_sub(elapsed).as_secs()
        } else {
            duration as u64
        };
        
        draw_header_stats(&mut stdout, terminal_width, center_y - 4, secs_left, 
                         current_wpm, current_accuracy, start_time)?;

        let start_line = current_line_set * visible_lines;
        let current_visible_lines: Vec<String> = all_text_lines
            .iter()
            .skip(start_line)
            .take(visible_lines)
            .cloned()
            .collect();

        let target_text = current_visible_lines.join(" ");
        let caret_pos = typed.chars().count();

        for (line_idx, line) in current_visible_lines.iter().enumerate() {
            let line_y = center_y - 1 + line_idx as u16;
            let line_x = center_text(line, terminal_width);
            stdout.execute(cursor::MoveTo(line_x, line_y))?;

            let line_start_pos = if line_idx == 0 {
                0
            } else {
                current_visible_lines[..line_idx]
                    .iter()
                    .map(|l| l.len() + 1) // +1 for space between lines
                    .sum::<usize>()
            };

            for (char_idx, target_ch) in line.chars().enumerate() {
                let global_pos = line_start_pos + char_idx;
                let is_caret = global_pos == caret_pos;

                stdout.execute(ResetColor)?;

                if let Some(typed_ch) = typed.chars().nth(global_pos) {
                    if typed_ch == target_ch {
                        stdout.execute(SetForegroundColor(Color::Green))?;
                        stdout.execute(Print(typed_ch))?;
                    } else {
                        stdout.execute(SetForegroundColor(Color::Red))?;
                        stdout.execute(Print(typed_ch))?;
                    }
                } else {
                    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
                    if is_caret {
                        stdout.execute(SetBackgroundColor(Color::White))?;
                        stdout.execute(SetForegroundColor(Color::Black))?;
                    }
                    stdout.execute(Print(target_ch))?;
                }
                
                stdout.execute(ResetColor)?;
            }
        }

        let instructions = if start_time.is_none() {
            "press any key to start typing"
        } else {
            "esc: quit • backspace: delete"
        };
        let inst_x = center_text(instructions, terminal_width);
        stdout.execute(cursor::MoveTo(inst_x, center_y + 4))?;
        stdout.execute(SetForegroundColor(Color::DarkGrey))?;
        stdout.execute(Print(instructions))?;
        stdout.execute(ResetColor)?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(c) => {
                            if start_time.is_none() {
                                start_time = Some(Instant::now());
                            }

                            typed.push(c);
                            total_typed_chars += 1;

                            if let Some(expected) = target_text.chars().nth(typed.len() - 1) {
                                if c == expected {
                                    correct_chars += 1;
                                }
                            }

                            if typed.len() >= target_text.len() {
                                current_line_set += 1;
                                typed.clear();
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(removed_char) = typed.pop() {
                                if total_typed_chars > 0 {
                                    total_typed_chars -= 1;
                                    if let Some(expected) = target_text.chars().nth(typed.len()) {
                                        if removed_char == expected && correct_chars > 0 {
                                            correct_chars -= 1;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    show_results(&mut stdout, correct_chars, total_typed_chars, start_time, lang)?;
    
    Ok(())
}

fn show_results(
    stdout: &mut std::io::Stdout, 
    correct_chars: usize, 
    total_typed_chars: usize, 
    start_time: Option<Instant>,
    lang: &str
) -> std::io::Result<()> {
    stdout.execute(terminal::Clear(ClearType::All))?;
    
    let (terminal_width, terminal_height) = terminal::size()?;
    let center_y = terminal_height / 2;

    let actual_duration = start_time.map(|t| t.elapsed()).unwrap_or(Duration::ZERO);
    let seconds_taken = actual_duration.as_secs_f64();
    let accuracy = if total_typed_chars > 0 {
        (correct_chars as f64 / total_typed_chars as f64) * 100.0
    } else {
        0.0
    };
    let minutes = seconds_taken / 60.0;
    let wpm = if minutes > 0.0 {
        correct_chars as f64 / 5.0 / minutes
    } else {
        0.0
    };

    let title = "── results ──";
    let title_x = center_text(title, terminal_width);
    stdout.execute(cursor::MoveTo(title_x, center_y - 3))?;
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(title))?;
    stdout.execute(ResetColor)?;

    let results_x = center_text(&format!("{:.0} wpm │ {:.0}% acc │ {:.0}s │ {}", wpm, accuracy, seconds_taken, lang), terminal_width);
    stdout.execute(cursor::MoveTo(results_x, center_y))?;
    
    let wpm_color = match wpm as u32 {
        60.. => Color::Green,
        40..=59 => Color::Cyan,
        20..=39 => Color::Yellow,
        _ => Color::Red,
    };
    stdout.execute(SetForegroundColor(wpm_color))?;
    stdout.execute(Print(&format!("{:.0}", wpm)))?;
    
    stdout.execute(SetForegroundColor(Color::Grey))?;
    stdout.execute(Print(" wpm"))?;
    
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(" │ "))?;
    
    let acc_color = match accuracy as u32 {
        95..=100 => Color::Green,
        80..=94 => Color::Yellow,
        _ => Color::Red,
    };
    stdout.execute(SetForegroundColor(acc_color))?;
    stdout.execute(Print(&format!("{:.0}", accuracy)))?;
    
    stdout.execute(SetForegroundColor(Color::Grey))?;
    stdout.execute(Print("% acc"))?;
    
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(" │ "))?;
    
    stdout.execute(SetForegroundColor(Color::Grey))?;
    stdout.execute(Print(&format!("{:.0}s", seconds_taken)))?;
    
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(" │ "))?;
    
    stdout.execute(SetForegroundColor(Color::Grey))?;
    stdout.execute(Print(lang))?;
    stdout.execute(ResetColor)?;

    let instructions = "tab: restart • esc: quit";
    let inst_x = center_text(instructions, terminal_width);
    stdout.execute(cursor::MoveTo(inst_x, center_y + 3))?;
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print(instructions))?;
    stdout.execute(ResetColor)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Tab => {
                        break;
                    }
                    KeyCode::Esc => {
                        stdout.execute(cursor::Show)?;
                        terminal::disable_raw_mode()?;
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }

    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
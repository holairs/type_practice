use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Result, Write};

pub fn get_keyboard_input() -> Result<()> {
    enable_raw_mode()?; // <- Necessary to auto-trigger actions
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // <-- clear the screen
    println!("Press Escape to exit");

    loop {
        // Waits an keyboard event
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // <-- clear the screen
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char(c) => {
                        // Gets the typed char
                        if modifiers.is_empty() {
                            println!("Typed char: {}", c);
                        } else {
                            println!("Typed char: {} (modifier: {:?})", c, modifiers);
                        }
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Esc => {
                        println!("Escape key | Closing...");
                        break;
                    }
                    _ => {
                        println!("Special: {:?}", code);
                    }
                }
            }
        }
    }

    // Disable rawmode at exit
    disable_raw_mode()?;
    Ok(())
}

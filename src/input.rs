use crossterm::event::{self, Event, KeyCode};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn setup_input_listener() -> mpsc::Receiver<KeyCode> {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
      loop {
          // Wait for key inputs...
          if event::poll(Duration::from_millis(100)).unwrap() {
              if let Event::Key(key_event) = event::read().unwrap() {
                  tx.send(key_event.code).unwrap();
              }
          }
      }
  });

  rx
}

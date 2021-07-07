use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

use rusty_audio::Audio;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};

mod components;

use components::frame::{new_frame, Drawable};
use components::player::Player;
use components::render::render;
use components::shot::Shot;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("win", "assets/sounds/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        while let Ok(current_frame) = render_rx.recv() {
            render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        // Per-Frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);

        // Draw and render
        player.draw(&mut current_frame);
        let _ = render_tx.send(current_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    return Ok(());
}

use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, thread_rng};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use common::{message::server::Message, Color};
use tokio::sync::mpsc::UnboundedSender;

lazy_static! {
    pub static ref COLORS: Vec<Color> = {
        vec![
            Color::new("red", 255, 0, 0),
            Color::new("green", 0, 255, 0),
            Color::new("blue", 0, 0, 255),
            Color::new("magenta", 255, 0, 255),
            Color::new("yellow", 255, 255, 0),
            Color::new("cyan", 0, 255, 255),
            Color::new("orange", 255, 165, 0),
            Color::new("dark green", 0, 100, 0),
            Color::new("teal", 0, 128, 128),
            Color::new("pink", 255, 192, 203),
            Color::new("brown", 165, 42, 42),
        ]
    };
}

fn random_color() -> Color {
    COLORS.choose(&mut thread_rng()).unwrap().clone()
}

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub color: Color,
    pub position: (i32, i32),
    pub sender: UnboundedSender<Message>,
}

impl User {
    pub fn new(sender: UnboundedSender<Message>) -> Self {
        let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
        let color = random_color();
        let position = (0, 0);
        User {
            id,
            color,
            position,
            sender,
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub users: HashMap<usize, User>,
}

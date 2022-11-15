mod state;

use std::{cell::RefCell, rc::Rc};

use common::message::{client, server};
use futures::{SinkExt, StreamExt};
use kodec::binary::Codec;
use wasm_bindgen::prelude::*;

use js_utils::{event::When, set_panic_hook, spawn, window};
use mezzenger::{Messages, Receive};
use web_sys::{MouseEvent, WebSocket};

use crate::state::{Ball, State};

#[wasm_bindgen(start)]
pub async fn main_client() -> Result<(), JsValue> {
    set_panic_hook();

    console_log::init_with_level(log::Level::Debug).expect("failed to init logger");
    log::info!("Hello World!");

    // setting up web socket
    let host = window()
        .location()
        .host()
        .expect("couldn't extract host from location");
    let url = format!("ws://{host}/ws");
    let web_socket = Rc::new(WebSocket::new(&url).unwrap());
    let (web_socket_sender, mut web_socket_receiver) = mezzenger_websocket::Transport::<
        Codec,
        server::Message,
        client::Message,
    >::new(&web_socket, Codec::default())
    .await
    .unwrap()
    .split();
    let web_socket_sender = Rc::new(RefCell::new(web_socket_sender));

    // wait for init message
    let my_color = {
        let init_message = web_socket_receiver.receive().await.unwrap();
        match init_message {
            server::Message::Init { id, color } => {
                log::info!("Received init message: id: {id}, color: {color}.");
                color
            }
            _ => panic!("unexpected message received"),
        }
    };

    let state = Rc::new(RefCell::new(State::new(my_color)));

    let mut message_stream = web_socket_receiver.messages_with_error_callback(move |error| {
        log::error!("Error occurred while receiving message from web socket: {error}.");
    });

    // setting up mouse move event handler
    let state_clone = state.clone();
    let _mouse_move_handler = Rc::new(window())
        .when("mousemove", move |event: MouseEvent| {
            let state = state_clone.borrow_mut();
            let position = (event.client_x(), event.client_y());
            state.my_ball.set_position(position);
            let message = client::Message { position };
            let web_socket_sender_clone = web_socket_sender.clone();
            spawn(async move {
                if let Err(error) = web_socket_sender_clone.borrow_mut().send(message).await {
                    log::error!("Failed to send message with error: {error}.");
                }
            });
        })
        .unwrap();

    // handle messages from server
    while let Some(message) = message_stream.next().await {
        match message {
            server::Message::Ball {
                id,
                color,
                position,
            } => {
                log::info!("Received 'new ball' message: id: {id}, color: {color}.");
                let ball = Ball::new(color);
                ball.set_position(position);
                state.borrow_mut().other_balls.insert(id, ball);
            }
            server::Message::Update { id, position } => {
                if let Some(ball) = state.borrow_mut().other_balls.get_mut(&id) {
                    ball.set_position(position);
                } else {
                    log::error!("Could not find ball with id: {id}.");
                }
            }
            server::Message::Disconnected { id } => {
                log::info!("Received 'disconnected' message: id: {id}.");
                if state.borrow_mut().other_balls.remove(&id).is_none() {
                    log::error!("Could not find ball with id: {id}.");
                }
            }
            _ => panic!("unexpected message received"),
        }
    }

    Ok(())
}

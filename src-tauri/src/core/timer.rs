use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::time;
use tauri::async_runtime::{spawn, JoinHandle};

#[derive(Default)]
pub struct Timer {
    interval: Option<Duration>,
    handle: Option<JoinHandle<()>>,
    sender: Option<tokio::sync::mpsc::UnboundedSender<Duration>>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            interval: None,
            handle: None,
            sender: None,
        }
    }

    pub fn start<F>(&mut self, interval: Duration, callback: F)
    where
        F: FnMut() + Send + 'static,
    {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
        let (sender, receiver) = unbounded_channel::<Duration>();
        let handle = spawn(async move {
            Self::timer_loop(interval, receiver, callback).await;
        });
        self.interval = Some(interval);
        self.sender = Some(sender);
        self.handle = Some(handle);
    }

    pub async fn stop(&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        if let Some(handle) = self.handle.take() {
            handle.abort();
            handle.await.unwrap_or_default();
        }
        self.interval = None;
    }

    pub fn set_interval(&mut self, period: Duration) {
        if let Some(sender) = self.sender.as_ref() {
            println!("set_interval send");
            sender.send(period).unwrap();
        }
        self.interval = Some(period);
    }

    async fn timer_loop<F>(period: Duration, mut receiver: UnboundedReceiver<Duration>, mut callback: F)
    where
        F: FnMut() + Send,
    {
        let mut interval = time::interval(period);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    callback();
                }
                Some(period) = receiver.recv() => {
                    println!("timer loop recv {:?}", period);
                    interval.tick().await;
                    println!("Contrinue timer loop");
                    interval = time::interval(period);
                }
            }
        }
    }
}

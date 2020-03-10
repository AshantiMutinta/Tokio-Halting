use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::prelude::*;
use tokio::sync::{self, oneshot};

const NUM_FUTURES: usize = 10;

lazy_static! {
    static ref REPL: sync::Mutex<()> = sync::Mutex::new(());
}
#[tokio::main]
async fn main() {
    println!("press enter to pause and see the state");
    let state = Arc::new(sync::Mutex::new(0));
    for _ in 0..NUM_FUTURES {
        tokio::spawn(foo(state.clone()));
    }
    let j = tokio::spawn(async move {
        loop {
            let mut stdin = io::stdin();
            // Future awaits without blocking thread
            let _ = stdin
                .read(&mut [0])
                .await
                .expect("Failed to read input");
            let (o_tx, o_rx) = oneshot::channel();
            tokio::spawn(pause(o_rx));
            let s = state.lock().await;
            println!("Paused, current state: {}", s);
            println!("Press enter to continue");
            let _ = stdin
                .read(&mut [0])
                .await
                .expect("Failed to read input");
            o_tx.send(()).ok();
        }
    });
    j.await.ok();
}

async fn foo(state: Arc<sync::Mutex<u32>>) {
    loop {
        // Try to get the lock if it can't then await
        try_repl().await;
        let mut s = state.lock().await;
        *s += 1;
        tokio::time::delay_for(std::time::Duration::from_millis(10)).await;
    }
}

async fn try_repl() {
    crate::REPL.lock().await;
}

async fn pause(rx: oneshot::Receiver<()>) {
    let _h = crate::REPL.lock().await;
    rx.await.ok();
}

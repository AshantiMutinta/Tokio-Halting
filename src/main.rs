use std::{
    io::{self, Read},
    thread, time,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("When program halts, press enter button to skip debug mode");
    loop {
        tokio::spawn(async move {
            let spawn_id = Uuid::new_v4();
            println!("Tokio spawned for {:?}", spawn_id);
            let mut stdin = io::stdin();
            let _ = stdin.read(&mut [0]).unwrap();
            println!("Debug mode ended for {:?}", spawn_id);
        });
        thread::sleep(time::Duration::from_millis(10));
    }
}

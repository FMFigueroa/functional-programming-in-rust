use std::sync::Arc;
use tokio::{
    sync::{Barrier, Notify},
    time::{sleep, Duration},
};

// cmd: cargo watch -q -c -w examples/ -x 'run --example philosopher_eating'

async fn philosopher_eating(name: &str, barrier: Arc<Barrier>, notify: Arc<Notify>) -> () {
    println!("{} está comiendo.", name);
    //sleep(Duration::from_secs(1)).await; // Simulación de comer
    let wait_result = barrier.wait().await;
    println!("{} ha finalizado de comer.", name);

    if wait_result.is_leader() {
        notify.notify_one();
    }
}

#[tokio::main]
async fn main() {
    let total_philosopher_eating = 2;

    let barrier = Arc::new(Barrier::new(total_philosopher_eating));
    let notify = Arc::new(Notify::new());
    notify.notify_one();

    let philosophers: Vec<&str> = vec![
        "Gilles Deleuze",
        "Emma Goldman",
        "Judith Butler",
        "Karl Marx",
        "Michel Foucault",
    ];

    let mut sorted_philosophers = philosophers.clone();
    sorted_philosophers.sort(); // Ordenar alfabéticamente

    let handle_spawn = sorted_philosophers.into_iter().enumerate().fold(
        Vec::new(),
        |mut handles, (i, philosopher)| {
            let philosopher_arc = philosopher;
            let barrier_arc = barrier.clone();
            let notify_arc = notify.clone();

            let handle = tokio::spawn(async move {
                if i % total_philosopher_eating == 0 {
                    notify_arc.notified().await;
                    sleep(Duration::from_secs(1)).await;
                }
                philosopher_eating(philosopher_arc, barrier_arc, notify_arc).await;
            });

            handles.push(handle);
            handles
        },
    );

    for handle in handle_spawn {
        let _result = handle.await.unwrap();
    }
}

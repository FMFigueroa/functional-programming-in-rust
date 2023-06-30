use std::sync::Arc;
use tokio::{
    sync::{Barrier, BarrierWaitResult, Notify},
    time::{sleep, Duration},
};

// cmd: cargo watch -q -c -w examples/ -x 'run --example philosopher_eating'

async fn philosopher_eating(
    name: &str, barrier: Arc<Barrier>, notify: Arc<Notify>,
) -> BarrierWaitResult {
    println!("{} está comiendo.", name);
    //sleep(Duration::from_secs(1)).await; // Simulación de comer
    let wait_result = barrier.wait().await;
    println!("{} ha finalizado de comer.", name);

    if wait_result.is_leader() {
        notify.notify_one();
    }

    wait_result
}

#[tokio::main]
async fn main() {
    let total_philosopher_eating = 2;

    let barrier = Arc::new(Barrier::new(total_philosopher_eating));
    let notify = Arc::new(Notify::new());
    notify.notify_one();

    let philosophers: String =
        "Gilles Deleuze, Emma Goldman, Judith Butler, Karl Marx, Michel Foucault".to_string();

    let task_handles: Vec<_> = philosophers
        .split(", ")
        .enumerate()
        .map(|(i, philosopher)| {
            let philosopher_arc = philosopher.clone();
            let barrier_arc = barrier.clone();
            let notify_arc = notify.clone();

            async move {
                if i % total_philosopher_eating == 0 {
                    notify_arc.notified().await;
                    sleep(Duration::from_secs(1)).await;
                }
                tokio::spawn(philosopher_eating(philosopher_arc, barrier_arc, notify_arc))
                    .await
                    .unwrap()
            }
        })
        .collect();

    for handle in task_handles {
        let result = handle.await;
    }
}

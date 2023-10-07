use petname::petname;
use tokio::sync::{oneshot, mpsc};
use diesel::{prelude::*, QueryDsl, result::Error};

#[derive(Debug)]
struct KeyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    keys: Vec<String>,
}

enum ActorMessage {
    NewKey {
        respond_to: oneshot::Sender<String>,
    },
}

impl KeyActor {
    async fn new(pool: deadpool_diesel::postgres::Pool, receiver: mpsc::Receiver<ActorMessage>) -> Self {
        let mut existing_keys: Vec<String> = vec![];

        use crate::schema::gusts::key;
        if let Ok(conn) = pool.get().await {
            let res: Vec<String> = conn
            .interact(|conn| {
                crate::schema::gusts::table
                    .select(key)
                    .load(conn)
            })
            .await
            .unwrap_or_else(|_| -> Result<Vec<String>, Error> {
                Ok(Vec::<String>::new())
            })
            .unwrap_or_default();

            existing_keys = res;

            println!("{:?}", existing_keys);
        }

        Self{
            receiver: receiver,
            keys: existing_keys,
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::NewKey { respond_to } => {
                let mut new_key = petname(4, "-");
                while self.keys.contains(&new_key) {
                    new_key = petname(4, "-");
                }

                self.keys.push(new_key.clone());

                let _ = respond_to.send(new_key);
            }
        }
    }
}

async fn run_my_actor(mut actor: KeyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct KeyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl KeyActorHandle {
    pub async fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = KeyActor::new(pool, receiver).await;
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub async fn get_unique_key(&self) -> String {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::NewKey {
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}
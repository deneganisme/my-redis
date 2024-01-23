/*
This is from Tokio tutorial on channels which is really Message Passing my fucking fave

Brush up on MP:
    - spawn a dedicated task to manage a resource (in this case our client)
    - tasks send a message to the client task
    - the client issues the request on behalf of the sender and returns it back to the sender

So, to get terms straight we pass "messages" into a channel and tokio provides us diff flavors of channels

(Some) Tokio channels
    - mpsc: "multi producer, single consumer"
    - oneshot: single producer, single consumer
    - broadcast: multi prod and consumer -- each receiver sees every value
    - watch: single producer, multi consumer -- many values CAN be sent but no history is kept, receivers only see most recent value 
*/

use bytes::Bytes;
use tokio::sync::mpsc;


#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }

}

#[tokio::main]
async fn main() {

    // Channel created with specific capacity
    // If messages sent faster than received, channel stores them up to capacity
    // If capacity reached then calling send(...).await will SLEEP until message has been removed by receiver
    let capacity = 32;

    // tx = sender; rx = receiver
    let (tx, mut rx) = mpsc::channel(capacity);

    // Clone tx for everything beyond first transaction
    let tx2 = tx.clone();
    
    tokio::spawn(async move {
        tx.send("sending t1").await;
    })

    tokio::spawn(async move {
        tx2.send("sending t2").await;
    })

    // And that's it here's our client just listen to receiver
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

}




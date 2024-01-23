use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


// Arc = atomic reference counter -- prevents var from going out of scope when spawning processes
// Mutex = "mutual exclusion" -- lets only one process change it at a atime using .lock()
//  - mutex locks are dropped when the lock goes out of scope or you can drop it with std::mem::drop
//  - the lock can be thought of as ref to Mutex's value
// let mut mutexA_changer5 = my_mutex.lock().unwrap(); -> change value with *mutex_changer = ??; 
type Db = Arc<Mutex<HashMap<String, Bytes>>>;


#[tokio::main]
async fn main() {

    let ip_addr = "127.0.0.1";
    
    let port = "6378";
    
    let listener = TcpListener::bind(format!("{ip_addr}:{port}")).await.unwrap();

    // So we are saying create this DB which other processes can reference but only ONE process can work on it at a time
    let db = Arc::new(Mutex::new(HashMap::new()));
    
    loop {
        
        // 2nd item contains IP and port of new connection 
        let (socket, _) = listener.accept().await.unwrap();
        
        let db = db.clone();

        // Spawn new task for each inbound socket
        // the socket is moved to the new task and processed there
        // returns a JoinHandle which we can call upon later with await
        tokio::spawn(async move {
            process(socket, db).await;
        });

    }
}

async fn process(socket: TcpStream, db: Db) {
    
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        
        let response = match Command::from_frame(frame).unwrap() {
            
            Set(cmd) => {
                
                /*
                    Blocking mutex acceptable when contention is minimal
                    When lock is contended, thread executing blocks waiting for mutex
                    This blocks current task but also all other tasks scheduled on current thread
                */
                let mut db = db.lock().unwrap();

                db.insert(cmd.key().to_string(), cmd.value().to_vec().into());

                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {

                let db = db.lock().unwrap();

                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }

}
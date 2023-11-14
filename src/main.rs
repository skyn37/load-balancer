use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio::time::interval;

#[derive(Debug, Clone)]
struct Server {
    port: String,
    ip: String,
    current_index: usize,
    online: Arc<Mutex<bool>>,
}

impl Server {
    fn new(port: String, ip: String, current_index: usize, online: Arc<Mutex<bool>>,) -> Self {
        Server {
            port,
            ip,
            current_index,
            online
        }
    }

    fn get_index(&self) -> usize {
        self.current_index
    }

    async fn check_health(&self) -> bool {

        let health = call_server(Some(&self.ip), &self.port, &[0]).await;
        match health {
            Ok(_) => {
                self.set_online().await;
                true
            }
            Err(_) => {
                self.set_offline().await;
                false
            }
         }
    }

    async fn set_offline(&self) {
        let mut online = self.online.lock().await;
        *online = false;
    }

    async fn set_online(&self) {
        let mut online = self.online.lock().await;
        *online = true;
    }
}


async fn hanlde_args() -> Option<Vec<Server>> {
    let args: Vec<String> = env::args().collect();
    let mut server_collection: Vec<Server> = vec![];
    if args.len() > 1 {
        let arg_values: Vec<&str> = args.iter().skip(1).flat_map(|s| s.split(',')).collect();
        for (index, server) in arg_values.iter().enumerate() {
            if let Some((a, b)) = server.split_once(':') {
                server_collection.push(Server::new(b.into(), a.into(), index, Arc::new(Mutex::new(true))));
            } else {
            }
        }
        Some(server_collection)
    } else {
        return None;
    }
}

async fn call_server(ip: Option<&str>, port: &str, request: &[u8]) -> Result<String, io::Error> {
    let addr = ip.unwrap_or("127.0.0.1");

    let timeout_duration = Duration::from_secs(5);

    let mut stream = timeout(
        timeout_duration,
        TcpStream::connect(format!("{}:{}", addr, port)),
    )
    .await??;

    stream.write_all(request).await?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).await?;
    return Ok(buffer);
}

async fn frwd_request(
    mut data: TcpStream,
    server_collection: Arc<Vec<Server>>,
    next_server: Arc<Mutex<usize>>,
) -> Result<(), io::Error> {
    let server_index = {
        let mut counter = next_server.lock().await;
        let index = *counter;
        *counter = (*counter + 1) % server_collection.len();
        index
    };

    // Try to find the next online server
    let mut next_index = server_index;
    while !*server_collection[next_index].online.lock().await {
        println!("Server {} is offline, trying the next server...", server_collection[next_index].get_index());
        next_index = (next_index + 1) % server_collection.len();
        if next_index == server_index {
            println!("All servers are offline, request not forwarded");
            return Ok(());
        }
    }

    let server = &server_collection[next_index];

    let mut buffer = [0; 1024];
    let req_bytes = data.read(&mut buffer).await?;
    if req_bytes == 0 {
        return Ok(());
    }
    let forwarded_res = call_server(Some(&server.ip), &server.port, &buffer).await?;
    data.write_all(forwarded_res.as_bytes()).await?;

    Ok(())
}



async fn health_check_loop(server_collection: Arc<Vec<Server>>) {
    // Check the health of each server at regular intervals
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;

        for server in server_collection.iter() {
            let is_healthy = server.check_health().await;

            if !is_healthy {
                println!("Server {} is not healthy", server.get_index());
            } else { 
                println!("Server {} healthy", server.get_index());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let server_collection = match hanlde_args().await {
        Some(servers) => Arc::new(servers),
        None => {
            eprintln!("No servers provided");
            return Ok(());
        }
    };
    let next_server = Arc::new(Mutex::new(0));
    let addr = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr).await?;
    println!("Starting Load Balancer on: {}", addr);

    // healthcheck 
    tokio::spawn(health_check_loop(Arc::clone(&server_collection)));


    // handle incomming requests 
    loop {
        let (stream, _) = listener.accept().await?;
        let cloned_server = Arc::clone(&next_server);
        let cloned_collection = server_collection.clone();
        tokio::spawn(async move {
            if let Err(e) = frwd_request(stream, cloned_collection, cloned_server).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }
}

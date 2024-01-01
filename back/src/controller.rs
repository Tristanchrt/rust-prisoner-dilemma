use rand::Rng;
use settings::{Game, Log, Party, Player, Protocol, Settings, Status};
use std::collections::HashMap;
use std::io::{Bytes, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::result;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Controller {
    pub listener: TcpListener,
    pub game: Game,
    pub players_stream: HashMap<u32, Arc<Mutex<TcpStream>>>,
}

type BufferSize = [u8; 1024];

impl Controller {
    pub fn new(settings: &Settings) -> Self {
        let listener =
            TcpListener::bind(String::from(format!("{}:{}", settings.host, settings.port)))
                .unwrap();
        let game = Game::default();
        let players_stream = HashMap::new();
        Self {
            listener,
            game,
            players_stream,
        }
    }

    pub fn run(&mut self) {
        let tcp_listener_c = self.listener.try_clone().unwrap();
        for stream in tcp_listener_c.incoming() {
            match stream {
                Ok(result) => self.process_message(&result),
                Err(_) => Log::show(
                    "ERROR",
                    format!("Someting went wrong for reading the stream"),
                ),
            }
        }
    }

    pub fn process_message(&mut self, mut tcp_stream: &TcpStream) {
        let mut buffer: BufferSize = [0; 1024];
        loop {
            match tcp_stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Client disconnected");
                        break;
                    }

                    let protocol: Protocol = Protocol::from_bytes(&buffer[..bytes_read]);

                    println!("Get new message");

                    self.handle_party(&protocol, &tcp_stream);
                }
                Err(e) => {
                    println!("Error reading from socket: {:?}", e);
                    break;
                }
            }
        }
    }

    pub fn handle_party(&mut self, protocol: &Protocol, tcp_stream: &TcpStream) {
        match protocol.party_status {
            Status::Init => self.init_player(&tcp_stream),
            Status::Created => self.create_game(&protocol),
            Status::WaitingPlayer => println!("3"),
            Status::JoinParty => self.join_game(&protocol),
            Status::Started => println!("2"),
            Status::Finished => println!("4"),
            _ => println!("Something went wrong with the Party status"),
        }
    }

    pub fn join_game(&mut self, protocol: &Protocol) {
        if let Some(found_element) = self
            .game
            .parties
            .iter()
            .find(|&element| element.status == Status::WaitingPlayer)
        {
            println!("Element found: {:?}", found_element);
        } else {
            println!("Element not found");
        }
    }

    pub fn send_message(&self, bytes: &Vec<u8>, mut tcp_steam: &TcpStream) {
        tcp_steam.write_all(&bytes).expect("error write");
        tcp_steam.flush().expect("error flush");
    }

    pub fn create_game(&mut self, protocol: &Protocol) {
        let mut party = Party::default();
        let mut rng = rand::thread_rng();

        party.id = rng.gen::<u32>();
        party.status = Status::WaitingPlayer;

        let default_player = Player::default();
        let player_from_protocol = protocol.player.clone();

        party.player1 = default_player;
        party.player2 = player_from_protocol;

        let player_id: u32 = protocol.player.id;

        let mut protocol_send = protocol.clone();
        protocol_send.party_id = party.id;
        protocol_send.party_status = Status::Created;
        self.game.add_party(party);

        let tcp: Option<MutexGuard<'_, TcpStream>> = self.get_stream(player_id);

        let bytes = protocol.to_bytes();
        match tcp {
            Some(stream) => {
                self.send_message(&bytes, &stream);
            }
            None => Log::show("ERROR", format!("Error getting stream")),
        }
    }

    pub fn get_stream(&self, player_id: u32) -> Option<MutexGuard<'_, TcpStream>> {
        if let Some(stream_arc_mutex) = self.players_stream.get(&player_id) {
            if let Ok(locked_stream) = stream_arc_mutex.lock() {
                Some(locked_stream)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn init_player(&mut self, tcp_stream: &TcpStream) {
        let mut protocol: Protocol = Protocol::default();
        let mut rng = rand::thread_rng();
        protocol.player.id = rng.gen::<u32>();

        Log::show("INFO", format!("New user #{}", protocol.player.id));

        let bytes = protocol.to_bytes();

        let cloned_stream = tcp_stream.try_clone().expect("Failed to clone TcpStream");

        self.send_message(&bytes, &cloned_stream);

        self.players_stream
            .insert(protocol.player.id, Arc::new(Mutex::new(cloned_stream)));
    }
}

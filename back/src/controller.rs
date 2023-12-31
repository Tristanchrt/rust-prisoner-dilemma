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
        let tcp_listener_c = self.listener.try_clone();
        for stream in tcp_listener_c.unwrap().incoming() {
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
        let _size = tcp_stream.read(&mut buffer).unwrap();

        if _size == 0 {
            panic!("Error reading message");
        }

        let protocol: Protocol = Protocol::from_bytes(&buffer[.._size]);

        self.handle_party(&protocol, &tcp_stream);
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

    pub fn create_game(&mut self, protocol: &Protocol) {
        let mut party = Party::default();
        let mut rng = rand::thread_rng();

        party.id = rng.gen::<u32>();
        party.status = Status::WaitingPlayer;

        let default_player = Player::default();
        let player_from_protocol = protocol.player.clone();

        party.player1 = default_player;
        party.player2 = player_from_protocol;
        self.game.add_party(party);

        let player_id: u32 = protocol.player.id;
        let tcp: Option<MutexGuard<'_, TcpStream>> = self.get_stream(player_id);
        match tcp {
            Some(stream) => {
                println!("stream, {:?}", stream)
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

    pub fn init_player(&mut self, mut tcp_stream: &TcpStream) {
        let mut protocol: Protocol = Protocol::default();
        let mut rng = rand::thread_rng();
        protocol.player.id = rng.gen::<u32>();

        Log::show("INFO", format!("New user #{}", protocol.player.id));

        let bytes = protocol.to_bytes();

        tcp_stream.write_all(&bytes).unwrap();
        tcp_stream.flush().unwrap();

        let cloned_stream = tcp_stream.try_clone().expect("Failed to clone TcpStream");

        self.players_stream
            .insert(protocol.player.id, Arc::new(Mutex::new(cloned_stream)));
    }
}

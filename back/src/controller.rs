use rand::Rng;
use settings::{Game, Log, Party, Player, Protocol, Settings, Status};
use std::io::{Bytes, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::result;

pub struct Controller {
    pub listener: TcpListener,
    pub game: Game,
}

type BufferSize = [u8; 1024];

impl Controller {
    pub fn new(settings: &Settings) -> Self {
        let listener =
            TcpListener::bind(String::from(format!("{}:{}", settings.host, settings.port)))
                .unwrap();
        let game = Game::default();
        Self { listener, game }
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
            Status::Created => self.create_game(&protocol, &tcp_stream),
            Status::WaitingPlayer => println!("3"),
            Status::JoinParty => self.join_game(&protocol, &tcp_stream),
            Status::Started => println!("2"),
            Status::Finished => println!("4"),
            _ => println!("Something went wrong with the Party status"),
        }
    }

    pub fn join_game(&mut self, protocol: &Protocol, mut tcp_stream: &TcpStream) {
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

    pub fn create_game(&mut self, protocol: &Protocol, mut tcp_stream: &TcpStream) {
        let mut party = Party::default();
        let mut rng = rand::thread_rng();

        party.id = rng.gen::<u32>();
        party.status = Status::WaitingPlayer;

        let default_player = Player::default();
        let player_from_protocol = protocol.player.clone();

        party.players = (default_player, player_from_protocol);
        self.game.add_party(party);
    }

    pub fn init_player(&self, mut tcp_stream: &TcpStream) {
        let mut protocol: Protocol = Protocol::default();
        let mut rng = rand::thread_rng();
        protocol.player.id = rng.gen::<u32>();

        Log::show("INFO", format!("New user #{}", protocol.player.id));

        let bytes = protocol.to_bytes();

        tcp_stream.write_all(&bytes).unwrap();
        tcp_stream.flush().unwrap();
    }
}

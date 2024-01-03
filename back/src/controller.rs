use rand::Rng;
use settings::{Game, Log, Party, Player, Protocol, Settings, Status};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub struct Controller {
    pub listener: TcpListener,
    pub game: Arc<Mutex<Game>>,
    pub players_stream: Arc<Mutex<HashMap<u32, TcpStream>>>,
}

type BufferSize = [u8; 1024];

impl Controller {
    pub fn new(settings: &Settings) -> Self {
        let listener =
            TcpListener::bind(String::from(format!("{}:{}", settings.host, settings.port)))
                .unwrap();
        let game = Arc::new(Mutex::new(Game::default()));
        let players_stream = Arc::new(Mutex::new(HashMap::new()));
        Self {
            listener,
            game,
            players_stream,
        }
    }

    pub fn run(&self) {
        let tcp_listener_c = self.listener.try_clone().unwrap();
        for stream in tcp_listener_c.incoming() {
            match stream {
                Ok(tcp) => {
                    let shared_game = Arc::clone(&self.game);
                    let shared_players_stream = Arc::clone(&self.players_stream);

                    thread::spawn(move || {
                        Controller::process_message(&tcp, shared_game, shared_players_stream);
                    });
                }

                Err(_) => Log::show(
                    "ERROR",
                    format!("Someting went wrong for reading the stream"),
                ),
            }
        }
    }

    pub fn process_message(
        mut tcp_stream: &TcpStream,
        game: Arc<Mutex<Game>>,
        players: Arc<Mutex<HashMap<u32, TcpStream>>>,
    ) {
        let mut buffer: BufferSize = [0; 1024];
        loop {
            match tcp_stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Client disconnected");
                        break;
                    }

                    let protocol: Protocol = Protocol::from_bytes(&buffer[..bytes_read]);

                    println!("Get new message {:?}", protocol);

                    Controller::handle_party(&protocol, &tcp_stream, &game, &players);
                }
                Err(e) => {
                    println!("Error reading from socket: {:?}", e);
                    break;
                }
            }
        }
    }

    pub fn handle_party(
        protocol: &Protocol,
        tcp_stream: &TcpStream,
        game: &Arc<Mutex<Game>>,
        players: &Arc<Mutex<HashMap<u32, TcpStream>>>,
    ) {
        match protocol.party_status {
            Status::Init => Controller::init_player(&tcp_stream, players),
            Status::Created => Controller::create_game(&protocol, &players, &game),
            Status::JoinParty => Controller::join_game(&protocol, &players, &game),
            Status::Started => Controller::process_game(&protocol, &players, &game),
            Status::Finished => println!("4"),
            _ => println!("Something went wrong with the Party status"),
        }
    }
    pub fn process_game(
        protocol: &Protocol,
        players: &Arc<Mutex<HashMap<u32, TcpStream>>>,
        game: &Arc<Mutex<Game>>,
    ) {
        let mut game_arc = game.lock().unwrap();
        if let Some(game_) = game_arc
            .parties
            .iter_mut()
            .find(|element| element.id == protocol.party_id)
        {
            println!("Find game {:?}", game_.id);
            if game_.round < game_.total_round {
                game_.round += 1;
            } else {
            }
        } else {
            println!("Not party found");
        }
    }

    pub fn join_game(
        protocol: &Protocol,
        players: &Arc<Mutex<HashMap<u32, TcpStream>>>,
        game: &Arc<Mutex<Game>>,
    ) {
        let mut game_arc = game.lock().unwrap();
        if let Some(element) = game_arc
            .parties
            .iter_mut()
            .find(|element| element.status == Status::WaitingPlayer)
        {
            println!("Find game {:?}", element.id);
            element.player1 = protocol.player.clone();
            element.status = Status::Started;

            let mut protocol_send = protocol.clone();
            protocol_send.party_status = Status::Started;

            let players_to_send = [element.player1.clone(), element.player2.clone()];
            for player in players_to_send.iter() {
                println!("Player send {:?}", player);
                let tcp: TcpStream = Controller::get_stream(&players, player.id);
                protocol_send.player = player.clone();
                let bytes = protocol_send.to_bytes();
                Controller::send_message(&bytes, &tcp);
            }
        } else {
            println!("Not party found");
        }
    }

    pub fn send_message(bytes: &Vec<u8>, mut tcp_steam: &TcpStream) {
        tcp_steam.write_all(&bytes).expect("error write");
        tcp_steam.flush().expect("error flush");
    }

    pub fn create_game(
        protocol: &Protocol,
        players: &Arc<Mutex<HashMap<u32, TcpStream>>>,
        game: &Arc<Mutex<Game>>,
    ) {
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

        let mut game_mutux = game.lock().unwrap();

        game_mutux.add_party(party);

        let tcp: TcpStream = Controller::get_stream(&players, player_id);

        let bytes = protocol_send.to_bytes();
        Controller::send_message(&bytes, &tcp);
    }

    pub fn get_stream(players: &Arc<Mutex<HashMap<u32, TcpStream>>>, player_id: u32) -> TcpStream {
        let players_stream_arc = players.lock().unwrap();
        return players_stream_arc
            .get(&player_id)
            .expect("Error get stream 1")
            .try_clone()
            .expect("Error get stream 2");
    }

    pub fn init_player(tcp_stream: &TcpStream, players: &Arc<Mutex<HashMap<u32, TcpStream>>>) {
        let mut protocol: Protocol = Protocol::default();
        let mut rng = rand::thread_rng();
        protocol.player.id = rng.gen::<u32>();

        Log::show("INFO", format!("New user #{}", protocol.player.id));

        let bytes = protocol.to_bytes();

        let cloned_stream = tcp_stream.try_clone().expect("Failed to clone TcpStream");

        Controller::send_message(&bytes, &cloned_stream);

        let mut players_stream_arc = players.lock().unwrap();
        players_stream_arc.insert(protocol.player.id, cloned_stream);
    }
}

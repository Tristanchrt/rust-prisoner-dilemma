slint::include_modules!();
use settings::{Log, Protocol, Settings, Status};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

pub struct Interface {
    pub ui: AppWindow,
}

pub struct Client {
    pub tcp: TcpStream,
}

pub struct Controller {
    pub settings: Settings,
}

type BufferSize = [u8; 1024];

impl Client {
    pub fn new(host: &str, port: &str) -> Self {
        let tcp = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
        Self { tcp }
    }

    fn init(&mut self) {
        let default_protocol = Protocol::default();
        self.send_message(default_protocol);

        let mut buffer: BufferSize = [0; 1024];
        let bytes_read: usize = self.tcp.read(&mut buffer).expect("Read error");

        let protocol: Protocol = Protocol::from_bytes(&buffer[..bytes_read]);
        Log::show(
            "INFO",
            format!(
                "Hello user #{} status {:?}",
                protocol.player.id, protocol.party_status
            ),
        );
    }

    fn send_message(&mut self, protocol: Protocol) {
        let bytes = protocol.to_bytes();
        self.tcp.write_all(&bytes).unwrap();
        self.tcp.flush().unwrap();
    }
}

impl Controller {
    pub fn run(&self) {
        let ui = Interface::new();
        let mut client = Client::new(&self.settings.host, &self.settings.port);

        client.init();
        ui.init(&client.tcp);

        ui.run();
    }
}

impl Interface {
    pub fn new() -> Self {
        let ui = AppWindow::new().unwrap();
        Self { ui }
    }

    fn init(&self, tcp_stream: &TcpStream) {
        self.set_default_input();
        self.reset_interface();
        self.attach_event_handlers(tcp_stream);
    }

    fn set_protocol_data(&self, protocol: Protocol) {}

    fn choice_interface(&self) {}

    fn attach_event_handlers(&self, tcp_stream: &TcpStream) {
        self.ui.on_event_game(move |data| {
            Log::show("INFO", data.to_string());
        });

        let ui_clone = self.ui.clone_strong();
        self.ui.on_create_game(move || {
            let round = ui_clone.get_number_round();
            let bet = ui_clone.get_number_bet();
            println!("AAAA {} {}", round, bet);
        });
    }

    fn set_default_input(&self) {
        self.ui.set_number_bet(10);
        self.ui.set_number_round(5);
    }

    fn reset_interface(&self) {
        self.ui.set_menu_visible(true);
        self.ui.set_game_visible(false);
        self.ui.set_search_visible(false);
        self.ui.set_create_visible(false);
        self.ui.set_wait_visible(false);
    }

    fn run(&self) {
        let _ = self.ui.run();
    }
}

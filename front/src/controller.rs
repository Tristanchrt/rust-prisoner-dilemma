slint::include_modules!();
use settings::{Log, Protocol, Settings, Status};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::thread::sleep;
use std::time::Duration;

pub struct Client {
    pub tcp: TcpStream,
    pub protocol: Protocol,
}

pub struct Controller {
    pub settings: Settings,
    pub interface: AppWindow,
    pub client: Client,
}

type BufferSize = [u8; 1024];

impl Client {
    pub fn new(host: &str, port: &str) -> Self {
        let tcp = TcpStream::connect(format!("{}:{}", host, port)).expect("Connection failed.");
        let protocol = Protocol::default();
        Self { tcp, protocol }
    }

    fn init(&mut self) {
        let bytes = self.protocol.to_bytes();
        self.send_message(&bytes);

        let mut buffer: BufferSize = [0; 1024];
        let bytes_read: usize = self.tcp.read(&mut buffer).expect("Read error");

        self.protocol = Protocol::from_bytes(&buffer[..bytes_read]);
        Log::show(
            "INFO",
            format!(
                "Hello user #{} status {:?}",
                self.protocol.player.id, self.protocol.party_status
            ),
        );
    }

    fn send_message(&mut self, bytes: &Vec<u8>) {
        self.tcp.write_all(&bytes).unwrap();
        self.tcp.flush().unwrap();
    }

    pub fn close_connection(&mut self) {
        self.tcp
            .shutdown(Shutdown::Both)
            .expect("Failed to close connection");
    }
}

pub struct Interface {}

impl Interface {
    fn set_default_input(ui: &AppWindow) {
        ui.set_number_bet(10);
        ui.set_number_round(5);
    }

    fn reset_interface(ui: &AppWindow) {
        ui.set_menu_visible(false);
        ui.set_game_visible(false);
        ui.set_search_visible(false);
        ui.set_create_visible(false);
        ui.set_wait_visible(false);
    }

    fn go_create_game_ui(ui: &AppWindow) {
        Interface::reset_interface(&ui);
        Interface::set_default_input(&ui);
        ui.set_create_visible(true);
    }

    fn go_waiting_player(ui: &AppWindow) {
        Interface::reset_interface(&ui);
        ui.set_wait_visible(true);
    }
}

impl Controller {
    pub fn new(settings: Settings) -> Self {
        Self {
            client: Client::new(&settings.host, &settings.port),
            settings: settings,
            interface: AppWindow::new().unwrap(),
        }
    }
    pub fn run(&mut self) {
        self.client.init();
        self.init();
        let _ = self.interface.run();
    }

    fn init(&mut self) {
        Interface::reset_interface(&self.interface);
        self.attach_event_handlers();
        self.interface.set_menu_visible(true);
    }

    fn set_protocol_data(&self, protocol: Protocol) {}

    fn choice_interface(&self) {}

    fn attach_event_handlers(&mut self) {
        let ui_cloned = self.interface.clone_strong();

        self.interface.on_event_game(move |data| {
            Log::show("INFO", data.to_string());
            if data.trim() == "CREATE" {
                Interface::go_create_game_ui(&ui_cloned);
            } else {
            }
        });

        let mut protocol = self.client.protocol.clone();
        let mut tcp_stream: TcpStream = self.client.tcp.try_clone().expect("Clone failed...");

        let ui_cloned = self.interface.clone_strong();

        self.interface.on_create_game(move || {
            let total_round = ui_cloned.get_number_round();
            let bet = ui_cloned.get_number_bet();

            protocol.bet = bet as u32;
            protocol.total_round = total_round as u32;
            protocol.party_status = Status::Created;

            let bytes = protocol.to_bytes();

            tcp_stream.write_all(&bytes).unwrap();
            tcp_stream.flush().unwrap();

            let mut buffer: BufferSize = [0; 1024];
            let bytes_read: usize = tcp_stream.read(&mut buffer).expect("Read error");

            let protcol = Protocol::from_bytes(&buffer[..bytes_read]);
            Log::show("INFO", format!("From server {:?}", protcol));

            Interface::go_waiting_player(&ui_cloned);
        });
    }
}

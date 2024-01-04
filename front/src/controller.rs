slint::include_modules!();
use settings::{Log, Protocol, Settings, Status};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::RwLock;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

pub struct Controller {
    pub settings: Settings,
    pub interface: Arc<Mutex<AppWindow>>,
    pub tcp: TcpStream,
    pub protocol: Arc<Mutex<Protocol>>,
}

type BufferSize = [u8; 1024];

// impl Client {
//     fn init(&mut self) {
//         let bytes = self.protocol.to_bytes();
//         self.send_message(&bytes);

//         let mut buffer: BufferSize = [0; 1024];
//         loop {
//             match self.tcp.read(&mut buffer) {
//                 Ok(bytes_read) => {
//                     if bytes_read == 0 {
//                         Log::show("ERROR", "Connection closed by remote endpoint".to_string());
//                     }
//                     self.protocol = Protocol::from_bytes(&buffer[..bytes_read]);
//                     Log::show(
//                         "INFO",
//                         format!(
//                             "Hello user #{} status {:?}",
//                             self.protocol.player.id, self.protocol.party_status
//                         ),
//                     );
//                 }
//                 Err(e) => {
//                     eprintln!("Read error: {}", e);
//                     break;
//                 }
//             }
//         }
//     }

//     fn send_message(&mut self, bytes: &Vec<u8>) {
//         self.tcp.write_all(&bytes).unwrap();
//         self.tcp.flush().unwrap();
//     }
// }

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

    fn go_in_game(ui: &AppWindow) {
        ui.set_search_visible(true);
    }
    fn go_waiting_player(ui: &AppWindow) {
        Interface::reset_interface(&ui);
        ui.set_wait_visible(true);
    }
}

unsafe impl Send for AppWindow {}
unsafe impl Sync for AppWindow {}

impl Controller {
    pub fn new(settings: Settings) -> Self {
        let tcp = TcpStream::connect(format!("{}:{}", settings.host, settings.port))
            .expect("Connection failed.");
        Self {
            settings: settings,
            tcp: tcp,
            protocol: Arc::new(Mutex::new(Protocol::default())),
            interface: Arc::new(Mutex::new(AppWindow::new().unwrap())),
        }
    }

    pub fn run(&mut self) {
        let ui = Arc::clone(&self.interface);
        let ui_for_closure = Arc::clone(&self.interface);
        let protocol = Arc::clone(&self.protocol);
        let protocol_for_closure = Arc::clone(&self.protocol);
        let protocol_for_closure_read = Arc::clone(&self.protocol);
        let mut tcp_stream = self.tcp.try_clone().unwrap();
        thread::spawn(move || {
            {
                let protocol_mut = protocol_for_closure.lock().unwrap();
                let bytes: Vec<u8> = protocol_mut.to_bytes();
                tcp_stream.write_all(&bytes).unwrap();
                tcp_stream.flush().unwrap();
            }

            let mut buffer: BufferSize = [0; 1024];
            loop {
                match tcp_stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {}
                        Log::show("ERROR", "Connection closed by remote endpoint".to_string());
                        let updated_protocol = Protocol::from_bytes(&buffer[..bytes_read]);
                        let mut protocol_guard = protocol_for_closure_read.lock().unwrap();
                        *protocol_guard = updated_protocol; // Update the content inside the Mutex
                        Log::show("INFO", format!("Protocol : {:?}", protocol_guard));

                        let protocol_c = protocol_guard.clone();
                        match protocol_c.party_status {
                            Status::Started => {
                                println!("EOLA");
                                let ui_arc = ui_for_closure.lock().unwrap();
                                ui_arc.clone_strong().set_search_visible(true);
                            }
                            _ => Log::show("ERROR", format!("Status unknowned")),
                        }
                    }
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        break;
                    }
                }
            }
        });

        let tcp_stream = self.tcp.try_clone().unwrap();

        Controller::init(ui, &tcp_stream, protocol)
    }

    fn init(ui: Arc<Mutex<AppWindow>>, tcp_stream: &TcpStream, protocol: Arc<Mutex<Protocol>>) {
        Controller::attach_event_handlers(&ui, &tcp_stream, protocol);
        let ui_arc = ui.lock().expect("Error");
        Interface::reset_interface(&ui_arc);
        ui_arc.set_menu_visible(true);
        let _ = ui_arc.run();
    }

    fn process_game(
        ui: &Arc<Mutex<AppWindow>>,
        tcp_stream: &TcpStream,
        protocol: &Arc<Mutex<Protocol>>,
    ) {
        let protocol_c = protocol.clone();
        let protocl_arc = protocol_c.lock().unwrap();
        match protocl_arc.party_status {
            Status::Started => println!("toto"),
            _ => Log::show("ERROR", format!("Status unknowned")),
        }
    }
    fn starting_game(
        ui: &Arc<Mutex<AppWindow>>,
        tcp_stream: &TcpStream,
        protocol: &Arc<Mutex<Protocol>>,
    ) {
        println!("TTTTTTTTTTESSS");
    }

    fn attach_event_handlers(
        ui: &Arc<Mutex<AppWindow>>,
        tcp_stream: &TcpStream,
        protocol: Arc<Mutex<Protocol>>,
    ) {
        let ui_cloned = ui.lock().unwrap().clone_strong();
        let mut tcp_stream_: TcpStream = tcp_stream.try_clone().expect("Clone failed...");

        let protocol_cloned_ = protocol.clone();
        ui.lock().unwrap().on_event_game(move |data| {
            Log::show("INFO", data.to_string());
            if data.trim() == "CREATE" {
                Interface::go_create_game_ui(&ui_cloned);
            } else {
                let mut protocol_cloned = protocol_cloned_.lock().unwrap();
                protocol_cloned.party_status = Status::JoinParty;

                let bytes = protocol_cloned.to_bytes();
                tcp_stream_.write_all(&bytes).unwrap();
                tcp_stream_.flush().unwrap();

                Interface::go_waiting_player(&ui_cloned);
            }
        });

        let ui_cloned = ui.lock().unwrap().clone_strong();
        let mut tcp_stream__: TcpStream = tcp_stream.try_clone().expect("Clone failed...");

        ui.lock().unwrap().on_create_game(move || {
            let total_round = ui_cloned.get_number_round();
            let bet = ui_cloned.get_number_bet();
            let mut protocol_cloned = protocol.lock().unwrap();
            protocol_cloned.bet = bet as u32;
            protocol_cloned.total_round = total_round as u32;
            protocol_cloned.party_status = Status::Created;

            let bytes = protocol_cloned.to_bytes();
            tcp_stream__.write_all(&bytes).unwrap();
            tcp_stream__.flush().unwrap();

            Interface::go_waiting_player(&ui_cloned);
        });
    }
}

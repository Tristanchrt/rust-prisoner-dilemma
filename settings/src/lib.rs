use config::Config;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub money: f64,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Init,
    Created,
    WaitingPlayer,
    JoinParty,
    Started,
    Finished,
}
#[derive(Debug)]
pub struct Party {
    pub id: u32,
    pub total_round: u32,
    pub round: u32,
    pub status: Status,
    pub bet: u32,
    pub players: (Player, Player),
    pub winner: Option<Player>,
    pub looser: Option<Player>,
}
#[derive(Debug)]
pub struct Game {
    pub parties: Vec<Party>,
    pub players: Vec<Player>,
}

#[derive(Debug)]
pub struct Protocol {
    pub player: Player,
    pub party_status: Status,
    pub total_round: u32,
    pub round: u32,
    pub bet: u32,
    pub party_id: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self { id: 0, money: 0.0 }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Init
    }
}

impl Default for Party {
    fn default() -> Self {
        Self {
            id: 0,
            total_round: 0,
            round: 0,
            status: Status::default(),
            bet: 0,
            players: (Player::default(), Player::default()),
            winner: None,
            looser: None,
        }
    }
}

impl Game {
    pub fn add_party(&mut self, party: Party) {
        self.parties.push(party);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            parties: Vec::new(),
            players: Vec::new(),
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self {
            player: Player::default(),
            party_status: Status::default(),
            total_round: 0,
            round: 0,
            bet: 0,
            party_id: 0,
        }
    }
}

impl Player {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.id.to_be_bytes());
        bytes.extend(&self.money.to_be_bytes());
        bytes
    }
    fn from_bytes(bytes: &[u8]) -> Option<Player> {
        if bytes.len() < 12 {
            return None;
        }
        let id = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let money_bytes: [u8; 8] = [
            bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
        ];
        let money = f64::from_be_bytes(money_bytes);
        Some(Player { id, money })
    }
}

impl Protocol {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.player.to_bytes());
        bytes.push(match self.party_status {
            Status::Init => 0,
            Status::Created => 1,
            Status::WaitingPlayer => 2,
            Status::Started => 3,
            Status::Finished => 4,
            Status::JoinParty => 5,
        });
        bytes.extend(&self.total_round.to_be_bytes());
        bytes.extend(&self.round.to_be_bytes());
        bytes.extend(&self.bet.to_be_bytes());
        bytes.extend(&self.party_id.to_be_bytes());
        bytes
    }
    pub fn from_bytes(bytes: &[u8]) -> Protocol {
        let player_bytes: &[u8] = &bytes[..12];
        let player: Player = Player::from_bytes(player_bytes).unwrap();

        let party_status = match bytes[12] {
            0 => Status::Init,
            1 => Status::Created,
            2 => Status::WaitingPlayer,
            3 => Status::Started,
            4 => Status::Finished,
            5 => Status::JoinParty,
            _ => Status::Init,
        };

        let total_round: u32 = u32::from_be_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
        let round: u32 = u32::from_be_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
        let bet: u32 = u32::from_be_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]);
        let party_id: u32 = u32::from_be_bytes([bytes[25], bytes[26], bytes[27], bytes[28]]);

        Protocol {
            player,
            party_status,
            total_round,
            round,
            bet,
            party_id,
        }
    }
}

pub struct Settings {
    pub host: String,
    pub port: String,
    pub client_max: String,
    pub buffer_size1: String,
    pub buffer_size2: String,
    pub party_name: String,
}

pub struct Log;

impl Log {
    pub fn show(key: &str, value: String) {
        match key {
            "WARN" => println!("** [WARN] {} **", value),
            "DEBUG" => println!("** [DEBUG] {} **", value),
            "INFO" => println!("** [INFO] {} **", value),
            "ERROR" => println!("** [ERROR] {} **", value),
            _ => println!("{}", value),
        }
    }
}

impl Settings {
    pub fn load(file_name: &str) -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name(file_name))
            .build()
            .unwrap();

        let settings_map = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        Self {
            host: Self::get_configuration_value(&settings_map, "host"),
            port: Self::get_configuration_value(&settings_map, "port"),
            client_max: Self::get_configuration_value(&settings_map, "client_max"),
            buffer_size1: Self::get_configuration_value(&settings_map, "buffer_size1"),
            buffer_size2: Self::get_configuration_value(&settings_map, "buffer_size2"),
            party_name: Self::get_configuration_value(&settings_map, "party_name"),
        }
    }

    pub fn get_configuration_value(dict: &HashMap<String, String>, key: &str) -> String {
        match dict.get(key) {
            Some(value) => value.to_string(),
            None => {
                Log::show(
                    "ERROR",
                    format!("Key {} not found in the configuration file.", key),
                );
                String::new()
            }
        }
    }
}

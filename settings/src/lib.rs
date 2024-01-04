use config::Config;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub money: f64,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Init,
    Created,
    WaitingPlayer,
    JoinParty,
    Started,
    Finished,
    Win,
    Lose,
    Equal,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlayStatus {
    Betrail,
    Cooperate,
    Stanby,
}

#[derive(Debug, Clone)]
pub struct PartyRound {
    pub round_played: Vec<((Player, PlayStatus, u32), (Player, PlayStatus, u32))>,
}
#[derive(Debug, Clone)]
pub struct Party {
    pub id: u32,
    pub total_round: u32,
    pub round: u32,
    pub status: Status,
    pub bet: u32,
    pub player1: Player,
    pub player2: Player,
    pub winner: Option<Player>,
    pub looser: Option<Player>,
    pub party_round: PartyRound,
}
#[derive(Debug)]
pub struct Game {
    pub parties: Vec<Party>,
    pub players: Vec<Player>,
}

#[derive(Debug, Clone)]
pub struct Protocol {
    pub player: Player,
    pub party_status: Status,
    pub total_round: u32,
    pub round: u32,
    pub bet: u32,
    pub party_id: u32,
    pub play: PlayStatus,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            money: 100.0,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Init
    }
}

impl Default for PlayStatus {
    fn default() -> Self {
        PlayStatus::Stanby
    }
}

impl Default for PartyRound {
    fn default() -> Self {
        Self {
            round_played: Vec::new(),
        }
    }
}

impl Default for Party {
    fn default() -> Self {
        Self {
            id: 0,
            total_round: 0,
            round: 1,
            status: Status::default(),
            bet: 0,
            winner: None,
            looser: None,
            player1: Player::default(),
            player2: Player::default(),
            party_round: PartyRound::default(),
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
            play: PlayStatus::default(),
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
        let mut bytes = Vec::with_capacity(30); // Adjust the capacity based on your exact byte size

        bytes.extend_from_slice(&self.player.to_bytes());

        bytes.push(match self.party_status {
            Status::Init => 0,
            Status::Created => 1,
            Status::WaitingPlayer => 2,
            Status::Started => 3,
            Status::Finished => 4,
            Status::JoinParty => 5,
            Status::Win => 6,
            Status::Lose => 7,
            Status::Equal => 8,
        });

        bytes.extend_from_slice(&self.total_round.to_be_bytes());
        bytes.extend_from_slice(&self.round.to_be_bytes());
        bytes.extend_from_slice(&self.bet.to_be_bytes());
        bytes.extend_from_slice(&self.party_id.to_be_bytes());

        bytes.push(match self.play {
            PlayStatus::Betrail => 0,
            PlayStatus::Cooperate => 1,
            PlayStatus::Stanby => 2,
        });

        bytes
    }
    pub fn from_bytes(bytes: &[u8]) -> Protocol {
        let player_bytes: &[u8; 12] = bytes.get(..12).unwrap().try_into().ok().unwrap();
        let player: Player = Player::from_bytes(player_bytes).unwrap();

        let party_status = match bytes[12] {
            0 => Status::Init,
            1 => Status::Created,
            2 => Status::WaitingPlayer,
            3 => Status::Started,
            4 => Status::Finished,
            5 => Status::JoinParty,
            6 => Status::Win,
            7 => Status::Lose,
            8 => Status::Equal,
            _ => Status::Init, // Invalid status byte
        };

        let total_round = u32::from_be_bytes(bytes[13..17].try_into().unwrap());
        let round = u32::from_be_bytes(bytes[17..21].try_into().unwrap());
        let bet = u32::from_be_bytes(bytes[21..25].try_into().unwrap());
        let party_id = u32::from_be_bytes(bytes[25..29].try_into().unwrap());

        let play = match bytes[29] {
            0 => PlayStatus::Betrail,
            1 => PlayStatus::Cooperate,
            2 => PlayStatus::Stanby,
            _ => PlayStatus::Stanby, // Invalid play status byte
        };

        Protocol {
            player,
            party_status,
            total_round,
            round,
            bet,
            party_id,
            play,
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

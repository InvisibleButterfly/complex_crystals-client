use std::mem::replace;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub status: String,
    pub tps: u16,
}
impl ServerInfo {
    pub fn replace(&mut self, new_info: ServerInfo) {
        self.name = new_info.name;
        self.status = new_info.status;
        self.tps = new_info.tps;
    }
}

#[derive(RustcDecodable)]
pub struct WorldSize {
    pub width: f64,
    pub height: f64,
}
impl WorldSize {
    pub fn replace(&mut self, new_size: WorldSize) {
        self.width = new_size.width;
        self.height = new_size.height;
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone, PartialEq, Debug)]
pub enum ObjectType {
    Asteroid,
    Builder,
    Harvester,
    Battlecruiser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum RadarType {
    None,
    Simple,
    Middle,
    Military,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum WeaponType {
    None,
    Mining,
    Laser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum CargoType {
    None,
    Mining,
    Battery,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum ArmorType {
    Asteroid,
    Light,
    Middle,
    Heavy,
    Building,
}

#[derive(RustcDecodable, Debug)]
pub struct ObjectResponse {
    pub name: String,
    pub owner: String,
    pub x: f64,
    pub y: f64,
    pub otype: ObjectType,
}

#[derive(RustcEncodable)]
pub struct ObjectInfoRequest {
    pub name: String,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    pub owner: String,
    pub name: String,
    pub otype: ObjectType,
    pub x: f64,
    pub y: f64,

    pub drive_speed: f64,
    pub drive_dest_x: f64,
    pub drive_dest_y: f64,

    pub radar_radius: f64,
    pub radar_type: RadarType,

    pub weapon_active: bool,
    pub weapon_type: WeaponType,
    pub weapon_radius: f64,
    pub weapon_target_x: f64,
    pub weapon_target_y: f64,

    pub cargo_type: CargoType,
    pub cargo_max: f64,
    pub cargo_current: f64,

    pub shell_health: f64,
    pub shell_type: ArmorType,
}

impl SampleObject {
    pub fn new_empty() -> Self {
        SampleObject {
            owner: "none".to_owned(),
            name: "none".to_owned(),
            otype: ObjectType::Asteroid,
            x: 0.0,
            y: 0.0,

            drive_speed: 0.0,
            drive_dest_x: 0.0,
            drive_dest_y: 0.0,

            radar_radius: 0.0,
            radar_type: RadarType::None,

            weapon_active: false,
            weapon_type: WeaponType::None,
            weapon_radius: 0.0,
            weapon_target_x: 0.0,
            weapon_target_y: 0.0,

            cargo_type: CargoType::None,
            cargo_max: 0.0,
            cargo_current: 0.0,

            shell_health: 0.0,
            shell_type: ArmorType::Asteroid,
        }
    }
    pub fn replace_object(&mut self, new: SampleObject) {
        replace(self, new);
    }
}

impl ObjectType {
    pub fn to_string(&self) -> String {
        match *self {
            ObjectType::Asteroid => "Asteroid".to_string(),
            ObjectType::Builder => "Builder".to_string(),
            ObjectType::Harvester => "Harvester".to_string(),
            ObjectType::Battlecruiser => "Battlecruiser".to_string(),
        }
    }
}

impl RadarType {
    pub fn to_string(&self) -> String {
        match *self {
            RadarType::None => "None".to_string(),
            RadarType::Middle => "Middle".to_string(),
            RadarType::Military => "Military".to_string(),
            RadarType::Simple => "Simple".to_string(),
        }
    }
}

impl WeaponType {
    pub fn to_string(&self) -> String {
        match *self {
            WeaponType::None => "None".to_string(),
            WeaponType::Laser => "Laser".to_string(),
            WeaponType::Mining => "Mining".to_string(),
        }
    }
}

impl CargoType {
    pub fn to_string(&self) -> String {
        match *self {
            CargoType::None => "None".to_string(),
            CargoType::Battery => "Battery".to_string(),
            CargoType::Mining => "Mining".to_string(),
        }
    }
}

impl ArmorType {
    pub fn to_string(&self) -> String {
        match *self {
            ArmorType::Asteroid => "Asteroid".to_string(),
            ArmorType::Building => "Building".to_string(),
            ArmorType::Heavy => "Heavy".to_string(),
            ArmorType::Light => "Light".to_string(),
            ArmorType::Middle => "Middle".to_string(),
        }
    }
}
use osmpbf::WayNodeLocation;




pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
}

impl Position {
    pub fn from_way_node_location(way_node: WayNodeLocation) -> Position {
        Position { longitude: way_node.lon(), latitude: way_node.lat() }
    }
}

pub enum MediumType{
   Default,
   Highway (StreetCategory),
   Railway,
   Waterway,
   Airway,
   SpaceTrajectory 
}

pub enum StreetCategory {
    Motorway,
    MotorwayLink,
    Trunk,
    TrunkLink,
    Primary,
    PrimaryLink,
    Secondary,
    SecondaryLink,
    Tertiary,
    TertiaryLink,
    Unclassified,
    Residential,
    LivingStreet,
    Service,
    Track,
    Road,
    Cycleway,
    Pedestrian,
    Path,
}

pub struct Medium {
    pub osm_id: Option<i64>,
    pub medium_type: MediumType,
    pub medium_positions: Vec<Position>
}

impl Medium {
    pub fn new() -> Medium {
        Medium {
            osm_id: None,
            medium_type: MediumType::Default,
            medium_positions: Vec::new() 
        }
    }
}

impl Default for Medium {
    fn default() -> Self {
        Self::new()
     }
}

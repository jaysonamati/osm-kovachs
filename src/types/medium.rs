use osmpbf::WayNodeLocation;

#[derive(Debug)]
pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
}

impl Position {
    pub fn from_way_node_location(way_node: WayNodeLocation) -> Position {
        Position { longitude: way_node.lon(), latitude: way_node.lat() }
    }
}

#[derive(Debug)]
pub enum MediumType{
   Default,
   Highway (Vec<StreetCategory>),
   Railway,
   Waterway,
   Airway,
   SpaceTrajectory 
}

#[derive(Debug)]
pub enum StreetCategory {
    /// High capacity highways designed to safely carry fast motor traffic.
    Motorway,
    /// The link roads (sliproads / ramps) leading to and from a motorway.
    MotorwayLink,
    /// Important roads that are not motorways.
    Trunk,
    /// The link road (sliproad / ramp) leading to and from a trunk highway.
    TrunkLink,
    /// A highway linking large towns.
    Primary,
    /// Connecting slip roads/ramps of primary highways
    PrimaryLink,
    /// A highway linking large towns.
    Secondary,
    /// Connecting slip roads/ramps of secondary highways.
    SecondaryLink,
    /// A road linking small settlements, or the local centres of a large town or city.
    Tertiary,
    /// Connecting slip road/ramp of a tertiary highway.
    TertiaryLink,
    /// Public access road, non-residential.
    Unclassified,
    /// Road in a residential area
    Residential,
    /// Road with very low speed limits and other pedestrian friendly traffic rules.
    LivingStreet,
    /// Generally for access to a building, service station, beach, campsite, industrial estate, business park, etc.
    Service,
    /// A minor land-access road like a farm or forest track.
    Track,
    /// Road with unknown classification.
    Road,
    /// For designated cycleways
    Cycleway,
    /// Roads mainly / exclusively for pedestrians
    Pedestrian,
    /// A generic path used by pedestrians, small vehicles, for animal riding or livestock walking. Not used by two-track vehicles. Very broad, non-specific meaning.
    Path,
    /// A path mainly or exclusively for pedestrians.
    Footway,
    /// The location of a street crossing for pedestrians, cyclists, or equestrians.
    Crossing,
    /// The street category has not been set
    Default,
}

#[derive(Debug)]
pub struct Medium {
    pub osm_id: Option<i64>,
    pub medium_osm_name: Option<String>,
    pub medium_type: MediumType,
    pub is_one_way: bool,
    pub medium_positions: Vec<Position>
}

impl Medium {
    pub fn new() -> Medium {
        Medium {
            osm_id: None,
            medium_osm_name: None, 
            medium_type: MediumType::Default,
            is_one_way: false,
            medium_positions: Vec::new() 
        }
    }
}

impl Default for Medium {
    fn default() -> Self {
        Self::new()
     }
}

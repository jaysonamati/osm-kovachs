pub mod types;

use std::time::SystemTime;

use osmpbf::{Element, ElementReader, IndexedReader};
use types::medium::{Medium, MediumType, Position, StreetCategory};

fn main() {
    println!("Reading command line args");
    let arg = std::env::args_os()
        .nth(1)
        .expect("need a *.osm.pbf file as argument");
    let path: &std::path::Path = std::path::Path::new(&arg);

    println!("Reading OSM PBF File: {:#?}", path);
    let path_str: &str = "/hdd/Data/osm/kenya-latest.osm.pbf";
    // count_ways_kenya(path_str)
    // count_everything(path);
    parse_all_to_medium(path);
}


fn count_ways_kenya(path_str: &str) {
    let reader = ElementReader::from_path(path_str).unwrap();
    let mut ways = 0_u64;

    // Increment the counter by one for each way.
    reader.for_each(|element| {
        if let Element::Way(w) =  element {
            eprintln!("{}",format!("Counting way: {}", w.id()));
            ways += 1;
        }
    }).unwrap();

    println!("{ways}: ways in file: {path_str}");

}

fn count_everything(path: &std::path::Path) {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    println!("Counting...");
    match reader.par_map_reduce(
        |element|match element {
            Element::Node(_) | Element::DenseNode(_) => {
                (1, 0, 0)
            },
            Element::Way(w) => {
                if w.node_locations().len() < 2 {
                    
                }
                let mut keys = Vec::new();
                let mut values = Vec::new();
                let ways_iter  = w.tags();
                for (key, value) in ways_iter {
                    if key.eq("highway") {
                        keys.push(key);
                        values.push(value);
                    } else if key.eq("surface") {
                        keys.push(key);
                        values.push(value);
                    };
                }
                let way_id = w.id();
                // println!("Way: {way_id} has tags of keys: {:#?} and values: {:#?}.", keys, values);
                (0, 1, 0)
            },
            Element::Relation(_)=> (0, 0, 1)
        }, // map_op,
        ||(0u64,0u64, 0u64), // identity,
        |a, b|(a.0 + b.0, a.1 + b.1, a.2 + b.2),//reduce_op
    ) {
        Ok((nodes, ways, relations))=> {
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
            println!("Finished counting in: {:#?}", duration);
            println!("Nodes: {nodes}");
            println!("Ways: {ways}");
            println!("Relations: {relations}");
        }
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    }
}

pub fn parse_all_to_medium(path: &std::path::Path) {
    let _reader = ElementReader::from_path(path).unwrap();
    let mut indexed_reader = IndexedReader::from_path(path).unwrap();
    println!("Parsing!");
    let start_time = SystemTime::now();
    println!("Started at {:#?}", start_time);
    
    let mut mediums: Vec<Medium> = Vec::new();

    let mut ways = 0;
    let mut nodes = 0;

    indexed_reader.read_ways_and_deps(
        |way| {
            // Filter ways. Return true if tags contain "highway" : "<V>"
            way.tags().any(|key_value| key_value.0 == "highway")
            // way.tags().any(|key_value| key_value == (("building", "yes)))
        },
        |element| {
            // Increment counter for ways and nodes
            match element {
                Element::Way(way) => {
                    ways += 1;
                    // For each way we create a medium 
                    // and populate it with nodes
                    let mut way_medium = Medium::new();
                    let mut way_one_way = false;
                    let mut med_positions = Vec::new();
                    let _ = way.node_locations().for_each(|n| {
                        let position = Position::from_way_node_location(n);
                        med_positions.push(position);
                    });
                    med_positions.push(Position { longitude: 0.0, latitude: 0.0 });
                    way_medium.medium_positions = med_positions;
                    let mut street_category = Vec::new();
                    let _ = way.tags().for_each(|(k,v)| {
                        if k == "highway" {
                            match v {
                                "residential"    => street_category.push(StreetCategory::Residential),
                                "service"        => street_category.push(StreetCategory::Service),
                                "track"          => street_category.push(StreetCategory::Track),
                                "footway"        => street_category.push(StreetCategory::Footway),
                                "unclassified"   => street_category.push(StreetCategory::Unclassified),
                                "path"           => street_category.push(StreetCategory::Path),
                                "crossing"       => street_category.push(StreetCategory::Crossing),
                                "tertiary"       => street_category.push(StreetCategory::Tertiary),
                                "secondary"      => street_category.push(StreetCategory::Secondary),
                                "primary"        => street_category.push(StreetCategory::Primary),
                                "living_street"  => street_category.push(StreetCategory::LivingStreet),
                                "cycleway"       => street_category.push(StreetCategory::Cycleway),
                                "trunk"          => street_category.push(StreetCategory::Trunk),
                                "motorway"       => street_category.push(StreetCategory::Motorway),
                                "motorway_link"  => street_category.push(StreetCategory::MotorwayLink),
                                "pedestrian"     => street_category.push(StreetCategory::Pedestrian),
                                "trunk_link"     => street_category.push(StreetCategory::TrunkLink),
                                "primary_link"   => street_category.push(StreetCategory::PrimaryLink),
                                "secondary_link" => street_category.push(StreetCategory::SecondaryLink),
                                "tertiary_link"  => street_category.push(StreetCategory::TertiaryLink),
                                "road"           => street_category.push(StreetCategory::Road),
                                _                => (),

                            }
                        }
                        else if k == "oneway" {
                            match v {
                                "yes" => way_one_way = true,
                                "no"  => way_one_way  = false,
                                _     => (),
                            }
                        }
                        else if k == "name" {
                            match v {
                                str => way_medium.medium_osm_name = Some(String::from(str))
                            }
                        }
                    });
                    way_medium.medium_type = MediumType::Highway(street_category);
                    way_medium.osm_id = Some(way.id());
                    way_medium.is_one_way = way_one_way;
                    mediums.push(way_medium);
                },
                Element::Node(_node) => nodes += 1,
                Element::DenseNode(_dense_node) => nodes += 1,
                Element::Relation(_) => {} // should not occur

            }
        },
    ).unwrap();

    // Timing
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
    println!("Finished counting in: {:#?}", duration);
    // Print result
    println!("ways:  {ways}\nnodes: {nodes}");
    println!("Created mediums: {:#?}", mediums.len());
    println!("Random medium type: {:#?}", mediums.get(0..50).unwrap());

}
pub mod types;

use std::{fmt::Debug, fs::File, io::{BufWriter, Write}, time::SystemTime, vec};

use osmpbf::{Element, ElementReader, IndexedReader};
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelExtend, ParallelIterator};
use types::medium::{Medium, MediumType, OsmNode, Position, StreetCategory};

fn main() {
    println!("Reading command line args");
    let arg1 = std::env::args_os()
        .nth(1)
        .expect("need a *.osm.pbf file as argument");
    let arg2 = std::env::args_os()
        .nth(2)
        .expect("Need a *.json file as an argument");
    let path: &std::path::Path = std::path::Path::new(&arg1);
    let out_file = std::path::Path::new(&arg2);

    println!("Reading OSM PBF File: {:#?}", path);
    let path_str: &str = "/hdd/Data/osm/kenya-latest.osm.pbf";
    // count_ways_kenya(path_str)
    // count_everything(path);
    // parse_all_to_medium(path);
    // par_vec_count_everything(path);
    par_parse_to_medium(path, out_file)
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

fn par_vec_count_everything(path: &std::path::Path) {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    println!("Counting...");
    match reader.par_map_reduce(
        |element|match element {
            Element::Node(_) | Element::DenseNode(_) => {
                (vec![1], Vec::new(), Vec::new())
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
                (Vec::new(), vec![1], Vec::new())
            },
            Element::Relation(_)=> (Vec::new(), Vec::new(), vec![1])
        }, // map_op,
        ||(Vec::new(),Vec::new(), Vec::new()), // identity,
        |mut a: (Vec<i32>, Vec<i32>, Vec<i32>), b: (Vec<i32>, Vec<i32>, Vec<i32>)|{
            a.0.extend(b.0);
            a.1.extend(b.1);
            a.2.extend(b.2);
            (a.0, a.1, a.2)
        },//reduce_op
    ) {
        Ok((nodes, ways, relations))=> {
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
            println!("Finished counting in: {:#?}", duration);
            let start_sum_time = SystemTime::now();
            let nodes_sum = nodes.iter().fold(0, |acc, n| acc + n);
            let ways_sum = ways.iter().fold(0, |acc, w| acc + w);
            let relations_sum = relations.iter().fold(0, |acc, r| acc + r);
            let end_sum_time = SystemTime::now();
            let sum_duration = end_sum_time.duration_since(start_sum_time).expect("Clock bad!");
            println!("Finished summing in: {:?}", sum_duration);
            println!("Nodes: {:?}", nodes_sum);
            println!("Ways: {:?}", ways_sum);
            println!("Relations: {:?}", relations_sum);
        }
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    }
}

fn par_parse_to_medium(path: &std::path::Path, out_file: &std::path::Path) {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    println!("Parsing to Medium... at{:?}", start_time);
    match reader.par_map_reduce(
        |element|match element {
            Element::Way(way) => {
                // For each way we create a medium
                // and populate it with nodes
                let mut way_medium = Medium::new();
                let mut way_one_way = false;
                let mut med_positions = Vec::new();
                let mut node_refs: Vec<i64> = Vec::new();
                // let _ = way.node_locations().for_each(|n| {
                //     let position = Position::from_way_node_location(n);
                //     med_positions.push(position);
                // });
                let _ = way.refs().for_each(|r|{
                    node_refs.push(r);
                });
                way_medium.osm_node_refs = node_refs;
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
                // mediums.push(way_medium);
                (vec![way_medium], 0, vec![], vec![])
            }
            Element::Relation(_r) => {
                (vec![], 1, vec![], vec![])
            }
            Element::Node(n) => {
                let osm_node = OsmNode::from_node(n); 
                (vec![], 0, vec![osm_node], vec![])
            }
            Element::DenseNode(n) => {
                let osm_node = OsmNode::from_dense_node(n);
                (vec![], 0, vec![], vec![osm_node])
            }

        }, // map_op,
        ||(vec![], 0u64, vec![], vec![]), // identity,
        |mut a, b|{
            a.0.extend(b.0);
            a.2.extend(b.2);
            a.3.extend(b.3);
            (a.0, a.1 + b.1, a.2, a.3)
        },//reduce_op
    ) {
        Ok((mut mediums, relations, mut nodes, node_densities))=> {
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
            let start_populating_med_pos = SystemTime::now();
            let node_densities_total = node_densities.len();
            nodes.extend(node_densities);
            let mut nodes_clone = nodes.clone();
            // nodes.par_iter().for_each(|n|{
            //     let _ = mediums.iter_mut().for_each(| m|{
            //         m.osm_node_refs.iter().for_each(|re|{
            //             if n.osm_id.eq(re) {
            //                 let pos = Position::from_osm_node(n);
            //                 m.medium_positions.push(pos);
            //             }
            //         })
            //     });
            // });
            let _ = mediums.par_iter_mut().for_each(|m|{
                let mut positions = Vec::new();
                if m.medium_osm_name.is_some() {
                    m.osm_node_refs.iter().for_each(|re|{    
                        // nodes.iter().for_each(|n|{
                        //     if n.osm_id.eq(re) {
                        //         let pos = Position::from_osm_node(n);
                    
                        //         println!("Created position {:#?}", pos);
                        //         // m.medium_positions.push(pos);
                        //         positions.push(pos);
                        //     }
                        // });
                        match nodes.par_iter().find_any(|n| n.osm_id.eq(re)) {
                            None => {
                                ()
                            }
                            Some(nn) => {
                                let pos = Position::from_osm_node(nn);
                                // println!("Created position {:#?}", pos);
                                positions.push(pos);
                            }
                        }
                    });
                    m.medium_positions = positions;
                }
            });
            let end_populating_med_pos = SystemTime::now();
            let duration_populating_med_pos = end_populating_med_pos.duration_since(start_populating_med_pos).expect("Bad time!");
            println!("Finished creating mediums in: {:#?}", duration);
            println!("Finished populating med positions in: {:?}", duration_populating_med_pos);
            println!("Created {:#?} Mediums", mediums.iter().len());
            println!("The nodes total: {:?}", nodes.len());
            println!("The node density total: {:?}", node_densities_total);
            println!("The relations total: {:?}", relations);
            println!("Random medium type: {:#?}", mediums.get(0..10).unwrap());
            println!("Writing medium results to json file");
            let start_writing_to_file = SystemTime::now();
            // let file = File::create("/hdd/Data/osm/osm-kovachs-medium-w-node-refs.json").unwrap();
            let file = File::create(out_file).unwrap(); // Unwrap!!!
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(& mut writer, &mediums).unwrap();
            writer.flush().unwrap();
            let end_writing_to_file = SystemTime::now();
            let duration_writing_to_file = end_writing_to_file.duration_since(start_writing_to_file).expect("Bad time!");
            println!("Finished writing to file in: {:#?}", duration_writing_to_file);
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
    // let mut nodes_vec = Vec::new();

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
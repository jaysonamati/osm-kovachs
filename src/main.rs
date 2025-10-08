pub mod data;
pub mod types;

use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufWriter, Write},
    ops::{Add, Deref},
    sync::{Arc, Mutex},
    time::SystemTime,
    vec,
};

use linya::{Bar, Progress};
use osmpbf::{Element, ElementReader, IndexedReader};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelExtend,
    ParallelIterator,
};
use types::medium::{Medium, MediumType, OsmNode, Position, StreetCategory};

fn main() {
    println!("Reading command line args");
    let arg1 = std::env::args_os()
        .nth(1)
        .expect("need a *.osm.pbf file as argument");
    let arg2 = std::env::args_os()
        .nth(2)
        .expect("Need a *.json file as an argument");
    let arg3 = std::env::args_os()
        .nth(3)
        .expect("Need a name of city as an argument");
    let path: &std::path::Path = std::path::Path::new(&arg1);
    let out_file = std::path::Path::new(&arg2);

    println!("Reading OSM PBF File: {:#?}", path);
    let path_str: &str = "/hdd/Data/osm/kenya-latest.osm.pbf";
    // count_ways_kenya(path_str)
    // count_everything(path);
    // parse_all_to_medium(path);
    // par_vec_count_everything(path);
    let mediums_w_refs = par_parse_to_medium(path, out_file);
    println!("Count for city: {:#?}", arg3);
    println!(
        "{}",
        format!("The number of mediums is: {}", mediums_w_refs.len())
    )
    //par_parse_to_medium_w_pos_hashmap(path, out_file, mediums_w_refs)
}

fn count_ways_kenya(path_str: &str) {
    let reader = ElementReader::from_path(path_str).unwrap();
    let mut ways = 0_u64;

    let total_ways = 7684884;
    let mut ways_bar = 0;

    let mut progress = Progress::new();

    // An owned handler to an internal bar.
    let bar = progress.bar(total_ways, "Processing osm...");

    // Increment the counter by one for each way.
    reader
        .for_each(|element| {
            if let Element::Way(w) = element {
                // eprintln!("{}", format!("Counting way: {}", w.id()));
                ways += 1;
                ways_bar += 1;
                progress.set_and_draw(&bar, ways_bar);
            }
        })
        .unwrap();

    println!("{ways}: ways in file: {path_str}");
}

fn count_everything(path: &std::path::Path) {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    println!("Counting...");

    let total_everything = 62020702;
    let mut everything_bar: Mutex<i64> = Mutex::new(0);

    let mut progress = Mutex::new(Progress::new());

    //let bar: Bar = progress
    //    .lock()
    //    .unwrap()
    //    .bar(50, format!("Downloading {}", n));

    // An owned handler to an internal bar.
    // let bar = progress.bar(total_everything, "Processing osm...");

    match reader.par_map_reduce(
        |element| {
            /*let bar: Bar = progress
                .lock()
                .unwrap()
                .bar(total_everything, "Processing osm");
            let everything_bar_acq = everything_bar.lock().unwrap().add(1);
            // everything_bar += 1;
            progress
                .lock()
                .unwrap()
                .set_and_draw(&bar, everything_bar_acq.try_into().unwrap());
            */
            match element {
                Element::Node(_) | Element::DenseNode(_) => (1, 0, 0),
                Element::Way(w) => {
                    if w.node_locations().len() < 2 {}
                    let mut keys = Vec::new();
                    let mut values = Vec::new();
                    let ways_iter = w.tags();
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
                    // everything_bar += 1;
                    //let everything_bar_acq = everything_bar.lock().unwrap().add(1);
                    //progress
                    //    .lock()
                    //    .unwrap()
                    //    .set_and_draw(&bar, everything_bar_acq.try_into().unwrap());

                    // println!("Way: {way_id} has tags of keys: {:#?} and values: {:#?}.", keys, values);
                    (0, 1, 0)
                }
                Element::Relation(_) => {
                    //let everything_bar_acq_i: i64 = everything_bar.lock().unwrap().add(1);
                    //progress
                    //    .lock()
                    //    .unwrap()
                    //    .set_and_draw(&bar, everything_bar_acq_i.try_into().unwrap());

                    (0, 0, 1)
                }
            }
        }, // map_op,
        || (0u64, 0u64, 0u64),                    // identity,
        |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2), //reduce_op
    ) {
        Ok((nodes, ways, relations)) => {
            let end_time = SystemTime::now();
            let duration = end_time
                .duration_since(start_time)
                .expect("Clock may have gone backwards");
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
        |element| match element {
            Element::Node(_) | Element::DenseNode(_) => (vec![1], Vec::new(), Vec::new()),
            Element::Way(w) => {
                if w.node_locations().len() < 2 {}
                let mut keys = Vec::new();
                let mut values = Vec::new();
                let ways_iter = w.tags();
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
            }
            Element::Relation(_) => (Vec::new(), Vec::new(), vec![1]),
        }, // map_op,
        || (Vec::new(), Vec::new(), Vec::new()), // identity,
        |mut a: (Vec<i32>, Vec<i32>, Vec<i32>), b: (Vec<i32>, Vec<i32>, Vec<i32>)| {
            a.0.extend(b.0);
            a.1.extend(b.1);
            a.2.extend(b.2);
            (a.0, a.1, a.2)
        }, //reduce_op
    ) {
        Ok((nodes, ways, relations)) => {
            let end_time = SystemTime::now();
            let duration = end_time
                .duration_since(start_time)
                .expect("Clock may have gone backwards");
            println!("Finished counting in: {:#?}", duration);
            let start_sum_time = SystemTime::now();
            let nodes_sum = nodes.iter().fold(0, |acc, n| acc + n);
            let ways_sum = ways.iter().fold(0, |acc, w| acc + w);
            let relations_sum = relations.iter().fold(0, |acc, r| acc + r);
            let end_sum_time = SystemTime::now();
            let sum_duration = end_sum_time
                .duration_since(start_sum_time)
                .expect("Clock bad!");
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
fn par_parse_to_medium_w_pos(
    path: &std::path::Path,
    out_file: &std::path::Path,
    mediums: Vec<Medium>,
) {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    let med_arc = Arc::new(Mutex::new(mediums));
    println!("Populating Mediums... at{:?}", start_time);
    match reader.par_map_reduce(
        |element| match element {
            Element::Way(_w) => (0, 0, 0),
            Element::Relation(_r) => (0, 0, 0),
            Element::Node(_n) => (0, 0, 0),
            Element::DenseNode(n) => {
                let node_id = n.id;
                match med_arc.lock() {
                    Err(e) => {
                        println!("{e}")
                    }
                    Ok(mut meds) => {
                        let _ = meds.iter_mut().enumerate().for_each(|(_i, m)| {
                            match m.osm_node_refs.par_iter().find_any(|nr| **nr == node_id) {
                                None => {}
                                Some(_nrr) => {
                                    let pos = Position::from_osm_node(&OsmNode::from_dense_node(
                                        n.clone(),
                                    ));
                                    m.medium_positions.push(pos);
                                }
                            }
                        });
                    }
                }
                (0, 0, 1)
            }
        },
        || (0u64, 0u64, 0u64),
        |a, b| (a.0, a.1, a.2 + b.2),
    ) {
        Ok((_, _, ns)) => {
            match med_arc.lock() {
                Err(e) => {
                    println!("Poisoned mutex {e}")
                }
                Ok(medss) => {
                    let end_time = SystemTime::now();
                    let duration = end_time
                        .duration_since(start_time)
                        .expect("Clock may have gone backwards");
                    println!("Random medium type: {:#?}", medss.get(0..10).unwrap());
                    println!("Processed {ns} dense nodes to medium positions");
                    println!("Finished populating mediums in: {:#?}", duration);
                    let start_writing_to_file = SystemTime::now();
                    // let file = File::create("/hdd/Data/osm/osm-kovachs-medium-w-node-refs.json").unwrap();
                    let file = File::create(out_file).unwrap(); // Unwrap!!!
                    let mut writer = BufWriter::new(file);
                    serde_json::to_writer(&mut writer, medss.deref()).unwrap();
                    writer.flush().unwrap();
                    let end_writing_to_file = SystemTime::now();
                    let duration_writing_to_file = end_writing_to_file
                        .duration_since(start_writing_to_file)
                        .expect("Bad time!");
                    println!(
                        "Finished writing to file in: {:#?}",
                        duration_writing_to_file
                    );
                }
            }
        }
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    }
}

fn par_parse_to_medium_w_pos_hashmap(
    path: &std::path::Path,
    out_file: &std::path::Path,
    mut mediums: Vec<Medium>,
) {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    //let med_arc = Arc::new(Mutex::new(mediums));
    println!("Populating Mediums... at{:?}", start_time);

    // Build the hashmap
    let node_to_medium_map: HashMap<i64, Vec<usize>> = {
        let mut map: HashMap<i64, Vec<usize>> = HashMap::new();
        for (medium_idx, medium) in mediums.iter().enumerate() {
            for &node_id in &medium.osm_node_refs {
                map.entry(node_id).or_default().push(medium_idx);
            }
        }
        map
    };

    // Process nodes in parallel, collecting positions per medium
    let positions_per_medium = match reader.par_map_reduce(
        |element| {
            let mut local_positions: HashMap<usize, Vec<Position>> = HashMap::new();
            if let Element::DenseNode(n) = element {
                let node_id = n.id;

                // Check if this node is referenced by any medium
                if let Some(medium_indices) = node_to_medium_map.get(&node_id) {
                    let pos = Position::from_osm_node(&OsmNode::from_dense_node(n.clone()));

                    // Add this position to all mediums that reference this node
                    for &medium_idx in medium_indices {
                        local_positions
                            .entry(medium_idx)
                            .or_default()
                            .push(pos.clone());
                    }
                }
            }

            local_positions
        },
        HashMap::new,
        |mut a, b| {
            // Merge the hashmaps
            for (medium_idx, mut positions) in b {
                a.entry(medium_idx).or_default().append(&mut positions);
            }
            a
        },
    ) {
        Ok(result) => result,
        Err(e) => {
            println!("{e}");
            std::process::exit(1)
        }
    };

    println!(
        "Collected positions for {} mediums",
        positions_per_medium.len()
    );

    // Apply collected positions to medium
    let mut total_nodes = 0;
    for (medium_idx, positions) in positions_per_medium {
        if let Some(medium) = mediums.get_mut(medium_idx) {
            total_nodes += positions.len();
            medium.medium_positions.extend(positions);
        }
    }

    let end_time = SystemTime::now();
    let duration = end_time
        .duration_since(start_time)
        .expect("Clock may have gone backwards");

    println!(
        "Random medium type: {:#?}",
        mediums.get(0..10.min(mediums.len()))
    );
    println!("Processed {} dense nodes to medium positions", total_nodes);
    println!("Finished populating mediums in: {:#?}", duration);

    let start_writing_to_file = SystemTime::now();
    let file = File::create(out_file).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, mediums.deref()).unwrap();
    writer.flush().unwrap();

    let end_writing_to_file = SystemTime::now();
    let duration_writing_to_file = end_writing_to_file
        .duration_since(start_writing_to_file)
        .expect("Bad time!");
    println!(
        "Finished writing to file in: {:#?}",
        duration_writing_to_file
    );
}

fn par_parse_to_medium(path: &std::path::Path, out_file: &std::path::Path) -> Vec<Medium> {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    println!("Parsing to Medium... at {:?}", start_time);

    // First pass: collect ways and build node->position map in parallel
    match reader.par_map_reduce(
        |element| match element {
            Element::Way(way) => {
                let mut way_medium = Medium::new();
                let mut way_one_way = false;
                let mut street_category = Vec::new();
                let mut node_refs: Vec<i64> = Vec::new();

                // Collect node references
                way.refs().for_each(|r| node_refs.push(r));
                way_medium.osm_node_refs = node_refs;

                // Parse tags
                way.tags().for_each(|(k, v)| match k {
                    "highway" => {
                        let category = match v {
                            "residential" => Some(StreetCategory::Residential),
                            "service" => Some(StreetCategory::Service),
                            "track" => Some(StreetCategory::Track),
                            "footway" => Some(StreetCategory::Footway),
                            "unclassified" => Some(StreetCategory::Unclassified),
                            "path" => Some(StreetCategory::Path),
                            "crossing" => Some(StreetCategory::Crossing),
                            "tertiary" => Some(StreetCategory::Tertiary),
                            "secondary" => Some(StreetCategory::Secondary),
                            "primary" => Some(StreetCategory::Primary),
                            "living_street" => Some(StreetCategory::LivingStreet),
                            "cycleway" => Some(StreetCategory::Cycleway),
                            "trunk" => Some(StreetCategory::Trunk),
                            "motorway" => Some(StreetCategory::Motorway),
                            "motorway_link" => Some(StreetCategory::MotorwayLink),
                            "pedestrian" => Some(StreetCategory::Pedestrian),
                            "trunk_link" => Some(StreetCategory::TrunkLink),
                            "primary_link" => Some(StreetCategory::PrimaryLink),
                            "secondary_link" => Some(StreetCategory::SecondaryLink),
                            "tertiary_link" => Some(StreetCategory::TertiaryLink),
                            "road" => Some(StreetCategory::Road),
                            _ => None,
                        };
                        if let Some(cat) = category {
                            street_category.push(cat);
                        }
                    }
                    "oneway" => {
                        way_one_way = v == "yes";
                    }
                    "name" => {
                        way_medium.medium_osm_name = Some(v.to_string());
                    }
                    _ => {}
                });

                way_medium.medium_type = MediumType::Highway(street_category);
                way_medium.osm_id = Some(way.id());
                way_medium.is_one_way = way_one_way;

                (vec![way_medium], HashMap::new())
            }
            Element::Node(n) => {
                let node_id = n.id();
                let pos = Position::from_osm_node(&OsmNode::from_node(n));
                let mut node_map = HashMap::new();
                node_map.insert(node_id, pos);
                (vec![], node_map)
            }
            Element::DenseNode(n) => {
                let node_id = n.id;
                let pos = Position::from_osm_node(&OsmNode::from_dense_node(n));
                let mut node_map = HashMap::new();
                node_map.insert(node_id, pos);
                (vec![], node_map)
            }
            Element::Relation(_) => (vec![], HashMap::new()),
        },
        || (vec![], HashMap::new()),
        |mut a, b| {
            // Merge mediums
            a.0.extend(b.0);
            // Merge node maps
            a.1.extend(b.1);
            a
        },
    ) {
        Ok((mut mediums, node_positions)) => {
            let end_parsing = SystemTime::now();
            let parsing_duration = end_parsing
                .duration_since(start_time)
                .expect("Clock may have gone backwards");

            println!("Finished parsing in: {:?}", parsing_duration);
            println!("Created {} mediums", mediums.len());
            println!("Collected {} node positions", node_positions.len());

            // Now populate medium positions using the node map
            let start_populating = SystemTime::now();

            mediums.par_iter_mut().for_each(|medium| {
                medium.medium_positions = medium
                    .osm_node_refs
                    .iter()
                    .filter_map(|node_ref| node_positions.get(node_ref).cloned())
                    .collect();
            });

            let end_populating = SystemTime::now();
            let populating_duration = end_populating
                .duration_since(start_populating)
                .expect("Bad time!");

            println!(
                "Finished populating positions in: {:?}",
                populating_duration
            );
            println!(
                "Random medium sample: {:#?}",
                mediums.get(0..10.min(mediums.len()))
            );

            // Write to file
            println!("Writing medium results to json file");
            let start_writing = SystemTime::now();

            let file = File::create(out_file).expect("Failed to create output file");
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, mediums.deref()).expect("Failed to write JSON");
            writer.flush().expect("Failed to flush writer");

            let end_writing = SystemTime::now();
            let writing_duration = end_writing
                .duration_since(start_writing)
                .expect("Bad time!");

            println!("Finished writing to file in: {:?}", writing_duration);

            let total_duration = end_writing.duration_since(start_time).expect("Bad time!");
            println!("Total time: {:?}", total_duration);

            mediums
        }
        Err(e) => {
            eprintln!("Error during parsing: {}", e);
            std::process::exit(1);
        }
    }
}

fn par_parse_to_medium_slow(path: &std::path::Path, out_file: &std::path::Path) -> Vec<Medium> {
    let start_time = SystemTime::now();
    let reader = ElementReader::from_path(path).unwrap();
    println!("Parsing to Medium... at{:?}", start_time);
    match reader.par_map_reduce(
        |element| match element {
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
                let _ = way.refs().for_each(|r| {
                    node_refs.push(r);
                });
                way_medium.osm_node_refs = node_refs;
                way_medium.medium_positions = med_positions;
                let mut street_category = Vec::new();
                let _ = way.tags().for_each(|(k, v)| {
                    if k == "highway" {
                        match v {
                            "residential" => street_category.push(StreetCategory::Residential),
                            "service" => street_category.push(StreetCategory::Service),
                            "track" => street_category.push(StreetCategory::Track),
                            "footway" => street_category.push(StreetCategory::Footway),
                            "unclassified" => street_category.push(StreetCategory::Unclassified),
                            "path" => street_category.push(StreetCategory::Path),
                            "crossing" => street_category.push(StreetCategory::Crossing),
                            "tertiary" => street_category.push(StreetCategory::Tertiary),
                            "secondary" => street_category.push(StreetCategory::Secondary),
                            "primary" => street_category.push(StreetCategory::Primary),
                            "living_street" => street_category.push(StreetCategory::LivingStreet),
                            "cycleway" => street_category.push(StreetCategory::Cycleway),
                            "trunk" => street_category.push(StreetCategory::Trunk),
                            "motorway" => street_category.push(StreetCategory::Motorway),
                            "motorway_link" => street_category.push(StreetCategory::MotorwayLink),
                            "pedestrian" => street_category.push(StreetCategory::Pedestrian),
                            "trunk_link" => street_category.push(StreetCategory::TrunkLink),
                            "primary_link" => street_category.push(StreetCategory::PrimaryLink),
                            "secondary_link" => street_category.push(StreetCategory::SecondaryLink),
                            "tertiary_link" => street_category.push(StreetCategory::TertiaryLink),
                            "road" => street_category.push(StreetCategory::Road),
                            _ => (),
                        }
                    } else if k == "oneway" {
                        match v {
                            "yes" => way_one_way = true,
                            "no" => way_one_way = false,
                            _ => (),
                        }
                    } else if k == "name" {
                        match v {
                            str => way_medium.medium_osm_name = Some(String::from(str)),
                        }
                    }
                });
                way_medium.medium_type = MediumType::Highway(street_category);
                way_medium.osm_id = Some(way.id());
                way_medium.is_one_way = way_one_way;
                // mediums.push(way_medium);
                (vec![way_medium], 0, vec![], vec![])
            }
            Element::Relation(_r) => (vec![], 1, vec![], vec![]),
            Element::Node(n) => {
                let osm_node = OsmNode::from_node(n);
                (vec![], 0, vec![osm_node], vec![])
            }
            Element::DenseNode(n) => {
                let osm_node = OsmNode::from_dense_node(n);
                (vec![], 0, vec![], vec![osm_node])
            }
        }, // map_op,
        || (vec![], 0u64, vec![], vec![]), // identity,
        |mut a, b| {
            a.0.extend(b.0);
            a.2.extend(b.2);
            a.3.extend(b.3);
            (a.0, a.1 + b.1, a.2, a.3)
        }, //reduce_op
    ) {
        Ok((mut mediums, relations, mut nodes, node_densities)) => {
            let end_time = SystemTime::now();
            let duration = end_time
                .duration_since(start_time)
                .expect("Clock may have gone backwards");
            let start_populating_med_pos = SystemTime::now();
            let medium_size = mediums.len();
            let node_densities_total = node_densities.len();
            nodes.par_extend(node_densities);
            // let mut nodes_clone = nodes.clone();
            let mut mediums_count = Arc::new(Mutex::new(0));
            let medium_count_down = Arc::new(Mutex::new(medium_size));
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
            // let _ = mediums.par_iter_mut().enumerate().for_each(|(i, m)| {
            //     let mut positions = Vec::new();
            //     if m.medium_osm_name.is_some() {
            //         m.osm_node_refs.iter().for_each(|re| {
            //             // nodes.iter().for_each(|n|{
            //             //     if n.osm_id.eq(re) {
            //             //         let pos = Position::from_osm_node(n);

            //             //         println!("Created position {:#?}", pos);
            //             //         // m.medium_positions.push(pos);
            //             //         positions.push(pos);
            //             //     }
            //             // });
            //             match nodes.par_iter().find_any(|n| n.osm_id.eq(re)) {
            //                 None => (),
            //                 Some(nn) => {
            //                     let pos = Position::from_osm_node(nn);
            //                     // println!("Created position {:#?}", pos);
            //                     positions.push(pos);
            //                     // m.medium_positions.push(pos);
            //                 }
            //             }
            //         });
            //         match mediums_count.lock() {
            //             Err(_) => (),
            //             Ok(mut mc) => {
            //                 *mc += 1;
            //             }
            //         };
            //         // println!("Added positions {:#?} to medium {:#?}", positions, m);
            //         m.medium_positions = positions;
            //     }
            //     match medium_count_down.lock() {
            //         Err(_) => (),
            //         Ok(mut mcd) => {
            //             *mcd -= 1;
            //         }
            //     }
            //     if i % 1000 == 0 {
            //         eprintln!("Medium with pos count: {:#?}", mediums_count);
            //         eprintln!(
            //             "Mediums left: {:#?} after {:#?}",
            //             medium_count_down,
            //             SystemTime::now()
            //                 .duration_since(start_populating_med_pos)
            //                 .expect("Bad time!")
            //         );
            //     }
            // });
            let end_populating_med_pos = SystemTime::now();
            let duration_populating_med_pos = end_populating_med_pos
                .duration_since(start_populating_med_pos)
                .expect("Bad time!");
            println!("Finished creating mediums in: {:#?}", duration);
            println!(
                "Finished populating med positions in: {:?}",
                duration_populating_med_pos
            );
            println!("Created {:#?} Mediums", mediums.iter().len());
            println!("The nodes total: {:?}", nodes.len());
            println!("The node density total: {:?}", node_densities_total);
            println!("The relations total: {:?}", relations);
            println!("Random medium type: {:#?}", mediums.get(0..10).unwrap());

            /*
                        println!("Writing medium results to json file");
                        let start_writing_to_file = SystemTime::now();
                        // let file = File::create("/hdd/Data/osm/osm-kovachs-medium-w-node-refs.json").unwrap();
                        let file = File::create(out_file).unwrap(); // Unwrap!!!
                        let mut writer = BufWriter::new(file);
                        serde_json::to_writer(&mut writer, mediums.deref()).unwrap();
                        writer.flush().unwrap();
                        let end_writing_to_file = SystemTime::now();
                        let duration_writing_to_file = end_writing_to_file
                            .duration_since(start_writing_to_file)
                            .expect("Bad time!");
                        println!(
                            "Finished writing to file in: {:#?}",
                            duration_writing_to_file
                        );
            */

            mediums
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

    indexed_reader
        .read_ways_and_deps(
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
                        med_positions.push(Position {
                            longitude: 0.0,
                            latitude: 0.0,
                        });
                        way_medium.medium_positions = med_positions;
                        let mut street_category = Vec::new();
                        let _ = way.tags().for_each(|(k, v)| {
                            if k == "highway" {
                                match v {
                                    "residential" => {
                                        street_category.push(StreetCategory::Residential)
                                    }
                                    "service" => street_category.push(StreetCategory::Service),
                                    "track" => street_category.push(StreetCategory::Track),
                                    "footway" => street_category.push(StreetCategory::Footway),
                                    "unclassified" => {
                                        street_category.push(StreetCategory::Unclassified)
                                    }
                                    "path" => street_category.push(StreetCategory::Path),
                                    "crossing" => street_category.push(StreetCategory::Crossing),
                                    "tertiary" => street_category.push(StreetCategory::Tertiary),
                                    "secondary" => street_category.push(StreetCategory::Secondary),
                                    "primary" => street_category.push(StreetCategory::Primary),
                                    "living_street" => {
                                        street_category.push(StreetCategory::LivingStreet)
                                    }
                                    "cycleway" => street_category.push(StreetCategory::Cycleway),
                                    "trunk" => street_category.push(StreetCategory::Trunk),
                                    "motorway" => street_category.push(StreetCategory::Motorway),
                                    "motorway_link" => {
                                        street_category.push(StreetCategory::MotorwayLink)
                                    }
                                    "pedestrian" => {
                                        street_category.push(StreetCategory::Pedestrian)
                                    }
                                    "trunk_link" => street_category.push(StreetCategory::TrunkLink),
                                    "primary_link" => {
                                        street_category.push(StreetCategory::PrimaryLink)
                                    }
                                    "secondary_link" => {
                                        street_category.push(StreetCategory::SecondaryLink)
                                    }
                                    "tertiary_link" => {
                                        street_category.push(StreetCategory::TertiaryLink)
                                    }
                                    "road" => street_category.push(StreetCategory::Road),
                                    _ => (),
                                }
                            } else if k == "oneway" {
                                match v {
                                    "yes" => way_one_way = true,
                                    "no" => way_one_way = false,
                                    _ => (),
                                }
                            } else if k == "name" {
                                match v {
                                    str => way_medium.medium_osm_name = Some(String::from(str)),
                                }
                            }
                        });
                        way_medium.medium_type = MediumType::Highway(street_category);
                        way_medium.osm_id = Some(way.id());
                        way_medium.is_one_way = way_one_way;
                        mediums.push(way_medium);
                    }
                    Element::Node(_node) => nodes += 1,
                    Element::DenseNode(_dense_node) => nodes += 1,
                    Element::Relation(_) => {} // should not occur
                }
            },
        )
        .unwrap();

    // Timing
    let end_time = SystemTime::now();
    let duration = end_time
        .duration_since(start_time)
        .expect("Clock may have gone backwards");
    println!("Finished counting in: {:#?}", duration);
    // Print result
    println!("ways:  {ways}\nnodes: {nodes}");
    println!("Created mediums: {:#?}", mediums.len());
    println!("Random medium type: {:#?}", mediums.get(0..50).unwrap());
}

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;

use crate::types::medium::Medium;

/// Load mediums from a JSON file
fn load_mediums_from_json(path: &Path) -> Result<Vec<Medium>, Box<dyn std::error::Error>> {
    let start_time = SystemTime::now();
    println!("Loading mediums from: {:?}", path);

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mediums: Vec<Medium> = serde_json::from_reader(reader)?;

    let end_time = SystemTime::now();
    let duration = end_time
        .duration_since(start_time)
        .expect("Clock may have gone backwards");

    println!("Loaded {} mediums in {:?}", mediums.len(), duration);
    println!(
        "Sample mediums: {:#?}",
        mediums.get(0..5.min(mediums.len()))
    );

    Ok(mediums)
}

/// Load mediums from JSON with unwrap (panics on error)
fn load_mediums_from_json_simple(path: &Path) -> Vec<Medium> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse JSON")
}

/// Example usage
fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    // Using the error-handling version
    let mediums = load_mediums_from_json(Path::new("mediums.json"))?;

    println!("Successfully loaded {} mediums", mediums.len());

    // Do something with mediums
    for medium in mediums.iter().take(10) {
        if let Some(name) = &medium.medium_osm_name {
            println!(
                "Medium: {} with {} positions",
                name,
                medium.medium_positions.len()
            );
        }
    }

    Ok(())
}

/// Alternative example with match
fn example_usage_with_match() {
    match load_mediums_from_json(Path::new("mediums.json")) {
        Ok(mediums) => {
            println!("Successfully loaded {} mediums", mediums.len());
            for medium in mediums.iter().take(10) {
                if let Some(name) = &medium.medium_osm_name {
                    println!(
                        "Medium: {} with {} positions",
                        name,
                        medium.medium_positions.len()
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading mediums: {}", e);
        }
    }
}

/// Load mediums with progress indicator for large files
fn load_mediums_with_progress(path: &Path) -> Result<Vec<Medium>, Box<dyn std::error::Error>> {
    use std::fs::metadata;

    let start_time = SystemTime::now();
    let file_size = metadata(path)?.len();
    println!(
        "Loading mediums from: {:?} ({} MB)",
        path,
        file_size / 1_000_000
    );

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    println!("Parsing JSON...");
    let mediums: Vec<Medium> = serde_json::from_reader(reader)?;

    let end_time = SystemTime::now();
    let duration = end_time
        .duration_since(start_time)
        .expect("Clock may have gone backwards");

    println!("✓ Loaded {} mediums in {:.2?}", mediums.len(), duration);

    // Print statistics
    let total_positions: usize = mediums.iter().map(|m| m.medium_positions.len()).sum();
    let named_mediums = mediums
        .iter()
        .filter(|m| m.medium_osm_name.is_some())
        .count();

    println!("  - Named mediums: {}", named_mediums);
    println!("  - Total positions: {}", total_positions);
    println!(
        "  - Avg positions per medium: {:.1}",
        total_positions as f64 / mediums.len() as f64
    );

    Ok(mediums)
}

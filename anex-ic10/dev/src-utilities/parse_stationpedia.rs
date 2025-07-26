use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("../../ic10lsp/stationpedia.txt")
        .expect("Failed to read stationpedia.txt");
    
    let mut device_name_to_hash: HashMap<String, i32> = HashMap::new();
    let mut hash_to_device_name: HashMap<i32, String> = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let hash_value: i32 = parts[0].parse().unwrap_or(0);
            let device_name = parts[1].trim().to_string();
            
            device_name_to_hash.insert(device_name.clone(), hash_value);
            hash_to_device_name.insert(hash_value, device_name);
        }
    }
    
    println!("Total entries parsed: {}", device_name_to_hash.len());
    
    // Count Structure devices
    let structure_count = device_name_to_hash.keys()
        .filter(|k| k.starts_with("Structure"))
        .count();
    println!("Structure devices: {}", structure_count);
    
    // Generate Rust-compatible mapping
    println!("\n=== DEVICE NAME TO HASH MAPPING ===");
    println!("use std::collections::HashMap;");
    println!("use phf::phf_map;\n");
    
    // Create a sorted list for consistent output
    let mut sorted_devices: Vec<_> = device_name_to_hash.iter().collect();
    sorted_devices.sort_by(|a, b| a.0.cmp(b.0));
    
    println!("pub const DEVICE_NAME_TO_HASH: phf::Map<&'static str, i32> = phf_map! {{");
    
    for (device_name, hash_value) in sorted_devices {
        // Escape special characters in device names
        let escaped_name = device_name.replace("\"", "\\\"");
        println!("    \"{}\" => {},", escaped_name, hash_value);
    }
    
    println!("}};");
    
    // Also create a focused mapping for Structure* devices
    println!("\n=== STRUCTURE DEVICES ONLY ===");
    let structure_devices: Vec<_> = device_name_to_hash.iter()
        .filter(|(k, _)| k.starts_with("Structure"))
        .collect();
    
    println!("pub const STRUCTURE_DEVICE_NAME_TO_HASH: phf::Map<&'static str, i32> = phf_map! {{");
    for (device_name, hash_value) in structure_devices {
        let escaped_name = device_name.replace("\"", "\\\"");
        println!("    \"{}\" => {},", escaped_name, hash_value);
    }
    println!("}};");
    
    // Create a reverse mapping (hash to device name)
    println!("\n=== HASH TO DEVICE NAME MAPPING ===");
    println!("pub const HASH_TO_DEVICE_NAME: phf::Map<i32, &'static str> = phf_map! {{");
    
    let mut sorted_hashes: Vec<_> = hash_to_device_name.iter().collect();
    sorted_hashes.sort_by(|a, b| a.0.cmp(b.0));
    
    for (hash_value, device_name) in sorted_hashes {
        let escaped_name = device_name.replace("\"", "\\\"");
        println!("    {} => \"{}\",", hash_value, escaped_name);
    }
    println!("}};");
}
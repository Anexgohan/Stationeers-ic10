use crate::hash_utils::*;

pub fn test_hash_lookup() {
    println!("=== Testing Hash Lookup ===");
    
    // Test HASH() function parsing
    let test_cases = [
        // Original test cases
        "HASH(\"StructureVolumePump\")",
        "HASH(\"StructureDaylightSensor\")",
        "HASH(\"Volume Pump\")",
        "HASH(\"Daylight Sensor\")",
        "HASH(\"StructurePipeAnalyzer\")",
        "HASH(\"StructurePipeAnalysizer\")", // typo test
        "HASH(\"StructureFiltration\")",
        "HASH(\"StructureDiodeSlide\")",
        "HASH(\"StructureGasTankStorage\")",
        "HASH(\"StructureGasMixer\")",
        // New comprehensive test cases
        "HASH(\"StructureAutolathe\")",
        "HASH(\"StructureFurnace\")",
        "HASH(\"StructureHydroponicsStation\")",
        "HASH(\"StructureLogicProcessor\")",
        "HASH(\"StructureSolarPanel\")",
        "HASH(\"StructureRecycler\")",
        "HASH(\"StructureElectrolyzer\")",
        "HASH(\"StructureAirConditioner\")",
    ];
    
    for test in test_cases {
        println!("\nTesting: {}", test);
        if let Some(device_name) = extract_hash_argument(test) {
            println!("  Extracted device name: {}", device_name);
            if let Some(hash_val) = get_device_hash(&device_name) {
                println!("  Hash value: {}", hash_val);
                if let Some(display_name) = get_device_name_for_hash(hash_val) {
                    println!("  Display name: {}", display_name);
                } else {
                    println!("  No display name found for hash {}", hash_val);
                }
            } else {
                println!("  Device name not found in registry");
            }
        } else {
            println!("  Failed to extract device name");
        }
    }
    
    // Test direct hash lookup
    println!("\n=== Testing Direct Hash Lookup ===");
    let direct_hashes = [-321403609, 1076425094];
    for hash in direct_hashes {
        println!("Hash {}: {:?}", hash, get_device_name_for_hash(hash));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_hash_lookup() {
        test_hash_lookup();
    }
}
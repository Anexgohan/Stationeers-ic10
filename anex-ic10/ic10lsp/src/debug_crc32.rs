use crc32fast::Hasher;

pub fn debug_crc32_values() {
    // Test various Volume Pump strings
    let test_strings = [
        "Volume Pump",
        "StructureVolumePump", 
        "ItemKitVolumePump",
        "StructureActivePump",
        "Daylight Sensor",
        "StructureDaylightSensor"
    ];
    
    println!("Expected: -321403609");
    for s in test_strings {
        let mut hasher = Hasher::new();
        hasher.update(s.as_bytes());
        let hash = hasher.finalize() as i32;
        println!("{}: {}", s, hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_crc32() {
        debug_crc32_values();
    }
}
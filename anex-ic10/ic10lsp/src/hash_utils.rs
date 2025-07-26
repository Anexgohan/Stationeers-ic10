use crc32fast::Hasher;
use crate::instructions::{PREFAB_TO_HASH, HASH_TO_DISPLAY};

/// Computes CRC32 hash for a given string using the same algorithm as Stationeers
pub fn compute_crc32(input: &str) -> i32 {
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    // Convert to signed 32-bit integer (Stationeers uses signed values)
    hash as i32
}

/// Extracts the device name from a HASH("device_name") function call
pub fn extract_hash_argument(input: &str) -> Option<String> {
    // Handle HASH("device_name") format
    let input = input.trim();
    
    // Must start with HASH(
    if !input.starts_with("HASH(") {
        return None;
    }
    
    // Must end with )
    if !input.ends_with(')') {
        return None;
    }
    
    // Extract content between HASH( and )
    let content = &input[5..input.len()-1].trim();
    
    // Handle quoted strings
    if content.len() >= 2 {
        let first_char = content.chars().next()?;
        let last_char = content.chars().last()?;
        
        if (first_char == '"' && last_char == '"') || (first_char == '\'' && last_char == '\'') {
            return Some(content[1..content.len()-1].to_string());
        }
    }
    
    // Handle unquoted strings (edge case)
    Some(content.to_string())
}

/// Checks if a string is a valid HASH() function call
pub fn is_hash_function_call(input: &str) -> bool {
    extract_hash_argument(input).is_some()
}

/// Looks up device name in device registry and returns the corresponding hash
pub fn get_device_hash(device_name: &str) -> Option<i32> {
    PREFAB_TO_HASH.get(device_name).copied()
}

/// Gets device name for a given hash value from the registry
pub fn get_device_name_for_hash(hash_value: i32) -> Option<&'static str> {
    HASH_TO_DISPLAY.get(&hash_value.to_string()).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_crc32() {
        assert_eq!(compute_crc32("StructureVolumePump"), -321403609);
        assert_eq!(compute_crc32("StructureDaylightSensor"), 1076425094);
    }

    #[test]
    fn test_extract_hash_argument() {
        assert_eq!(extract_hash_argument("HASH(\"StructureVolumePump\")"), Some("StructureVolumePump".to_string()));
        assert_eq!(extract_hash_argument("HASH('StructureVolumePump')"), Some("StructureVolumePump".to_string()));
        assert_eq!(extract_hash_argument("HASH(StructureVolumePump)"), Some("StructureVolumePump".to_string()));
        assert_eq!(extract_hash_argument("HASH(\"Volume Pump\")"), Some("Volume Pump".to_string()));
        assert_eq!(extract_hash_argument("not_hash"), None);
        assert_eq!(extract_hash_argument("HASH("), None);
    }

    #[test]
    fn test_get_device_hash() {
        assert_eq!(get_device_hash("StructureVolumePump"), Some(-321403609));
        assert_eq!(get_device_hash("StructureDaylightSensor"), Some(1076425094));
        assert_eq!(get_device_hash("NonExistentDevice"), None);
    }

    #[test]
    fn test_is_hash_function_call() {
        assert!(is_hash_function_call("HASH(\"StructureVolumePump\")"));
        assert!(is_hash_function_call("HASH('StructureDaylightSensor')"));
        assert!(is_hash_function_call("HASH(Volume Pump)"));
        assert!(!is_hash_function_call("define"));
        assert!(!is_hash_function_call("HASH"));
    }
}
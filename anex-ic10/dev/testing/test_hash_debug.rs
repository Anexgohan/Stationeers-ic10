fn main() {
    // Test the device mapping
    let device_name = "StructureVolumePump";
    let extracted = ic10lsp::hash_utils::extract_hash_argument(&format!("HASH(\"{}\")", device_name));
    println!("Extracted device name: {:?}", extracted);
    
    if let Some(name) = extracted {
        let hash = ic10lsp::hash_utils::get_device_hash(&name);
        println!("Device hash: {:?}", hash);
        
        if let Some(h) = hash {
            let display_name = ic10lsp::hash_utils::get_device_name_for_hash(h);
            println!("Display name: {:?}", display_name);
        }
    }
}
use std::{
    env,
    fs::{self, File},
    io::BufWriter,
    io::Write,
    path::Path,
    collections::HashMap,
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("stationpedia.rs");

    let mut map_builder = ::phf_codegen::Map::new();
    let mut set_builder = ::phf_codegen::Set::new();
    let mut prefab_to_hash_builder = ::phf_codegen::Map::new();
    let mut hash_to_display_builder = ::phf_codegen::Map::new();
    let mut check_set = std::collections::HashSet::new();
    
    // Store entries to avoid borrowing issues
    let mut prefab_entries: Vec<(String, String)> = Vec::new();
    let mut hash_entries: Vec<(String, String)> = Vec::new();

    let infile = Path::new("../dev/extractor/StationeersDataExtractor/output/stationpedia.txt");
    let contents = fs::read_to_string(infile).unwrap();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse format: "prefab_name" signed_hash hex_hash "display_name"
        // Find the positions of all quotes
        let quote_positions: Vec<usize> = line.match_indices('"').map(|(i, _)| i).collect();
        
        if quote_positions.len() >= 4 {
            // Extract the parts between quotes
            let prefab_name = &line[quote_positions[0] + 1..quote_positions[1]];
            let display_name = &line[quote_positions[2] + 1..quote_positions[3]];
            
            // Find the space-separated parts between the quoted sections
            let middle_part = &line[quote_positions[1] + 1..quote_positions[2]].trim();
            let middle_parts: Vec<&str> = middle_part.split_whitespace().collect();
            
            if middle_parts.len() >= 2 {
                let hash = middle_parts[0]; // signed_hash
                
                // Original mapping (hash -> display name)
                map_builder.entry(hash, &format!("\"{}\"", display_name));
                
                // Store entries for later processing
                let hash_int: i32 = hash.parse().unwrap_or(0);
                prefab_entries.push((prefab_name.to_string(), hash_int.to_string()));
                hash_entries.push((hash_int.to_string(), format!("\"{}\"", display_name)));

                if !check_set.contains(display_name) {
                    set_builder.entry(display_name);
                    check_set.insert(display_name);
                }
            }
        }
    }
    
    // Build the additional maps
    for (prefab, hash) in &prefab_entries {
        prefab_to_hash_builder.entry(prefab, hash);
    }
    
    for (hash, display) in &hash_entries {
        hash_to_display_builder.entry(hash, display);
    }

    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    write!(
        &mut writer,
        "pub(crate) const HASH_NAME_LOOKUP: phf::Map<&'static str, &'static str> = {};\n",
        map_builder.build()
    )
    .unwrap();

    write!(
        &mut writer,
        "pub(crate) const HASH_NAMES: phf::Set<&'static str> = {};\n",
        set_builder.build()
    )
    .unwrap();

    write!(
        &mut writer,
        "pub(crate) const PREFAB_TO_HASH: phf::Map<&'static str, i32> = {};\n",
        prefab_to_hash_builder.build()
    )
    .unwrap();

    write!(
        &mut writer,
        "pub(crate) const HASH_TO_DISPLAY: phf::Map<&'static str, &'static str> = {};\n",
        hash_to_display_builder.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=../dev/extractor/StationeersDataExtractor/output/stationpedia.txt");
}

use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("../extractor/StationeersDataExtractor/output/stationpedia.txt")
        .expect("Failed to read stationpedia.txt");
    
    let mut device_mappings: Vec<(String, i32, String)> = Vec::new(); // (structure_name, hash, display_name)
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let hash_value: i32 = parts[0].parse().unwrap_or(0);
            let display_name = parts[1].trim();
            
            // Create Structure* names for common devices
            let structure_name = match display_name {
                "Volume Pump" => Some("StructureVolumePump"),
                "Turbo Volume Pump (Gas)" => Some("StructureTurboVolumePump"),
                "Turbo Volume Pump (Liquid)" => Some("StructureTurboVolumePumpLiquid"),
                "Liquid Volume Pump" => Some("StructureLiquidVolumePump"),
                "Active Vent" => Some("StructureActiveVent"),
                "Daylight Sensor" => Some("StructureDaylightSensor"),
                "Gas Sensor" => Some("StructureGasSensor"),
                "Gas Mixer" => Some("StructureGasMixer"),
                "Filtration" => Some("StructureFiltration"),
                "Diode Slide" => Some("StructureDiodeSlide"),
                "Gas Tank Storage" => Some("StructureGasTankStorage"),
                "Liquid Tank Storage" => Some("StructureLiquidTankStorage"),
                "Pipe Analyzer" => Some("StructurePipeAnalyzer"),
                "Liquid Pipe Analyzer" => Some("StructureLiquidPipeAnalyzer"), 
                "LED Display (Large)" => Some("StructureConsoleLED1x3"),
                "LED Display (Medium)" => Some("StructureConsoleLED1x2"),
                "LED Display (Small)" => Some("StructureConsoleLED5"),
                "Harvie" => Some("StructureHarvie"),
                "Autolathe" => Some("StructureAutolathe"),
                "Furnace" => Some("StructureFurnace"),
                "Arc Furnace" => Some("StructureArcFurnace"),
                "Advanced Furnace" => Some("StructureAdvancedFurnace"),
                "Centrifuge" => Some("StructureCentrifuge"),
                "Combustion Centrifuge" => Some("StructureCombustionCentrifuge"),
                "Electrolyzer" => Some("StructureElectrolyzer"),
                "Gas Fuel Generator" => Some("StructureGasFuelGenerator"),
                "Generator (Solid Fuel)" => Some("StructureSolidFuelGenerator"),
                "Recycler" => Some("StructureRecycler"),
                "Ice Crusher" => Some("StructureIceCrusher"),
                "Hydroponics Station" => Some("StructureHydroponicsStation"),
                "Hydroponics Tray" => Some("StructureHydroponicsTray"),
                "Advanced Composter" => Some("StructureAdvancedComposter"),
                "Grow Light" => Some("StructureGrowLight"),
                "Airlock" => Some("StructureAirlockDoor"),
                "Advanced Airlock" => Some("StructureAdvancedAirlockDoor"),
                "Blast Door" => Some("StructureBlastDoor"),
                "Wall Light" => Some("StructureWallLight"),
                "Flashing Light" => Some("StructureFlashingLight"),
                "Air Conditioner" => Some("StructureAirConditioner"),
                "Wall Cooler" => Some("StructureWallCooler"),
                "Liquid Wall Cooler" => Some("StructureLiquidWallCooler"),
                "Digital Valve" => Some("StructureDigitalValve"),
                "Liquid Digital Valve" => Some("StructureLiquidDigitalValve"),
                "Back Pressure Regulator" => Some("StructureBackPressureRegulator"),
                "Liquid Volume Regulator" => Some("StructureLiquidVolumeRegulator"),
                "Igniter" => Some("StructureIgniter"),
                "Computer" => Some("StructureComputer"),
                "Console" => Some("StructureConsole"),
                "Logic Reader" => Some("StructureLogicReader"),
                "Logic Writer" => Some("StructureLogicWriter"),
                "Logic Memory" => Some("StructureLogicMemory"),
                "Logic Processor" => Some("StructureLogicProcessor"),
                "Logic Sorter" => Some("StructureLogicSorter"),
                "Logic Transmitter" => Some("StructureLogicTransmitter"),
                "Batch Reader" => Some("StructureBatchReader"),
                "Batch Writer" => Some("StructureBatchWriter"),
                "Sorter" => Some("StructureSorter"),
                "Stacker" => Some("StructureStacker"),
                "Chute Bin" => Some("StructureChuteBin"),
                "Chute Inlet" => Some("StructureChuteInlet"),
                "Chute Outlet" => Some("StructureChuteOutlet"),
                "Cable Analyzer" => Some("StructureCableAnalyzer"),
                "Transformer (Large)" => Some("StructureTransformerLarge"),
                "Area Power Control" => Some("StructureAreaPowerControl"),
                "Power Control" => Some("StructurePowerControl"),
                "Solar Panel" => Some("StructureSolarPanel"),
                "Solar Panel (Heavy Dual)" => Some("StructureSolarPanelHeavy"),
                "Radiator" => Some("StructureRadiator"),
                "Large Tank" => Some("StructureTankLarge"),
                "Insulated Liquid Tank Big" => Some("StructureTankLiquidInsulated"),
                "Tank Connector" => Some("StructureTankConnector"),
                "Insulated Tank Connector" => Some("StructureTankConnectorInsulated"),
                "Pressure Regulator" => Some("StructurePressureRegulator"),
                "Expansion Valve" => Some("StructureExpansionValve"),
                "Condensation Valve" => Some("StructureCondensationValve"),
                "Battery Cell (Large)" => Some("StructureBatteryLarge"),
                "Battery Cell Charger" => Some("StructureBatteryCharger"),
                "Beacon" => Some("StructureBeacon"),
                "Motion Sensor" => Some("StructureMotionSensor"),
                "Camera" => Some("StructureCamera"),
                "Camera Display" => Some("StructureCameraDisplay"),
                "Graph Display" => Some("StructureGraphDisplay"),
                "Gas Display" => Some("StructureGasDisplay"),
                "Hash Display" => Some("StructureHashDisplay"),
                "Weather Station" => Some("StructureWeatherStation"),
                _ => None,
            };
            
            if let Some(struct_name) = structure_name {
                device_mappings.push((struct_name.to_string(), hash_value, display_name.to_string()));
            }
        }
    }
    
    // Sort by structure name
    device_mappings.sort_by(|a, b| a.0.cmp(&b.0));
    
    println!("// Comprehensive Structure device mappings");
    println!("// Generated from stationpedia.txt - {} devices", device_mappings.len());
    println!();
    
    // Output forward mappings (Structure* -> hash)
    for (struct_name, hash_value, display_name) in &device_mappings {
        println!("    \"{}\" => {}, // {}", struct_name, hash_value, display_name);
    }
    
    println!();
    println!("// Reverse mappings (hash -> display name)");
    for (_, hash_value, display_name) in &device_mappings {
        println!("    {}i32 => \"{}\",", hash_value, display_name);
    }
    
    println!();
    println!("Total mappings created: {}", device_mappings.len());
}
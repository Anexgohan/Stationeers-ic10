use phf::phf_map;

/// Enhanced documentation for instruction hover tooltips
/// This module provides examples, categories, and related instruction mappings
/// for comprehensive hover documentation in the IC10 language server.

pub(crate) const INSTRUCTION_EXAMPLES: phf::Map<&'static str, &'static str> = phf_map! {
    "add" => "add r0 r1 r2      # Simple: r0 = r1 + r2\nadd r7 r5 r6      # Total charge from both batteries\nadd r10 r8 r9   # Total max power",
    "sub" => "sub r0 r1 r2      # Simple: r0 = r1 - r2\nsub currentRoomTemperature currentRoomTemperature 273.15\nsub temp temp 10  # temp = temp - 10",
    "mul" => "mul r0 r1 r2      # Simple: r0 = r1 * r2\nmul r3 r1 2       # PowerRequired in 1 second\nmul r15 r15 r14  # Temperature * TotalMoles",
    "div" => "div r0 r1 r2      # Simple: r0 = r1 / r2\ndiv timeRemaining r2 r3  # Power reserve in seconds\ndiv chargePercent totalCharge r10",
    "mod" => "mod r0 r1 r2      # r0 = r1 mod r2\nmod r0 timer 10   # r0 = timer mod 10",
    "move" => "move r0 100           # Simple: set r0 to 100\nmove targetPipePressure 5000  # Set to 5MPa\nmove fillVentBatch HASH(\"ActiveVentExhaust\")",
    "l" => "l r0 d0 Temperature     # Simple: read temperature from device 0\nl currentRoomPressure gasSensor Pressure\nl leverState01 leverSwitch01 Open",
    "s" => "s d1 Setting r0         # Simple: set device 1 setting to r0\ns pressureRegulator Setting targetPipePressure\ns db Setting currentRoomPressure",
    "lb" => "lb r0 HASH(\"StructureBattery\") Charge Average  # Simple: average battery charge\nlb r1 HASH(\"StructureGasSensor\") Pressure Sum\nlb totalPower HASH(\"StructureGasTurbine\") PowerGeneration Maximum",
    "sb" => "sb HASH(\"StructureHeater\") Setting r0        # Simple: set all heaters to r0\nsb HASH(\"StructureGasTurbine\") On r1\nsb heaterHASH Setting targetTemperature",
    "lbn" => "lbn r0 HASH(\"Sensor\") HASH(\"TempSensor\") Temperature 0\nlbn r0 PipeAnalyser AnalyserBeforePump Pressure 0\nlbn vertical DaylightSensor DaylightSensorClock Vertical 0",
    "sbn" => "sbn HASH(\"Pump\") HASH(\"MainPump\") On r0\nsbn TurboVolumePump TurboVolPumpIntake Setting r13\nsbn DisplayMedium DisplayHour Color hourColor",
    "ls" => "ls r0 d0 0 Occupied     # Simple: check if slot 0 occupied in d0\nls r1 sorter 1 Quantity\nls quantity conveyor 2 Damage",
    "lr" => "lr r0 d1 Contents Iron  # Simple: read iron content from d1\nlr r2 furnace Required Uranium\nlr totalMoles centrifuge Contents Volatiles",
    "beq" => "beq r0 r1 loop     # Simple: branch to 'loop' if r0 == r1\nbeq r12 r11 end    # Branch to 'end' if recipe matches\nbeq leverState01 0 InitialSetup",
    "bne" => "bne r0 r1 loop     # Simple: branch to 'loop' if r0 != r1\nbne r2 0 continue  # Continue if r2 is not zero\nbne temp 0 continue",
    "bgt" => "bgt r0 100 heating # Simple: branch if r0 > 100\nbgt r12 1 2        # Skip next instruction if ratio > 1\nbgt leverState02 0 FillRoomMode",
    "blt" => "blt r0 50 cooling  # Simple: branch if r0 < 50\nblt r15 8000 2     # Skip if fuel pressure low\nblt temp 273 heat_on",
    "bge" => "bge r0 r1 ready    # Simple: branch if r0 >= r1\nbge currentRoomPressure targetRoomPressure StopActiveVents\nbge pressure 1000 emergency",
    "ble" => "ble r0 10 low      # Simple: branch if r0 <= 10\nble r1 7 2         # Skip next 2 instructions\nble temp 373 normal",
    "beqz" => "beqz r0 off        # Simple: branch to 'off' if r0 == 0\nbeqz leverState01 InitialSetup  # Restart if lever off\nbeqz r7 end",
    "bnez" => "bnez r0 loop       # Simple: branch to 'loop' if r0 != 0\nbnez r1 active     # Jump to 'active' if r1 is not zero\nbnez power continue",
    "j" => "j loop             # Jump to 'loop' label\nj main             # Jump to 'main' label",
    "jal" => "jal subroutine     # Jump to subroutine, save return address\njal calculate_avg  # Call function",
    "jr" => "jr ra             # Return from subroutine\njr r15            # Jump to address in r15",
    "alias" => "alias temp r0        # Simple: name r0 as 'temp'\nalias sensor d0       # Simple: name d0 as 'sensor'\nalias currentRoomPressure r4  # Room pressure reading",
    "define" => "define MAX_TEMP 100  # Simple: define constant\ndefine ActiveVentHASH -1129453144  # Hash for ActiveVent devices\ndefine stationBattery HASH(\"StructureBattery\")  # Prefab Hash of Station Battery",
    "bdse" => "bdse d0 ready       # Simple: branch to 'ready' if d0 is set\nbdse sensor loop   # Branch to 'loop' if sensor is set\nbdse pump pump_active",
    "bdns" => "bdns d1 off         # Simple: branch to 'off' if d1 not set\nbdns pump shutdown # Branch to 'shutdown' if pump not set\nbdns furnace heat_off",
    "slt" => "slt r0 r1 r2       # Simple: r0 = 1 if r1 < r2\nslt r3 temp 100    # Check if temp < 100\nslt r10 temp mintemp",
    "sgt" => "sgt r0 r1 r2       # Simple: r0 = 1 if r1 > r2\nsgt r3 temp 200    # Check if temp > 200\nsgt r6 press maxpress",
    "seq" => "seq r0 r1 r2       # Simple: r0 = 1 if r1 == r2\nseq r3 temp 273    # Check if temp == 273\nseq ready power 1",
    "sne" => "sne r0 r1 r2       # Simple: r0 = 1 if r1 != r2\nsne r3 temp 0      # Check if temp != 0\nsne active error 0",
    "and" => "and r0 r1 r2      # Simple: r0 = 1 if both r1 and r2 not zero\nand r3 power heat  # Check both power and heat\nand ready sensor1 sensor2",
    "or" => "or r0 r1 r2       # Simple: r0 = 1 if r1 or r2 not zero\nor r3 error1 error2 # Check any error\nor alarm high_temp low_pressure",
    "sleep" => "sleep 1           # Wait 1 second between readings\nsleep 3           # Wait before checking vents again",
    "yield" => "yield             # Allow other scripts to run\nyield             # Pause execution for 1 tick",
    "sqrt" => "sqrt r0 r1        # Simple: r0 = square root of r1\nsqrt r2 r3         # Calculate square root\nsqrt distance r1  # distance = sqrt(r1)",
    "abs" => "abs r0 r1         # Simple: r0 = absolute value of r1\nabs r2 temperature # Get absolute temperature\nabs magnitude velocity",
    "sin" => "sin r0 r1         # Simple: r0 = sine of r1 (radians)\nsin r2 angle       # Calculate sine\nsin y_pos angle",
    "cos" => "cos r0 r1         # Simple: r0 = cosine of r1 (radians)\ncos r2 angle       # Calculate cosine\ncos x_pos angle",
    "select" => "select r0 r1 10 20       # Simple: r0 = 10 if r1!=0, else 20\nselect r2 isEast 1 0       # Choose 1 if east, 0 if west\nselect r9 playerOrigin 0 1   # Choose door index",
    "floor" => "floor r0 r1             # Simple: r0 = floor of r1\nfloor r2 temperature       # Round down temperature\nfloor minutes minutes     # Round down to integer",
    "round" => "round r0 r1             # Simple: r0 = rounded r1\nround r2 pressure          # Round pressure value\nround timeRemaining timeRemaining",
    "min" => "min r0 r1 r2             # Simple: r0 = minimum of r1 and r2\nmin r3 temp 100           # Limit temp to max 100\nmin r13 r13 MaxPumpSetting",
    "max" => "max r0 r1 r2             # Simple: r0 = maximum of r1 and r2\nmax r3 temp 0             # Ensure temp >= 0\nmax r13 r13 MinPumpSetting",
    "snez" => "snez r0 r1               # Simple: r0=1 if r1>0, else 0\nsnez r2 pressure          # Check if pressure exists\nsnez r14 r3                # Check if nitrogen detected",
    "bnezal" => "bnezal r0 subroutine     # Simple: call 'subroutine' if r0 != 0\nbnezal r2 ProcessData      # Call function if r2 is true\nbnezal condition ProcessGas",
    "trunc" => "trunc r0 r1             # Simple: r0 = integer part of r1\ntrunc r2 temperature       # Remove decimal from temp\ntrunc result calculation   # Integer portion only",
};

pub(crate) const INSTRUCTION_CATEGORIES: phf::Map<&'static str, &'static str> = phf_map! {
    "add" => "Arithmetic", "sub" => "Arithmetic", "mul" => "Arithmetic", "div" => "Arithmetic", "mod" => "Arithmetic",
    "abs" => "Arithmetic", "sqrt" => "Arithmetic", "round" => "Arithmetic", "trunc" => "Arithmetic", "ceil" => "Arithmetic", "floor" => "Arithmetic",
    "min" => "Arithmetic", "max" => "Arithmetic",
    "sin" => "Arithmetic", "cos" => "Arithmetic", "tan" => "Arithmetic", "asin" => "Arithmetic", "acos" => "Arithmetic", "atan" => "Arithmetic", "atan2" => "Arithmetic",
    "exp" => "Arithmetic", "log" => "Arithmetic", "rand" => "Arithmetic",
    "l" => "Device I/O", "s" => "Device I/O", "lr" => "Device I/O", "ls" => "Device I/O", "ld" => "Device I/O", "sd" => "Device I/O", "ss" => "Device I/O",
    "lb" => "Batch Operations", "sb" => "Batch Operations", "lbn" => "Batch Operations", "lbs" => "Batch Operations", "sbn" => "Batch Operations", "sbs" => "Batch Operations",
    "move" => "Register Operations", "select" => "Register Operations", "peek" => "Register Operations", "push" => "Register Operations", "pop" => "Register Operations",
    "get" => "Stack Operations", "getd" => "Stack Operations", "put" => "Stack Operations", "putd" => "Stack Operations", "poke" => "Stack Operations", "clr" => "Stack Operations", "clrd" => "Stack Operations",
    "slt" => "Comparison", "sgt" => "Comparison", "sle" => "Comparison", "sge" => "Comparison", "seq" => "Comparison", "sne" => "Comparison",
    "sltz" => "Comparison", "sgtz" => "Comparison", "slez" => "Comparison", "sgez" => "Comparison", "seqz" => "Comparison", "snez" => "Comparison",
    "sap" => "Comparison", "sna" => "Comparison", "sapz" => "Comparison", "snaz" => "Comparison", "snan" => "Comparison", "snanz" => "Comparison",
    "sdse" => "Device Status", "sdns" => "Device Status", "bdse" => "Device Status", "bdns" => "Device Status", "brdse" => "Device Status", "brdns" => "Device Status",
    "and" => "Logic", "or" => "Logic", "xor" => "Logic", "nor" => "Logic",
    "j" => "Control Flow", "jr" => "Control Flow", "jal" => "Control Flow",
    "beq" => "Branching", "bne" => "Branching", "blt" => "Branching", "bgt" => "Branching", "ble" => "Branching", "bge" => "Branching",
    "beqz" => "Branching", "bnez" => "Branching", "bltz" => "Branching", "bgtz" => "Branching", "blez" => "Branching", "bgez" => "Branching",
    "bap" => "Branching", "bna" => "Branching", "bapz" => "Branching", "bnaz" => "Branching", "bnan" => "Branching",
    "breq" => "Relative Branching", "brne" => "Relative Branching", "brlt" => "Relative Branching", "brgt" => "Relative Branching", "brle" => "Relative Branching", "brge" => "Relative Branching",
    "breqz" => "Relative Branching", "brnez" => "Relative Branching", "brltz" => "Relative Branching", "brgtz" => "Relative Branching", "brlez" => "Relative Branching", "brgez" => "Relative Branching",
    "brap" => "Relative Branching", "brna" => "Relative Branching", "brapz" => "Relative Branching", "brnaz" => "Relative Branching", "brnan" => "Relative Branching",
    "beqal" => "Branch and Link", "bneal" => "Branch and Link", "bltal" => "Branch and Link", "bgtal" => "Branch and Link", "bleal" => "Branch and Link", "bgeal" => "Branch and Link",
    "beqzal" => "Branch and Link", "bnezal" => "Branch and Link", "bltzal" => "Branch and Link", "bgtzal" => "Branch and Link", "blezal" => "Branch and Link", "bgezal" => "Branch and Link",
    "bapal" => "Branch and Link", "bnaal" => "Branch and Link", "bapzal" => "Branch and Link", "bnazal" => "Branch and Link", "bdseal" => "Branch and Link", "bdnsal" => "Branch and Link",
    "alias" => "Assembly", "define" => "Assembly", "label" => "Assembly",
    "sleep" => "Flow Control", "yield" => "Flow Control", "hcf" => "Flow Control",
    "rmap" => "Advanced", "hash" => "Advanced",
};

pub(crate) const RELATED_INSTRUCTIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "add" => &["sub", "mul", "div", "mod"],
    "sub" => &["add", "mul", "div", "mod"],
    "mul" => &["add", "sub", "div", "mod"],
    "div" => &["add", "sub", "mul", "mod"],
    "mod" => &["add", "sub", "mul", "div"],
    "l" => &["s", "lb", "sb", "lr", "ls", "ld", "sd"],
    "s" => &["l", "lb", "sb", "lr", "ls", "ld", "sd"],
    "lb" => &["l", "s", "sb", "lbn", "lbs", "sbn", "sbs"],
    "sb" => &["l", "s", "lb", "lbn", "lbs", "sbn", "sbs"],
    "ls" => &["l", "s", "lb", "sb", "lr"],
    "lr" => &["l", "s", "lb", "sb", "ls"],
    "beq" => &["bne", "blt", "bgt", "ble", "bge", "breq", "beqz"],
    "bne" => &["beq", "blt", "bgt", "ble", "bge", "brne", "bnez"],
    "blt" => &["beq", "bne", "bgt", "ble", "bge", "brlt", "bltz"],
    "bgt" => &["beq", "bne", "blt", "ble", "bge", "brgt", "bgtz"],
    "ble" => &["beq", "bne", "blt", "bgt", "bge", "brle", "blez"],
    "bge" => &["beq", "bne", "blt", "bgt", "ble", "brge", "bgez"],
    "beqz" => &["bnez", "bltz", "bgtz", "blez", "bgez", "beq"],
    "bnez" => &["beqz", "bltz", "bgtz", "blez", "bgez", "bne"],
    "j" => &["jr", "jal", "beq", "bne", "blt", "bgt"],
    "jr" => &["j", "jal", "breq", "brne", "brlt", "brgt"],
    "jal" => &["j", "jr", "beqal", "bneal", "bltal", "bgtal"],
    "slt" => &["sgt", "sle", "sge", "seq", "sne", "blt"],
    "sgt" => &["slt", "sle", "sge", "seq", "sne", "bgt"],
    "seq" => &["sne", "slt", "sgt", "sle", "sge", "beq"],
    "sne" => &["seq", "slt", "sgt", "sle", "sge", "bne"],
    "and" => &["or", "xor", "nor"],
    "or" => &["and", "xor", "nor"],
    "xor" => &["and", "or", "nor"],
    "nor" => &["and", "or", "xor"],
    "bdse" => &["bdns", "brdse", "brdns", "sdse", "sdns"],
    "bdns" => &["bdse", "brdse", "brdns", "sdse", "sdns"],
    "sqrt" => &["abs", "sin", "cos", "exp", "log"],
    "sin" => &["cos", "tan", "asin", "acos", "atan"],
    "cos" => &["sin", "tan", "asin", "acos", "atan"],
    "move" => &["select", "add", "sub", "l", "peek"],
    "sleep" => &["yield", "hcf"],
    "yield" => &["sleep", "hcf"],
    "alias" => &["define", "label"],
    "define" => &["alias", "label"],
    "lbn" => &["lb", "sbn", "sb", "l", "s"],
    "sbn" => &["sb", "lbn", "lb", "s", "l"],
    "min" => &["max", "add", "sub", "mul", "div"],
    "max" => &["min", "add", "sub", "mul", "div"],
    "floor" => &["round", "trunc", "ceil", "abs"],
    "round" => &["floor", "trunc", "ceil", "abs"],
    "trunc" => &["floor", "round", "ceil", "abs"],
    "snez" => &["seq", "sne", "slt", "sgt"],
    "select" => &["move", "beq", "bne", "and", "or"],
    "bnezal" => &["bnez", "jal", "beqal", "bneal"],
};

/// Helper functions for enhanced hover documentation
pub(crate) fn get_instruction_examples(instruction: &str) -> Option<&'static str> {
    INSTRUCTION_EXAMPLES.get(instruction).copied()
}

pub(crate) fn get_instruction_category(instruction: &str) -> Option<&'static str> {
    INSTRUCTION_CATEGORIES.get(instruction).copied()
}

pub(crate) fn get_related_instructions(instruction: &str) -> Option<&'static [&'static str]> {
    RELATED_INSTRUCTIONS.get(instruction).copied()
}

pub(crate) fn get_instruction_syntax(instruction: &str) -> String {
    if let Some(signature) = crate::instructions::INSTRUCTIONS.get(instruction) {
        let mut content = instruction.to_string();
        for parameter in signature.0 {
            content.push_str(&format!(" {parameter}"));
        }
        content
    } else {
        instruction.to_string()
    }
}

/// Create enhanced hover content for instructions with examples, syntax, and related commands
pub(crate) fn create_enhanced_instruction_hover(instruction: &str) -> Vec<tower_lsp::lsp_types::MarkedString> {
    use tower_lsp::lsp_types::{LanguageString, MarkedString};
    
    let mut hover_content = Vec::new();
    
    // Add instruction syntax
    let syntax = get_instruction_syntax(instruction);
    hover_content.push(MarkedString::LanguageString(LanguageString {
        language: "ic10".to_string(),
        value: syntax,
    }));
    
    // Build markdown content
    let mut markdown_parts = Vec::new();
    
    // Add instruction title and description
    if let Some(doc) = crate::instructions::INSTRUCTION_DOCS.get(instruction) {
        markdown_parts.push(format!("**{}**\n\n{}", instruction, doc));
    } else {
        markdown_parts.push(format!("**{}**", instruction));
    }
    
    // Add examples if available
    if let Some(examples) = get_instruction_examples(instruction) {
        markdown_parts.push(format!("**Examples:**\n```ic10\n{}\n```", examples));
    }
    
    // Add category and related instructions
    if let Some(category) = get_instruction_category(instruction) {
        markdown_parts.push(format!("**Category:** {}", category));
    }
    
    if let Some(related) = get_related_instructions(instruction) {
        if !related.is_empty() {
            let related_list = related.iter()
                .take(5) // Limit to avoid overwhelming the tooltip
                .map(|r| format!("`{}`", r))
                .collect::<Vec<_>>()
                .join(", ");
            markdown_parts.push(format!("**Related:** {}", related_list));
        }
    }
    
    // Add interactive links (these will be implemented in VS Code extension)
    markdown_parts.push("---".to_string());
    if let Some(category) = get_instruction_category(instruction) {
        markdown_parts.push(format!("📖 [View Related Instructions](command:ic10.showRelated?{})", instruction));
        markdown_parts.push(format!("🔍 [Search {} Operations](command:ic10.searchCategory?{})", category, category));
    }
    
    // Combine all markdown parts
    let markdown_content = markdown_parts.join("\n\n");
    hover_content.push(MarkedString::String(markdown_content));
    
    hover_content
}
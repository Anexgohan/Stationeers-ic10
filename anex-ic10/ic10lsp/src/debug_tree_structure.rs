use tree_sitter::{Parser, Query, QueryCursor};

pub fn debug_tree_structure() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_ic10::language()).unwrap();
    
    // Test with sample define statements
    let test_code = r#"define Pump HASH("StructureVolumePump")
define Pump2 -321403609
define Test HASH("StructureDaylightSensor")"#;
    
    let tree = parser.parse(test_code, None).unwrap();
    
    println!("=== Tree Structure Debug ===");
    print_node(&tree.root_node(), test_code.as_bytes(), 0);
    
    println!("\n=== Testing Define Query ===");
    let define_query = Query::new(tree_sitter_ic10::language(), "(instruction (operation \"define\"))@x").unwrap();
    let mut cursor = QueryCursor::new();
    
    for (capture, _) in cursor.captures(&define_query, tree.root_node(), test_code.as_bytes()) {
        let node = capture.captures[0].node;
        println!("Found define instruction at {:?}", node.range());
        
        // Print all children
        let mut child_cursor = node.walk();
        for child in node.children(&mut child_cursor) {
            println!("  Child: {} at {:?}", child.kind(), child.range());
            
            if child.kind() == "operand" {
                println!("    Operand found, checking children:");
                let mut operand_cursor = child.walk();
                for operand_child in child.children(&mut operand_cursor) {
                    println!("      {}: '{}'", operand_child.kind(), 
                           operand_child.utf8_text(test_code.as_bytes()).unwrap_or("<error>"));
                }
            }
        }
    }
}

fn print_node(node: &tree_sitter::Node, source: &[u8], depth: usize) {
    let indent = "  ".repeat(depth);
    let text = node.utf8_text(source).unwrap_or("<error>");
    let text_preview = if text.len() > 50 { 
        format!("{}...", &text[..47]) 
    } else { 
        text.to_string() 
    };
    
    println!("{}{}: '{}'", indent, node.kind(), text_preview.replace('\n', "\\n"));
    
    if depth < 4 { // Limit depth to avoid too much output
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            print_node(&child, source, depth + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_tree_structure() {
        debug_tree_structure();
    }
}
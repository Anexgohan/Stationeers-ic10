use std::collections::HashMap;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};
use tree_sitter::{Query, QueryCursor, Tree};

use crate::Range;

#[derive(Debug, Clone)]
pub struct RegisterUsage {
    pub assignments: Vec<Range>,  // Where register is assigned values
    pub reads: Vec<Range>,        // Where register is read/used
    pub alias_name: Option<String>, // If register has an alias
}

#[derive(Debug, Clone)]
pub enum RegisterState {
    Unused,           // Never assigned or read
    AssignedNotRead,  // Assigned but value never used
    ReadBeforeAssign, // Read before any assignment (error)
    Used,            // Properly assigned and read
}

impl RegisterUsage {
    pub fn new() -> Self {
        Self {
            assignments: Vec::new(),
            reads: Vec::new(),
            alias_name: None,
        }
    }
    
    pub fn get_state(&self) -> RegisterState {
        if self.assignments.is_empty() && self.reads.is_empty() {
            RegisterState::Unused
        } else if !self.assignments.is_empty() && self.reads.is_empty() {
            RegisterState::AssignedNotRead
        } else if self.assignments.is_empty() && !self.reads.is_empty() {
            RegisterState::ReadBeforeAssign
        } else {
            // Check if any reads happen before first assignment
            // Sort assignments and reads by line number to ensure proper ordering
            let mut assignments_by_line: Vec<_> = self.assignments.iter().map(|r| r.0.start.line).collect();
            let mut reads_by_line: Vec<_> = self.reads.iter().map(|r| r.0.start.line).collect();
            
            assignments_by_line.sort();
            reads_by_line.sort();
            
            if let (Some(&first_assignment_line), Some(&first_read_line)) = (assignments_by_line.first(), reads_by_line.first()) {
                if first_read_line < first_assignment_line {
                    RegisterState::ReadBeforeAssign
                } else {
                    RegisterState::Used
                }
            } else {
                RegisterState::Used
            }
        }
    }
}

pub struct RegisterAnalyzer {
    register_usage: HashMap<String, RegisterUsage>,
}

impl RegisterAnalyzer {
    pub fn new() -> Self {
        Self {
            register_usage: HashMap::new(),
        }
    }

    pub fn analyze_register_usage(&mut self, tree: &Tree, content: &str, aliases: &HashMap<String, crate::DefinitionData<crate::AliasValue>>) {
        self.register_usage.clear();
        
        // Initialize all known registers
        for reg in ["r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "ra", "sp"] {
            self.register_usage.insert(reg.to_string(), RegisterUsage::new());
        }
        
        // Add aliased registers
        for (alias_name, alias_data) in aliases {
            if let crate::AliasValue::Register(reg_name) = &alias_data.value {
                if let Some(usage) = self.register_usage.get_mut(reg_name) {
                    usage.alias_name = Some(alias_name.clone());
                }
            }
        }
        
        self.detect_register_assignments(tree, content, aliases);
        self.detect_register_reads(tree, content, aliases);
        self.detect_jal_ra_assignments(tree, content);
    }
    
    fn detect_register_assignments(&mut self, tree: &Tree, content: &str, aliases: &HashMap<String, crate::DefinitionData<crate::AliasValue>>) {
        let mut cursor = QueryCursor::new();
        
        // Query for all instructions and manually check the first operand
        let instruction_query = Query::new(
            tree_sitter_ic10::language(),
            "(instruction (operation) @op) @instruction",
        ).unwrap();
        
        let op_idx = instruction_query.capture_index_for_name("op").unwrap();
        let instruction_idx = instruction_query.capture_index_for_name("instruction").unwrap();
        
        for (capture, _) in cursor.captures(&instruction_query, tree.root_node(), content.as_bytes()) {
            let mut operation = None;
            let mut instruction_node = None;
            
            for cap in capture.captures {
                if cap.index == op_idx {
                    operation = Some(cap.node.utf8_text(content.as_bytes()).unwrap());
                } else if cap.index == instruction_idx {
                    instruction_node = Some(cap.node);
                }
            }
            
            if let (Some(op), Some(inst_node)) = (operation, instruction_node) {
                if self.is_assignment_operation(op) {
                    // Get the first operand (target for assignment operations)
                    let mut tree_cursor = inst_node.walk();
                    let operands: Vec<_> = inst_node.children_by_field_name("operand", &mut tree_cursor).collect();
                    
                    if let Some(first_operand) = operands.first() {
                        if let Some(operand_child) = first_operand.child(0) {
                            match operand_child.kind() {
                                "register" => {
                                    let reg_name = operand_child.utf8_text(content.as_bytes()).unwrap();
                                    if let Some(usage) = self.register_usage.get_mut(reg_name) {
                                        usage.assignments.push(Range::from(operand_child.range()));
                                    }
                                }
                                "identifier" => {
                                    let identifier = operand_child.utf8_text(content.as_bytes()).unwrap();
                                    if let Some(alias_data) = aliases.get(identifier) {
                                        if let crate::AliasValue::Register(reg_name) = &alias_data.value {
                                            if let Some(usage) = self.register_usage.get_mut(reg_name) {
                                                usage.assignments.push(Range::from(operand_child.range()));
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn detect_register_reads(&mut self, tree: &Tree, content: &str, aliases: &HashMap<String, crate::DefinitionData<crate::AliasValue>>) {
        let mut cursor = QueryCursor::new();
        
        // Query for all instructions and manually check operands
        let instruction_query = Query::new(
            tree_sitter_ic10::language(),
            "(instruction (operation) @op) @instruction",
        ).unwrap();
        
        let op_idx = instruction_query.capture_index_for_name("op").unwrap();
        let instruction_idx = instruction_query.capture_index_for_name("instruction").unwrap();
        
        for (capture, _) in cursor.captures(&instruction_query, tree.root_node(), content.as_bytes()) {
            let mut operation = None;
            let mut instruction_node = None;
            
            for cap in capture.captures {
                if cap.index == op_idx {
                    operation = Some(cap.node.utf8_text(content.as_bytes()).unwrap());
                } else if cap.index == instruction_idx {
                    instruction_node = Some(cap.node);
                }
            }
            
            if let (Some(op), Some(inst_node)) = (operation, instruction_node) {
                let mut tree_cursor = inst_node.walk();
                let operands: Vec<_> = inst_node.children_by_field_name("operand", &mut tree_cursor).collect();
                
                // For assignment operations, skip the first operand (target)
                // For other operations, all operands are potential reads
                let start_idx = if self.is_assignment_operation(op) { 1 } else { 0 };
                
                for operand in operands.into_iter().skip(start_idx) {
                    if let Some(operand_child) = operand.child(0) {
                        match operand_child.kind() {
                            "register" => {
                                let reg_name = operand_child.utf8_text(content.as_bytes()).unwrap();
                                if let Some(usage) = self.register_usage.get_mut(reg_name) {
                                    usage.reads.push(Range::from(operand_child.range()));
                                }
                            }
                            "identifier" => {
                                let identifier = operand_child.utf8_text(content.as_bytes()).unwrap();
                                if let Some(alias_data) = aliases.get(identifier) {
                                    if let crate::AliasValue::Register(reg_name) = &alias_data.value {
                                        if let Some(usage) = self.register_usage.get_mut(reg_name) {
                                            usage.reads.push(Range::from(operand_child.range()));
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    fn detect_jal_ra_assignments(&mut self, tree: &Tree, content: &str) {
        let mut cursor = QueryCursor::new();
        
        // Query for jal instructions that implicitly assign to ra
        let jal_query = Query::new(
            tree_sitter_ic10::language(),
            "(instruction (operation \"jal\") @jal) @instruction",
        ).unwrap();
        
        let jal_idx = jal_query.capture_index_for_name("jal").unwrap();
        
        for (capture, _) in cursor.captures(&jal_query, tree.root_node(), content.as_bytes()) {
            for cap in capture.captures {
                if cap.index == jal_idx {
                    // jal implicitly assigns the return address to ra
                    if let Some(usage) = self.register_usage.get_mut("ra") {
                        usage.assignments.push(Range::from(cap.node.range()));
                    }
                }
            }
        }
    }

    fn is_assignment_operation(&self, operation: &str) -> bool {
        // Operations that assign to their first register operand
        matches!(operation, 
            "move" | "add" | "sub" | "mul" | "div" | "mod" | "max" | "min" |
            "abs" | "ceil" | "floor" | "round" | "sqrt" | "trunc" | "exp" | "log" |
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "atan2" |
            "and" | "or" | "xor" | "nor" | "not" | "sla" | "sll" | "sra" | "srl" |
            "l" | "lb" | "lr" | "ls" | "lbn" | "lbs" | "lbns" | "lhz" | "lhs" |
            "peek" | "pop" | "sap" | "sapz" |
            "sdns" | "sdse" | "select" | "seq" | "seqz" | "sge" | "sgez" |
            "sgt" | "sgtz" | "sle" | "slez" | "slt" | "sltz" | "sna" | "snaz" |
            "sne" | "snez" | "rget" | "alias"
        )
    }
    
    pub fn generate_diagnostics(&self) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (register_name, usage) in &self.register_usage {
            match usage.get_state() {
                RegisterState::Unused => {
                    // Only warn about unused aliases, not bare registers
                    if let Some(alias_name) = &usage.alias_name {
                        // We don't have the alias definition range here, would need to pass it
                        // This will be handled when integrating with main diagnostics
                    }
                }
                RegisterState::AssignedNotRead => {
                    for assignment_range in &usage.assignments {
                        let register_display = usage.alias_name.as_ref()
                            .map(|alias| format!("'{}' ({})", alias, register_name))
                            .unwrap_or_else(|| register_name.clone());
                            
                        diagnostics.push(Diagnostic::new(
                            assignment_range.clone().into(),
                            Some(DiagnosticSeverity::WARNING),
                            None,
                            None,
                            format!("Register {} is assigned but never read. Consider removing to optimize register usage.", register_display),
                            None,
                            None,
                        ));
                    }
                }
                RegisterState::ReadBeforeAssign => {
                    for read_range in &usage.reads {
                        let register_display = usage.alias_name.as_ref()
                            .map(|alias| format!("'{}' ({})", alias, register_name))
                            .unwrap_or_else(|| register_name.clone());
                            
                        diagnostics.push(Diagnostic::new(
                            read_range.clone().into(),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            format!("Register {} is read before being assigned a value.", register_display),
                            None,
                            None,
                        ));
                    }
                }
                RegisterState::Used => {
                    // No diagnostic needed for properly used registers
                }
            }
        }
        
        diagnostics
    }
    
    pub fn get_register_usage(&self) -> &HashMap<String, RegisterUsage> {
        &self.register_usage
    }
}
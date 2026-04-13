use crate::languages::LanguageProfile;

pub struct TypeScriptProfile;
pub struct TsxProfile;

fn extract_function_name(node: &arborium::tree_sitter::Node, source: &[u8]) -> Option<String> {
    node.child_by_field_name("name")
        .and_then(|n| n.utf8_text(source).ok())
        .map(|s| s.to_string())
}

fn is_method(node: &arborium::tree_sitter::Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "class_body" {
            return true;
        }
        current = parent.parent();
    }
    false
}

impl LanguageProfile for TypeScriptProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_declaration", "method_definition"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "for_in_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
            "catch_clause",
            "else_clause",
            "ternary_expression",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "for_in_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["arrow_function", "function_expression"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["comment"]
    }

    fn extract_function_name(
        &self,
        node: &arborium::tree_sitter::Node,
        source: &[u8],
    ) -> Option<String> {
        extract_function_name(node, source)
    }

    fn parser_language(&self) -> arborium::tree_sitter::Language {
        arborium::lang_typescript::language().into()
    }

    fn extensions(&self) -> &[&str] {
        &[".ts", ".mts", ".cts"]
    }

    fn is_method(&self, node: &arborium::tree_sitter::Node) -> bool {
        is_method(node)
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["switch_statement"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["switch_case", "switch_default"]
    }
}

impl LanguageProfile for TsxProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_declaration", "method_definition"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "for_in_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
            "catch_clause",
            "else_clause",
            "ternary_expression",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "for_in_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["arrow_function", "function_expression"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["comment"]
    }

    fn extract_function_name(
        &self,
        node: &arborium::tree_sitter::Node,
        source: &[u8],
    ) -> Option<String> {
        extract_function_name(node, source)
    }

    fn parser_language(&self) -> arborium::tree_sitter::Language {
        arborium::lang_tsx::language().into()
    }

    fn extensions(&self) -> &[&str] {
        &[".tsx"]
    }

    fn is_method(&self, node: &arborium::tree_sitter::Node) -> bool {
        is_method(node)
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["switch_statement"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["switch_case", "switch_default"]
    }
}

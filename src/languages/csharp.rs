use crate::languages::LanguageProfile;

pub struct CSharpProfile;

impl LanguageProfile for CSharpProfile {
    fn function_nodes(&self) -> &[&str] {
        &[
            "method_declaration",
            "local_function_statement",
            "constructor_declaration",
        ]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "foreach_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
            "catch_clause",
            "conditional_expression",
            "else",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "foreach_statement",
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
        &["lambda_expression"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["comment"]
    }

    fn extract_function_name(
        &self,
        node: &arborium::tree_sitter::Node,
        source: &[u8],
    ) -> Option<String> {
        node.child_by_field_name("name")
            .and_then(|n| n.utf8_text(source).ok())
            .map(|s| s.to_string())
    }

    fn parser_language(&self) -> arborium::tree_sitter::Language {
        arborium::lang_c_sharp::language().into()
    }

    fn extensions(&self) -> &[&str] {
        &[".cs"]
    }

    fn is_method(&self, node: &arborium::tree_sitter::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "declaration_list" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn call_nodes(&self) -> &[&str] {
        &["invocation_expression"]
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["switch_statement"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["switch_section"]
    }
}

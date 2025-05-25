#[cfg(test)]
mod tests {
    use super::*;
    use crate::neuro_symbolic::symbolic::{
        Fact, KnowledgeBase, QueryResult, Rule, SimpleKnowledgeBase, Term, SymbolicError
    };

    #[test]
    fn test_term_display() {
        let variable = Term::Variable("X".to_string());
        let constant = Term::Constant("john".to_string());
        
        assert_eq!(format!("{}", variable), "?X");
        assert_eq!(format!("{}", constant), "john");
    }

    #[test]
    fn test_fact_creation() {
        let fact = Fact::new(
            "person",
            vec![Term::Constant("john".to_string())]
        );
        
        assert_eq!(fact.predicate, "person");
        assert_eq!(fact.arguments.len(), 1);
        assert_eq!(format!("{}", fact), "person(john)");
    }

    #[test]
    fn test_rule_creation() {
        let head = Fact::new(
            "parent",
            vec![
                Term::Variable("X".to_string()),
                Term::Variable("Y".to_string())
            ]
        );
        
        let body = vec![
            Fact::new(
                "father",
                vec![
                    Term::Variable("X".to_string()),
                    Term::Variable("Y".to_string())
                ]
            )
        ];
        
        let rule = Rule::new(head, body);
        
        assert_eq!(format!("{}", rule), "parent(?X, ?Y) :- father(?X, ?Y)");
    }

    #[test]
    fn test_simple_knowledge_base() {
        let mut kb = SimpleKnowledgeBase::new();
        
        // Add facts
        let fact1 = Fact::new(
            "person",
            vec![Term::Constant("john".to_string())]
        );
        
        let fact2 = Fact::new(
            "person",
            vec![Term::Constant("mary".to_string())]
        );
        
        let fact3 = Fact::new(
            "father",
            vec![
                Term::Constant("john".to_string()),
                Term::Constant("bob".to_string())
            ]
        );
        
        kb.add_fact(fact1).unwrap();
        kb.add_fact(fact2).unwrap();
        kb.add_fact(fact3).unwrap();
        
        // Test query
        let result = kb.query("person(?X)").unwrap();
        assert!(result.success);
        assert_eq!(result.bindings.len(), 2);
        
        // Test clear
        kb.clear().unwrap();
        assert_eq!(kb.facts.len(), 0);
        assert_eq!(kb.rules.len(), 0);
    }

    #[test]
    fn test_query_result() {
        let mut bindings1 = std::collections::HashMap::new();
        bindings1.insert("X".to_string(), Term::Constant("john".to_string()));
        
        let mut bindings2 = std::collections::HashMap::new();
        bindings2.insert("X".to_string(), Term::Constant("mary".to_string()));
        
        let result = QueryResult::success(vec![bindings1, bindings2]);
        
        assert!(result.success);
        assert_eq!(result.bindings.len(), 2);
        
        let failure = QueryResult::failure();
        assert!(!failure.success);
        assert_eq!(failure.bindings.len(), 0);
    }

    #[test]
    fn test_symbolic_error() {
        let parse_error = SymbolicError::ParseError("Invalid syntax".to_string());
        let query_error = SymbolicError::QueryError("Query failed".to_string());
        
        assert_eq!(
            format!("{}", parse_error),
            "Parse error: Invalid syntax"
        );
        assert_eq!(
            format!("{}", query_error),
            "Query error: Query failed"
        );
    }

    #[test]
    fn test_unify_terms() {
        let kb = SimpleKnowledgeBase::new();
        let mut bindings = std::collections::HashMap::new();
        
        // Constant-Constant match
        assert!(kb.unify_terms(
            &Term::Constant("john".to_string()),
            &Term::Constant("john".to_string()),
            &mut bindings
        ));
        
        // Constant-Constant mismatch
        assert!(!kb.unify_terms(
            &Term::Constant("john".to_string()),
            &Term::Constant("mary".to_string()),
            &mut bindings
        ));
        
        // Variable-Constant binding
        assert!(kb.unify_terms(
            &Term::Variable("X".to_string()),
            &Term::Constant("john".to_string()),
            &mut bindings
        ));
        
        // Check binding
        assert_eq!(
            bindings.get("X"),
            Some(&Term::Constant("john".to_string()))
        );
        
        // Variable with existing binding
        assert!(kb.unify_terms(
            &Term::Variable("X".to_string()),
            &Term::Constant("john".to_string()),
            &mut bindings
        ));
        
        // Variable with conflicting binding
        assert!(!kb.unify_terms(
            &Term::Variable("X".to_string()),
            &Term::Constant("mary".to_string()),
            &mut bindings
        ));
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}

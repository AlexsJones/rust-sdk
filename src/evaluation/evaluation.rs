use std::collections::HashMap;

#[derive(Clone)]
pub struct EvaluationContext {
    pub targeting_key: String,
    pub attributes: HashMap<String, String>,
}

pub struct EvaluationOptions {}

pub type FlattenedContext = HashMap<String, String>;

pub fn flatten_context(context: EvaluationContext) -> FlattenedContext {
    let mut flattened_context = HashMap::new();

    if !context.attributes.is_empty() {
        flattened_context = context.attributes();
    }
    if !context.targeting_key().is_empty() {
        flattened_context.insert(
            context.targeting_key(),
            context.attribute(context.targeting_key()),
        );
    }
    flattened_context
}
// impl
impl EvaluationContext {
    pub fn new(targeting_key: String, attributes: HashMap<String, String>) -> Self {
        Self {
            targeting_key,
            attributes,
        }
    }
    fn attribute(&self, key: String) -> String {
        if !self.attributes.contains_key(&key) {
            return "".to_string();
        }
        self.attributes.get(&key).unwrap().clone()
    }
    fn targeting_key(&self) -> String {
        self.targeting_key.clone()
    }
    fn attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::evaluation::{flatten_context, EvaluationContext};

    #[test]
    fn test_evaluation_context_1() {
        let mut attributes = HashMap::new();
        attributes.insert("key1".to_string(), "value1".to_string());
        attributes.insert("key2".to_string(), "value2".to_string());
        attributes.insert("key3".to_string(), "value3".to_string());

        let context = EvaluationContext::new("targeting_key".to_string(), attributes);

        assert_eq!(context.targeting_key(), "targeting_key".to_string());
        assert_eq!(context.attribute("key1".to_string()), "value1".to_string());
        assert_eq!(context.attribute("key2".to_string()), "value2".to_string());
        assert_eq!(context.attribute("key3".to_string()), "value3".to_string());
        assert_eq!(context.attributes().len(), 3);
    }
    #[test]
    fn test_empty_evaluation_context_2() {
        let attributes = HashMap::new();

        let context = EvaluationContext::new("targeting_key".to_string(), attributes);

        assert_eq!(context.targeting_key(), "targeting_key".to_string());
        assert_eq!(context.attribute("key1".to_string()), "".to_string());
        assert_eq!(context.attribute("key2".to_string()), "".to_string());
        assert_eq!(context.attribute("key3".to_string()), "".to_string());
        assert_eq!(context.attributes().len(), 0);
    }
    #[test]
    fn test_flatten_context() {
        let mut attributes = HashMap::new();
        attributes.insert("key1".to_string(), "value1".to_string());
        attributes.insert("key2".to_string(), "value2".to_string());
        attributes.insert("key3".to_string(), "value3".to_string());

        let context = EvaluationContext::new("targeting_key".to_string(), attributes);

        let flattened_context = flatten_context(context);

        assert_eq!(flattened_context.len(), 4);
        assert_eq!(
            flattened_context.get("key1").unwrap().to_string(),
            "value1".to_string()
        );
        assert_eq!(
            flattened_context.get("key2").unwrap().to_string(),
            "value2".to_string()
        );
        assert_eq!(
            flattened_context.get("key3").unwrap().to_string(),
            "value3".to_string()
        );
    }
}

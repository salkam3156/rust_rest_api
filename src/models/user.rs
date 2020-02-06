use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct User {
    name: String,
    id: u32,
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    #[test]
    fn should_serialize_deserialize_object() {
        let test_model = User {
            name: String::from("Test Model"),
            id: 32,
        };

        let serialized_model = serde_json::to_string(&test_model).unwrap();
        let deserialized_model: User = serde_json::from_str(&serialized_model).unwrap();

        assert_ne!(
            test_model,
            User {
                name: String::from("Not Test Model"),
                id: 32
            }
        );

        assert_eq!(test_model, deserialized_model);
    }
}

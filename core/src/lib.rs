use serde::{Deserialize, Serialize};

/// SampleValue
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SampleValue {
    SampleA,
    SampleB,
    Other(String),
}

#[test]
fn use_database() {
    let v = SampleValue::SampleA;
    assert_eq!(serde_json::to_string(&v).unwrap(), "\"SampleA\"")
}

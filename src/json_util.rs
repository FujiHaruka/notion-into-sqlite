use serde_json::Value;

#[derive(Debug)]
pub enum JsonKey<'a> {
    String(&'a str),
    Index(usize),
}
impl<'a> From<&'a str> for JsonKey<'a> {
    fn from(s: &'a str) -> Self {
        JsonKey::String(s)
    }
}
impl From<usize> for JsonKey<'_> {
    fn from(i: usize) -> Self {
        JsonKey::Index(i)
    }
}

pub fn dig_json<'a>(source: &'a Value, keys: &Vec<JsonKey>) -> Option<&'a Value> {
    let mut value = source;
    for key in keys {
        value = match key {
            &JsonKey::String(k) => value.as_object()?.get(k)?,
            &JsonKey::Index(index) => value.as_array()?.get(index.clone())?,
        }
    }
    Some(value)
}

mod tests {
    use super::*;

    #[test]
    fn test_dig_json() {
        let data = serde_json::from_str::<Value>("{}").unwrap();
        let keys: Vec<JsonKey> = vec!["foo".into(), "foo".into(), 1.into()];
        assert!(dig_json(&data, &keys).is_none());

        let data = serde_json::from_str::<Value>(
            r#"{
            "foo": {
                "bar": [
                    {
                        "id": "xxx"
                    }
                ]
            }
        }"#,
        )
        .unwrap();
        let keys: Vec<JsonKey> = vec!["foo".into(), "bar".into(), 0.into(), "id".into()];
        assert_eq!(dig_json(&data, &keys).unwrap(), "xxx");
    }
}

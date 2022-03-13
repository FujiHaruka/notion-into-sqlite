use serde_json::Value;

pub enum JsonKey<'a> {
    String(&'a str),
    Index(usize),
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
        let keys = vec![JsonKey::String("foo"), JsonKey::Index(1)];
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
        let keys = vec![
            JsonKey::String("foo"),
            JsonKey::String("bar"),
            JsonKey::Index(0),
            JsonKey::String("id"),
        ];
        assert_eq!(dig_json(&data, &keys).unwrap(), "xxx");
    }
}

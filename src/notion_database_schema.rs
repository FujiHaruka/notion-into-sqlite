use serde_json::Value;

#[derive(Debug)]
pub enum NotionPropertyType {
    Title,
    Number,
    Select,
    Other,
}

#[derive(Debug)]
pub struct NotionProperty {
    pub name: String,
    pub property_type: NotionPropertyType,
    pub property_raw_type: String,
}

#[derive(Debug)]
pub struct NotionDatabaseSchema {
    pub properties: Vec<NotionProperty>,
}

pub fn parse_database_schema(database_resp: &Value) -> Option<NotionDatabaseSchema> {
    let raw_properties = database_resp.as_object()?.get("properties")?.as_object()?;
    let properties = raw_properties
        .keys()
        .filter_map(|key| {
            let property = raw_properties.get(key)?.as_object()?;
            let name = property.get("name")?.as_str()?;
            let property_raw_type = property.get("type")?.as_str()?;
            let property_type = match property_raw_type {
                "title" => NotionPropertyType::Title,
                "select" => NotionPropertyType::Select,
                "number" => NotionPropertyType::Number,
                _ => NotionPropertyType::Other,
            };
            Some(NotionProperty {
                name: name.to_string(),
                property_raw_type: property_raw_type.to_string(),
                property_type,
            })
        })
        .collect::<Vec<NotionProperty>>();

    Some(NotionDatabaseSchema { properties })
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_database_schema() {
        let data = r#"
        {
            "object": "database",
            "id": "f2bf4cd7-b8d1-44fc-856e-8fe60c128b58",
            "cover": null,
            "icon": null,
            "created_time": "2022-03-12T00:15:00.000Z",
            "created_by": {
                "object": "user",
                "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
            },
            "last_edited_by": {
                "object": "user",
                "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
            },
            "last_edited_time": "2022-03-12T00:20:00.000Z",
            "title": [
                {
                    "type": "text",
                    "text": {
                        "content": "Animals",
                        "link": null
                    },
                    "annotations": {
                        "bold": false,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "Animals",
                    "href": null
                }
            ],
            "properties": {
                "Age": {
                    "id": "GPCK",
                    "name": "Age",
                    "type": "number",
                    "number": {
                        "format": "number"
                    }
                },
                "Animal": {
                    "id": "wzVU",
                    "name": "Animal",
                    "type": "select",
                    "select": {
                        "options": [
                            {
                                "id": "67fe1cf3-29f8-4cb7-9517-803e1d975e86",
                                "name": "cat",
                                "color": "green"
                            },
                            {
                                "id": "18ce9dcd-b7e1-4511-ad35-9420c0399e13",
                                "name": "dog",
                                "color": "orange"
                            }
                        ]
                    }
                },
                "Name": {
                    "id": "title",
                    "name": "Name",
                    "type": "title",
                    "title": {}
                }
            }
        }
        "#;
        let json = serde_json::from_str(data).unwrap();
        let schema = super::parse_database_schema(&json).unwrap();
        assert_eq!(schema.properties.len(), 3);
    }
}

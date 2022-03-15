#[allow(dead_code)]
pub static NOTION_DATABASE_JSON: &str = r#"
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

#[allow(dead_code)]
pub static NOTION_DATABASE_IRREGULAR_JSON: &str = r#"
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
      "あ&\";#' f　_": {
          "id": "GPCK",
          "name": "あ&\";#' f　_",
          "type": "number",
          "number": {
              "format": "number"
          }
      }
  }
}
"#;

#[allow(dead_code)]
pub static NOTION_LIST_JSON: &str = r#"
{
    "object": "list",
    "results": [
      {
        "object": "page",
        "id": "a75b9220-455d-48e1-a36b-c581a345f777",
        "created_time": "2022-03-12T00:15:00.000Z",
        "last_edited_time": "2022-03-12T00:16:00.000Z",
        "created_by": {
          "object": "user",
          "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
        },
        "last_edited_by": {
          "object": "user",
          "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
        },
        "cover": null,
        "icon": null,
        "parent": {
          "type": "database_id",
          "database_id": "f2bf4cd7-b8d1-44fc-856e-8fe60c128b58"
        },
        "archived": false,
        "properties": {
          "Age": {
            "id": "GPCK",
            "type": "number",
            "number": 10
          },
          "Animal": {
            "id": "wzVU",
            "type": "select",
            "select": {
              "id": "67fe1cf3-29f8-4cb7-9517-803e1d975e86",
              "name": "cat",
              "color": "green"
            }
          },
          "Name": {
            "id": "title",
            "type": "title",
            "title": [
              {
                "type": "text",
                "text": {
                  "content": "Meu",
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
                "plain_text": "Meu",
                "href": null
              }
            ]
          }
        },
        "url": "https://www.notion.so/Meu-a75b9220455d48e1a36bc581a345f777"
      }
    ],
    "next_cursor": "e6c9af10-44ec-4a48-a969-156ba5438ff0",
    "has_more": true,
    "type": "page",
    "page": {}
}
"#;

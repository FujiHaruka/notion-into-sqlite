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
pub static NOTION_DATABASE_ALL_TYPES_JSON: &str = r#"
{
  "object": "database",
  "id": "8a281474-f071-4c54-8afc-17d8a4b7c782",
  "cover": null,
  "icon": null,
  "created_time": "2022-03-21T02:39:00.000Z",
  "created_by": {
    "object": "user",
    "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
  },
  "last_edited_by": {
    "object": "user",
    "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
  },
  "last_edited_time": "2022-03-21T03:37:00.000Z",
  "title": [
    {
      "type": "text",
      "text": {
        "content": "All types",
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
      "plain_text": "All types",
      "href": null
    }
  ],
  "properties": {
    "Rollup": {
      "id": "%40%3FJb",
      "name": "Rollup",
      "type": "rollup",
      "rollup": {
        "rollup_property_name": "Age",
        "relation_property_name": "Relation",
        "rollup_property_id": "GPCK",
        "relation_property_id": "knfs",
        "function": "show_original"
      }
    },
    "URL": {
      "id": "CaK%5B",
      "name": "URL",
      "type": "url",
      "url": {}
    },
    "Select": {
      "id": "DSOY",
      "name": "Select",
      "type": "select",
      "select": {
        "options": [
          {
            "id": "4c10699d-c938-4267-b91e-105a84d2c2e2",
            "name": "option",
            "color": "orange"
          }
        ]
      }
    },
    "Phone": {
      "id": "IZjW",
      "name": "Phone",
      "type": "phone_number",
      "phone_number": {}
    },
    "LastEditedTime": {
      "id": "O%3E%3B%7D",
      "name": "LastEditedTime",
      "type": "last_edited_time",
      "last_edited_time": {}
    },
    "MultiSelect": {
      "id": "Pim~",
      "name": "MultiSelect",
      "type": "multi_select",
      "multi_select": {
        "options": [
          {
            "id": "e1cb1dee-ff1a-47ac-81c9-7ce174a9e448",
            "name": "multi",
            "color": "default"
          },
          {
            "id": "017e0af6-6e94-4f09-a43e-ce2131c81baa",
            "name": "select",
            "color": "brown"
          }
        ]
      }
    },
    "CreatedTime": {
      "id": "RRxv",
      "name": "CreatedTime",
      "type": "created_time",
      "created_time": {}
    },
    "Number": {
      "id": "SAc%3F",
      "name": "Number",
      "type": "number",
      "number": {
        "format": "number"
      }
    },
    "LastEditedBy": {
      "id": "SWLu",
      "name": "LastEditedBy",
      "type": "last_edited_by",
      "last_edited_by": {}
    },
    "Formula": {
      "id": "TzKS",
      "name": "Formula",
      "type": "formula",
      "formula": {
        "expression": "pi"
      }
    },
    "Files": {
      "id": "b%5Be%3F",
      "name": "Files",
      "type": "files",
      "files": {}
    },
    "Relation": {
      "id": "knfs",
      "name": "Relation",
      "type": "relation",
      "relation": {
        "database_id": "f2bf4cd7-b8d1-44fc-856e-8fe60c128b58",
        "synced_property_name": "Related to All types (Relation)",
        "synced_property_id": "JpOZ"
      }
    },
    "Date": {
      "id": "oVB%5B",
      "name": "Date",
      "type": "date",
      "date": {}
    },
    "RichText": {
      "id": "ozQm",
      "name": "RichText",
      "type": "rich_text",
      "rich_text": {}
    },
    "CreatedBy": {
      "id": "ppm%5E",
      "name": "CreatedBy",
      "type": "created_by",
      "created_by": {}
    },
    "Checkbox": {
      "id": "q%40Di",
      "name": "Checkbox",
      "type": "checkbox",
      "checkbox": {}
    },
    "Email": {
      "id": "zEAy",
      "name": "Email",
      "type": "email",
      "email": {}
    },
    "People": {
      "id": "%7BK%5D%60",
      "name": "People",
      "type": "people",
      "people": {}
    },
    "Name": {
      "id": "title",
      "name": "Name",
      "type": "title",
      "title": {}
    }
  },
  "parent": {
    "type": "page_id",
    "page_id": "5256af6e-80cc-4c63-a6f2-6fc9e4166239"
  },
  "url": "https://www.notion.so/8a281474f0714c548afc17d8a4b7c782",
  "archived": false
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

#[allow(dead_code)]
pub static NOTION_LIST_ALL_TYPES_JSON: &str = r#"
{
  "object": "list",
  "results": [
    {
      "object": "page",
      "id": "ce4593d9-0cfb-4659-8012-12594b723312",
      "created_time": "2022-03-21T02:39:00.000Z",
      "last_edited_time": "2022-03-21T03:36:00.000Z",
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
        "database_id": "8a281474-f071-4c54-8afc-17d8a4b7c782"
      },
      "archived": false,
      "properties": {
        "Rollup": {
          "id": "%40%3FJb",
          "type": "rollup",
          "rollup": {
            "type": "array",
            "array": [
              {
                "type": "number",
                "number": 10
              }
            ],
            "function": "show_original"
          }
        },
        "URL": {
          "id": "CaK%5B",
          "type": "url",
          "url": "https://example.com"
        },
        "Select": {
          "id": "DSOY",
          "type": "select",
          "select": {
            "id": "4c10699d-c938-4267-b91e-105a84d2c2e2",
            "name": "option",
            "color": "orange"
          }
        },
        "Phone": {
          "id": "IZjW",
          "type": "phone_number",
          "phone_number": "09000000000"
        },
        "LastEditedTime": {
          "id": "O%3E%3B%7D",
          "type": "last_edited_time",
          "last_edited_time": "2022-03-21T03:36:00.000Z"
        },
        "MultiSelect": {
          "id": "Pim~",
          "type": "multi_select",
          "multi_select": [
            {
              "id": "e1cb1dee-ff1a-47ac-81c9-7ce174a9e448",
              "name": "multi",
              "color": "default"
            },
            {
              "id": "017e0af6-6e94-4f09-a43e-ce2131c81baa",
              "name": "select",
              "color": "brown"
            }
          ]
        },
        "CreatedTime": {
          "id": "RRxv",
          "type": "created_time",
          "created_time": "2022-03-21T02:39:00.000Z"
        },
        "Number": {
          "id": "SAc%3F",
          "type": "number",
          "number": 10
        },
        "LastEditedBy": {
          "id": "SWLu",
          "type": "last_edited_by",
          "last_edited_by": {
            "object": "user",
            "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
          }
        },
        "Formula": {
          "id": "TzKS",
          "type": "formula",
          "formula": {
            "type": "number",
            "number": 3.14159265359
          }
        },
        "Files": {
          "id": "b%5Be%3F",
          "type": "files",
          "files": [
            {
              "name": "icon.png",
              "type": "file",
              "file": {
                "url": "https://example.com/file",
                "expiry_time": "2022-03-21T05:31:48.908Z"
              }
            }
          ]
        },
        "Relation": {
          "id": "knfs",
          "type": "relation",
          "relation": [
            {
              "id": "a75b9220-455d-48e1-a36b-c581a345f777"
            }
          ]
        },
        "Date": {
          "id": "oVB%5B",
          "type": "date",
          "date": {
            "start": "2022-03-20",
            "end": null,
            "time_zone": null
          }
        },
        "RichText": {
          "id": "ozQm",
          "type": "rich_text",
          "rich_text": [
            {
              "type": "text",
              "text": {
                "content": "rich text ",
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
              "plain_text": "rich text ",
              "href": null
            },
            {
              "type": "text",
              "text": {
                "content": "link",
                "link": {
                  "url": "https://example.com"
                }
              },
              "annotations": {
                "bold": false,
                "italic": false,
                "strikethrough": false,
                "underline": false,
                "code": false,
                "color": "default"
              },
              "plain_text": "link",
              "href": "https://example.com"
            }
          ]
        },
        "CreatedBy": {
          "id": "ppm%5E",
          "type": "created_by",
          "created_by": {
            "object": "user",
            "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
          }
        },
        "Checkbox": {
          "id": "q%40Di",
          "type": "checkbox",
          "checkbox": true
        },
        "Email": {
          "id": "zEAy",
          "type": "email",
          "email": "someone@example.com"
        },
        "People": {
          "id": "%7BK%5D%60",
          "type": "people",
          "people": [
            {
              "object": "user",
              "id": "9d069f8b-6223-4853-b7eb-8fe3dfe7d389"
            }
          ]
        },
        "Name": {
          "id": "title",
          "type": "title",
          "title": [
            {
              "type": "text",
              "text": {
                "content": "name",
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
              "plain_text": "name",
              "href": null
            }
          ]
        }
      },
      "url": "https://www.notion.so/name-ce4593d90cfb4659801212594b723312"
    }
  ],
  "next_cursor": "f02fa979-d029-4909-b95a-bcd4d18da7c6",
  "has_more": true,
  "type": "page",
  "page": {}
}"#;

# notion-into-sqlite

A command line tool to download your Notion's database and save it locally into SQLite.

## Usage

You need Notion API key and database ID you want to download.

```
notion-into-sqlite --api-key <NOTION_API_KEY> --database-id <NOTION_DATABASE_ID>
```

For more detail, `$ notion-into-sqlite --help` shows available options.

## Installation

Using [Eget](https://github.com/zyedidia/eget), which enables you to easiliy get pre-built binaries, is the most quick way to install.

```
eget FujiHaruka/notion-into-sqlite
```

You can directly download binaries from [Releases](https://github.com/FujiHaruka/notion-into-sqlite/releases/).

Or you can install via cargo.

```
cargo install notion-into-sqlite
```

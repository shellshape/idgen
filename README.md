# idgen

A simple CLI to generate various UID formats in the terminal.

Currently, the following UID formats are supported:
  - [xid](https://github.com/rs/xid)
  - [uuid](http://guid.one/guid)
  - [snowflake](https://en.wikipedia.org/wiki/Snowflake_ID)

## Usage

```
$ idgen -h
A simple CLI to generate various UIDs in console.

Usage: idgen [OPTIONS] <COMMAND>

Commands:
  xid        Generate XIDs (see https://github.com/rs/xid)
  uuid       Generate UUIDs (aka. GUIDs)
  snowflake  Generate Snowflake
  help       Print this message or the help of the given subcommand(s)

Options:
  -n, --number <NUMBER>  Set to larger than 1 to generate a list of UIDs [default: 1]
  -h, --help             Print help information
  -V, --version          Print version information
```
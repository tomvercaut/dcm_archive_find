# dcm_archive_find

## Description

`dcm_archive_find` is a utility that helps locate DICOM files in an archive using patient IDs. It maintains an SQLite
database of archive paths and provides functionality to manage these paths and search for specific patient records.

## Building

To build the project, you'll need Rust installed on your system. Then:

```shell
cargo build --release
```

## Usage

The application can be used in two modes: command-based operations for managing a search path database, and direct
patient ID
searches.

### Database Management Commands

- Initialize a new database:

```shell
dcm_archive_find init
```

- Add a new archive path to the database:

```shell
dcm_archive_find add /path/to/archive
```

- Remove an archive path from the database:

```shell
dcm_archive_find remove /path/to/archive
```

- List all the search paths in the database:

```shell
dcm_archive_find list
```

### Patient ID Search

Searching for a patient ID:

```shell
dcm_archive_find "XXXXXX XXXAXX"
```

where `X` is a digit between 0 and 9.

## Logging

Logging at a specific can be enabled by using the commandline argument:

- verbose:

```shell
dcm_archive_find --verbose
```

- debug:

```shell
dcm_archive_find --debug
```

- trace:

```shell
dcm_archive_find --trace
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
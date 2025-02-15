# JDbrowser - Sqlite Database Browser

Browse an Sqlite database from the terminal.

# Installation and Building

No configuration needed.

### Download Binary

Binaries are available for download [Here](https://github.com/Jkeyuk/JDbrowser/releases) 

### Install With Rust

A simple way to install the binary using Rust:

`cargo install --path .`

### Build with Rust

A binary can also be directly built with:

`cargo build --release` 

`cargo build --release --target x86_64-unknown-linux-musl` 

The binary will be available at ***target/release/jdbrowser***

# Usage

- Run the application from the directory containing the database.

- Select your database from the initial file menu.

- Use the keybindings to browse the tables and views of your database.

## Key Binds

### File Menu

| Action | Keybind |
| ------------- | -------------- |
| Up        |  k        |
| Down      |  j        |
| Select    |  Enter    |

### Main view left side navigation

| Action | Keybind |
| ------------- | -------------- |
| Tab left - right        |  q, e        |
| Up        |  k        |
| Down      |  j        |

### Table View

| Action | Keybind |
| ------------- | -------------- |
| Tab left - right        |  h, l        |
| Page Up / Down Half |  u, d |  
| Up            |  shift + k  |
| Down            |  shift + j  |
| Left            |  shift + h  |
| Right            |  shift + l  |

# Screen Shots

![file_menu](docs/filemenu.png) 

![table_view](docs/table_view.png) 

![schema_view](docs/schema_view.png) 

# TODO

- error handling popup
- handle blob data
- show preview window

- Add docs like peaclock
- add shell script to build or install

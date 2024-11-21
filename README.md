
I'm starting to see an issue with the usage of this project. I want to be able to make the expenses on the go, taking picture of the receipts as I go about my day. Having a cli app would stop me from achieving this. Ideally this would need to be a cross-platform mobile app of some sort. I've already used expo for mobile development, I'll check for alternatives. I wanted to do a Rust project, but the ecosystem doesn't seem mature enough yet. I'll still consider rust and do an xp to try to use the features I want. So let's enter brainstorming again and try to find libraries grouped by languages that could be used. I'm not *super* familiar with mobile development as I haven't done a lot and it didn't inspire me that much.

This could be used with a server that does the actual syncing of the expenses. Exports could be done from the server. Lua script would still be available for varying the export procedure.

The requirements for choosing a mobile framework:

- can take pictures (directly or by using camera app then returning image file)
- can send http requests (to sync with backend server)

# Languages

Here are the groups of language that I would want to use. Ordered from more attracted to least.

## Rust

### [tauri](https://v2.tauri.app/)

Check [create-project](https://v2.tauri.app/start/create-project/) and [develop](https://v2.tauri.app/develop/). These documentation seem to provide enough instruction to build & run the desktop/mobile apps. The folder `xp/` in this repo contains experiments for the different libraries.

### [wry](https://github.com/tauri-apps/wry)

## Javascript

### [Expo](https://expo.dev/)

This seems like the best option. Already offers incredible support out of the box. If I want to use Rust I could create a lib that would be linked in the final package. This could be then called by the react-native runtime. But I don't think it'll even be necessary. If the mobile app just creates some expenses and syncs them with a server, the server will be in charge of the export. The server could be extremely lightweight and for simplicity, single user.

## Golang

- [gomobile](https://go.dev/wiki/Mobile)

# General Design

The mobile application will syncs expenses to the server. For accurate synchronization, we could use [Lamport timestamp](https://en.wikipedia.org/wiki/Lamport_timestamp) to order the events correctly. I don't think this is useful in our case, since we could have a column defining if the data was actually synced to the server. We could periodically re-check all the rows we have saved to ensure they're found in the server. After a certain period of time, synced rows could be removed from the mobile app since they can be found in the server.

In the case the server is not available, the mobile app should still be useable. Information should be displayed that the server is currently inaccessible so certain features are limited.

The server will act for a single user. A key could be generated for the two to communicate securely. The server will be in charge of exporting the data. So any exporting methods will need to be configured in the server. The server should be in order to provide the data back to the client, using paging to avoid sending excessive payloads.

# Data modeling

The main data structure to be used should be an ExpenseReport. This can include multiple expenses with different receipts. This is useful to track taxes for various different things.

Expenses {
  uuid: String,
  created: Datetime,
  modifier: Datetime,
  value: f32,
  additional_data: Json
}

ExpenseReport {
  expenses: List<Expenses>
}

---

# Archive

Everything underneath is outdated and will eventually be removed.

# General

This repository is to help keeping track of
expenses account and to export them. Export can be
something like an excel spreadsheet, and API
somewhere than accepts something, whatever.

## Brainstorming

The main concern is the export. Since the output
format could be variable, they are multiple paths
to take. The best bet would be using Lua as the
scripting language and execute a script for the
export. The first thing to test will be the usage
of Golang to run Lua scripts with modules. See the
`lua-testing` folder.

Feedback from the script could be accounted in the
cli. Meaning if we try to export but the script
fails, we could have an error message and an exit
code.

### Inputs

What does the user need to do to interact with the
tool?

This will be a cli app written in golang. The
features include:

- can input expenses to be tracked (cobra)
    * can specify date (by default current date)
    * can specify type of expense
    * can specify a description
- can list the expenses in a table view (go-pretty)
    * can filter by type
    * can filter by date or date range
- can export the expenses (gopher-lua)
    * using lua script

#### Export Script

The export script is a lua script that will be
executed and handled the responsability of
exporting that data. This decouples the specific
need for an export path from the cli tool.

#### Storage Backend

The expenses need to be stored somewhere. The
easiest would be a csv file somewhere, but that
can be troublesome if you want to keep a large
history of previous expenses. A better and more
future proof solution could be using a sqlite
database. I'm seeing `ncruces/go-sqlite3` as a
good option. 

### Commands

This is brainstorming of the commands available
and the general syntax for them.

```bash
# adds a new expense with travel tag
expenses add 23.54 desc:"going to Montreal" +travel
# show table of expenses
expenses
expenses <50 +travel
# export expenses using api.lua script
expenses export api
```

### Configuration

There should be a place where configuration could
be defined in toml format. Configuration could be
used in the cli and passed to the script to handle
different cases.


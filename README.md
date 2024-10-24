
Soooo, I've setup a minimal POC to see how this
would go using Golang (to learn). BUT, startup
time to load the actual sqlite & start performing
queries is very slow (go 0.99 seconds vs Rust 0.00
lol). I was comparing with taskwarrior since this
is the inspiration for this project. I did a
couple of tests for performance and man, I don't
think I like go very much. So I'm going to rebuild
this start of a project in Rust and stick to it.

# Performance tests

Using the first sqlite module that I've found,
performance was lacking a little bit. Hence why I
started experimenting with other sqlite library. I
found [this one](https://github.com/zombiezen/go-sqlite) which could be enough. The question
is, do I want 'enough', or do I build this using
Rust. This is a tool that I will be using. Meaning
I want this to be running smoothly.

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


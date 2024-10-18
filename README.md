
# General

This repository is to help keeping track of
expenses account and to export them. Export can be
something like an excel spreadsheet, and API
somewhere than accepts something, whatever.

## Brainstorming

This is the area where I ramble what the project
will be like and what it should be able to do.

The main concern is the export. Since the output
format could be variable, they are multiple paths
to take. The first one is to have `Exporters` in
the code that can export the data. This can be
great if you know exactly where to access
everything and the export path will always be the
same. But this breaks appart when you have two
different companies since they won't handle
expenses the same way. The second way could be to
have generic exporters which can be configured a
little bit like a pipeline. This could be nice but
involves complex configuration. The last thing
would be to use a scripting language alongside go
to write scripts for exporting the data. Data
could be injected in the script and the script
would handle the export itself. This allows the
user to manage everything as it sees fit.

The best bet would be using Lua as the scripting
language and execute a script for the export. The
first thing to test will be the usage of Golang to
run Lua scripts with modules. See the `lua-testing`
folder.

### Inputs

What does the user need to do to interact with the
tool?

This will be a cli app written in golang. The
features include:

- can input expenses to be tracked
    * can specify date (by default current date)
    * can specify type of expense
    * can specify a description
- can list the expenses
    * can filter by type
    * can filter by date or date range
- can export the expenses
    * using lua script?


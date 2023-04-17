# GenOS
This is an operating system based on [PHIL'S tutorial](https://os.phil-opp.com/)

This have the goal of proposing a different way of storing data, inspired by set theory: one can do intersection, union and complementary of tags, where each file can have multiple tags.

For now it serves as a template to a project on another branch (`Choke`) witch aim to be the first game to be play on a operating system.
This branch have a TUI manager and a custom trait use to create a scene system, and a buzzer sound implementation for sound output.

Progress made:
 - [x] GDT
 - [x] IDT
 - [x] keyboard interactions
 - [x] Screen system to render multiple TUI with a custom trait
 - [ ] A simple command-line
 - [ ] Making the command line Turing-complete
 - [ ] Added file communication driver
 - [ ] Adding a request system for files
 - [ ] Testing how fast it is on different data structures

## The request system (TODO)

The idea is that the MBR of a hardrive is set to a custom type of partition, witch itself have a header containing a list of all files, and their tags.

Every file have multiple tag (at least the tag `ID:<int>`)

There are 3 types of tag : 
 - Simple tag of the form `<word>`
 - A `<key>:<value>` tag
 - A `<key>:<value>,<value>,...` tag

Where a value can be a string, or a file's ID.
Every file have a `id:<int>` tag, witch is unique

Here's an example:
`id:10 || (image && created:23-01-2022)` will request the file with ID `10` or every image made on the 23/01/2022

Other things will be available :
 - Filtering a set by any P a proposition 
 - Making relations (like a `son:X,Y,Z` tag that will allow for a path-like system) and relation operation easier

It is very much still in the works.

## Branches and versions

`Choke` is a game design to be an OS at the same time. This branch use this OS to this endevor.

The screen trait is only present on the `Choke` branch for now.

See [GenOS-old](https://github.com/Cypooos/GenOS-old) for the old C++ version of this project

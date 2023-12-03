# Obgrep

Obgrep is a simple command line tool, it works a lot like `grep -r` but provides a list of matches with numbers corresponding to each file, entering that number then opens the file in Obsidian, intendedas an in terminal way of searching Obsidian notes

# Usage

`obgrep <folder> <text>` will recursively search `<folder>` (ignoring dot files) for lines matching `<text>`

e.g
```
obgrep ~/personal_notes multiple
(1) testing note.md: checkout multiple lines over at [[testing note 2]]
(2) testing note 2.md: It's important this note has multiple lines!
(2) testing note 2.md: we need them to match multiple things!
(2) testing note 2.md: Wow, multiple!
```

entering 1 or 2 would then open `testing note.md` or `testing note 2.md` respectively in Obsidian (the idea is that the folder you're searching will mostly correspond to an Obsidian vault)

# Setup
The source code here needs to be compiled into a binary by running `cargo build --release` (for the sake of security, it's better that you do this yourself), you might then want to move `obgrep` (`./target/release/obgrep`) to somewhere in your path, or extend your path to include it. After doing that you can happily remove the repo if you like.

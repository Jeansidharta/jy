# jy

This is an extremely simple JSON to YAML converter. It'll read JSON from stdin and print out YAML on stdout.

This was inspired by [yj](https://github.com/bruceadams/yj). Even though yj can do the same thing as this project, I wanted something very simple for this.

# installation

You must have the rust toolchain installed in your system. You can install it [here](https://www.rust-lang.org/tools/install) if you need to.

`cargo install jy`

# Usage

This program receives no arguments. It'll simply read from the stdin and write to stdout. Example:

`echo "{potato: \"tomato\"}" | jy`

will output `potato: tomato`

# `viz`

`viz` makes a html document showing a debug visualisation of what the output of a hpgl file will look like. It takes a single input file argument, and outputs a html document to stdout. Currently, it only supports the `PU`, `PD`, `PA`, `PR`, and `SP` commands. The `IN` command is treated as a nop.

To use it:

* Run `cargo run -- ~/path/to/your/file.hpgl > /tmp/some_file.html`
* Open `file:///tmp/some_file.html` in your web browser


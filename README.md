# plotter-tools

Some tools for interacting with the HP7440A pen plotter.

## `chunker`

`chunker` is used to communicate with the pen plotter.

It gets around the plotters 60 byte buffer, by taking in a file of HP-GL
commands, and sending each one to the plotter punctuated by a `OA` command,
then waiting to send the next command to the plotter until a response has been
received. This allows sending large files to the plotter without overflowing
the internal buffer. The `chunker` command also prepends `IN;OI;` to the entire
file, initializing the plotter.

To use `chunker`:

* Install Rust from [rustup.rs](https://rustup.rs/)
* `cd` into the `chunker` directory
* Run `cargo build`
* You many need to install `libudev-dev` or a similar package if it fails to
  build
* Run `sudo ./target/debug/chunker /dev/ttyUSB0 ~/path/to/your/hpgl/file`

And your file should print!

If you're using a Mac, install [this driver](http://www.prolific.com.tw/US/ShowProduct.aspx?p_id=229&pcid=41)
and replace `/dev/ttyUSB0` with `/dev/tty.usbserial`.

If you run into mysterious problems, try disconnecting and reconnecting the
plotter before running the `chunker` command.

## `viz`

There is also a `viz` command, which makes a html document showing a debug
visualisation of what the output of a command will look like. It takes a single
input file argument, and outputs a html document to stdout. Currently, it only
supports the `PU`, `PD`, `PA`, `PR`, and `SP` commands. For commands that can
take a list of points, only the one-point versions are supported.

To run it:

* Install Rust from [rustup.rs](https://rustup.rs/)
* `cd` into the `viz` directory
* Run `cargo run ~/path/to/your/hpgl/file > /tmp/some_file.html`
* Open `file:///tmp/some_file.html` in your web browser

## `convert.py`

`convert.py` converts a HP-GL file that is in only absolute movements to one
that is in only relative movements. It is very limited - it only works on files
that have one command per line, don't use `PD` or `PU` to move the pen, and
only use `PA` movement commands with a single position as a parameter. I use
it, in addition to some vim macros to convert HP-GL files outputted by inkscape
to ones that can be used at arbitrary points in a script. It's pretty janky
right now, improvements are appreciated <3

## `typewriter`

`typewriter` generates plotting instructions that render text on the page, for
any arbitrary `ttf`-compatible font on your computer. Unfortunately, due to
limitations in `rusttype`, it only supports LTR text for now. The output assumes
that the correct pen is already selected.

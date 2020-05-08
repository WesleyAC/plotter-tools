# plotter-tools <a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a> [![Build Status](https://travis-ci.org/WesleyAC/plotter-tools.svg?branch=master)](https://travis-ci.org/WesleyAC/plotter-tools)

Some tools for interacting with pen plotters that use HPGL.

* [`chunker`](./chunker/) - send hpgl files to the pen plotter, dealing with software flow control.
* [`viz`](./viz/) - convert a hpgl file to svg for easy debugging.
* [`adapter`](./adapter/) - 3d models for adapters to hold modern pens in vintage HP plotters.
* [`typewriter`](./typewriter/) - convert text to hpgl, using a given font.
* [`hpgl2gcode`](./hpgl2gcode/) - convert a hpgl file to G-Code, for use with the [NYCR plotter](https://trmm.net/Plotter).

The [plotters](./plotters/) directory has some documentation, notes, models for spare parts, and firmware dumps for various models of HP plotters.

My typical workflow is to write a Python script that uses print statements to output HPGL code directly to standard out, then I'll do something like:

```bash
viz <(./my_script.py) > /tmp/output.html
```

I'll also typically use [`entr`](https://bitbucket.org/eradman/entr/src/default/) and [Live Reload](https://github.com/blaise-io/live-reload/) so that I can automatically run my script when I save it, and have my browser reload the output - that looks something like this:

```bash
echo my_script.py | entr bash -c "viz <(./my_script.py) > /tmp/plotter-art/index.html"
```

then in `/tmp/plotter-art`, run a web server in order for Live Reload to be able to work:

```bash
python3 -m http.server 8080
```

Once I have a piece that I'm happy with, I'll plot it:

```bash
chunker <(./my_script.py)
```

## Getting Started

Before using any of these tools, you'll need to install the Rust toolchain - you can do that by following the instructions at [rustup.rs](https://rustup.rs).

To use a tool, `cd` into the directory it's in, and run `cargo build` - this should create a directory called `target/debug/` that has the binary in it - for instance, you might build and run `viz` like this:

```bash
cd viz
cargo build
./target/debug/viz /path/to/my/hpgl/file.hpgl > /path/to/my/output/file.html
```

The instructions are similar for all of the tools, but checkout the READMEs in each directory for more details.

## Contributing

Feel free to send pull requests, ask for features, or ask for questions/help! If there's a feature you'd like to add, but you aren't sure how to go about it, I'm happy to provide advice or pair on it - you can reach me@wesleyac.com, or reach out on Zulip if you're part of the [Recurse Center](https://www.recurse.com/) community :)

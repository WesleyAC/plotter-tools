# plotter-tools

Some tools for interacting with the HP7440A pen plotter.

The main tool in this repo is `chunker`, which gets around the plotters 60 byte
buffer, by taking in a file of HP-GL commands, and sending each one to the
plotter punctuated by a `OA` command, then waiting to send the next command to
the plotter until a response has been received. This allows sending large files
to the plotter without overflowing the internal buffer. The `chunker` command
also prepends `IN;OI;` to the entire file, initializing the plotter.

The `chunker` command takes two arguments - the first is the serial devices for
the plotter (for me, typically `/dev/ttyUSB0`), and the second, the name of the
file that you would like to send.

If you run into mysterious problems, try disconnecting and reconnecting the
plotter before running the `chunker` command.

There is also a `viz` command, which makes a html document showing a debug
visualisation of what the output of a command will look like. It takes a single
input file argument, and outputs a html document to stdout. Currently, it only
supports the `PU`, `PD`, `PA`, `PR`, and `SP` commands. For commands that can
take a list of points, only the one-point versions are supported.

`convert.py` converts a HP-GL file that is in only absolute movements to one
that is in only relative movements. It is very limited - it only works on files
that have one command per line, don't use `PD` or `PU` to move the pen, and
only use `PA` movement commands with a single position as a parameter. I use
it, in addition to some vim macros to convert HP-GL files outputted by inkscape
to ones that can be used at arbitrary points in a script. It's pretty janky
right now, improvements are appreciated <3

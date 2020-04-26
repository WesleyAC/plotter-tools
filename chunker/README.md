# `chunker`

`chunker` is used to communicate with the pen plotter.

It gets around the plotters 60 byte buffer, by taking in a file of HP-GL commands, and sending each one to the plotter punctuated by a `OA` command, then waiting to send the next command to the plotter until a response has been received. This allows sending large files to the plotter without overflowing the internal buffer. The `chunker` command also prepends `IN;` to the entire file, initializing the plotter.

To use `chunker`:

* Run `cargo build`
* You many need to install `libudev-dev` or a similar package if it fails to build
* Run `sudo ./target/debug/chunker /dev/ttyUSB0 ~/path/to/your/hpgl/file`

And your file should print!

If you're using a Mac, install [this driver](http://www.prolific.com.tw/US/ShowProduct.aspx?p_id=229&pcid=41) and use `/dev/tty.usbserial` in the above command, instead of `/dev/ttyUSB0`.

If you run into mysterious problems, try disconnecting and reconnecting the plotter before running the `chunker` command.

# `chunker`

`chunker` is used to communicate with the pen plotter.

It gets around the plotters 60 byte buffer, by taking in a file of HP-GL commands, and sending each one to the plotter punctuated by a `OA` command, then waiting to send the next command to the plotter until a response has been received. This allows sending large files to the plotter without overflowing the internal buffer.

To use `chunker`:

* Run `cargo build`
* You many need to install `libudev-dev` or a similar package if it fails to build
* Run `sudo ./target/debug/chunker ~/path/to/your/hpgl/file`

And your file should print!

If you're using a Mac, install [this driver](http://www.prolific.com.tw/US/ShowProduct.aspx?p_id=229&pcid=41) and check that the file `/dev/tty.usbserial` exists when the serial cable is plugged in, in order to make sure it installed correctly.

The program tries to automatically choose the serial device if only one of `/dev/ttyUSB*` and `/dev/tty.usbserial` exist, but you can also specify the serial device on the command line if you'd like.

You'll probably need to be root or use `sudo` to access the serial port by default, but you should be able to add your user to the `dialout` group to fix this - `sudo usermod -a -G dialout $USER` should get you all set up :)

If you run into mysterious problems, try disconnecting and reconnecting the plotter before running the `chunker` command.

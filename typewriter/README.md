# `typewriter`

`typewriter` generates plotting instructions that render text on the page, for a font on your computer. Unfortunately, due to limitations in `rusttype`, it only supports LTR text and `ttf` font files for now. The output assumes that the correct pen is already selected. You may also want to set the velocity so that straight lines and curves are plotted with the same darkness — `VS 2;` seems to work well.

To use `typewriter`:

* Run `cargo run -- <x> <y> <font size> "your text here" /path/to/font/file.ttf > yourfile.hgpl`


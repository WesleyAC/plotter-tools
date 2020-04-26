# `hpgl2gcode`

`hpgl2gcode` converts HPGL programs to G-Code programs that can be plotted on the [plotter at NYC Resistor](https://trmm.net/Plotter). To use it:

* Run `cargo run -- ~/path/to/your/file.hpgl > ~/path/to/your/file.gcode`
* Use [ReplicatorG](http://replicat.org/) to send the gcode file to the plotter.

It applies a scale factor to both axes as well - currently, it is not configurable without changing the source, but pull requests are welcome :) The HPGL -> Gcode step is the correct step to apply the scale factor at, since HPGL is integer-only, and thus suffers from more rounding problems than Gcode.

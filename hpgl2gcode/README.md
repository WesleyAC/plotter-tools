# `hpgl2gcode`

`hpgl2gcode` converts HPGL programs to G-Code programs that can be plotted on the [plotter at NYC Resistor](https://trmm.net/Plotter). To use it:

* Run `cargo run -- ~/path/to/your/file.hpgl > ~/path/to/your/file.gcode`
* Use [ReplicatorG](http://replicat.org/) to send the gcode file to the plotter.

It applies a scale factor to both axes, configurable with the `--xscale`/`-x` and `--yscale`/`-y` options. The default is 0.076, which works well for converting from the scale of the HP7440A (the pen plotter at the Recurse Center) to the weird custom pen plotter at NYC Resistor. The HPGL -> Gcode step is the correct step to apply the scale factor at, since HPGL is integer-only, and thus suffers from more rounding problems than Gcode.

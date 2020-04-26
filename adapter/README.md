# HP Plotter Pen Adapters

This directory contains some OpenSCAD code to generate a 3d model of an adapter that allows for the use of pens other than the vintage HP plotter pens.  It's based on [Jean-Philippe Côté's model](https://www.thingiverse.com/thing:2955469).

In order to make holders for new pens, you'll need to install OpenSCAD, open `adapter.scad`, and add a new module for the pen you want to add. If you make one that works, pleas submit a PR! I'm very excited to try new pens :)

We've found that SLA machines have good enough resolution to print these, and Jean-Philippe [mentions](https://www.tinkercad.com/things/kDwyCi3l2R6) that using a SLS machine with PA12 nylon seems to work well. I think that it's unlikely that a typical hobbyist FDM machine would be accurate enough to work well, but I haven't tried it.

The model is currently designed so that it can be used in either a HP7440 or HP7475 plotter, but Jean-Philippe's model might be a better bet if you only need to use a HP7440, since it's a bit wider, possibly giving the arm a better grip on the pen. The HP7475 has a smaller carousel, so the wider adapter doesn't fit.

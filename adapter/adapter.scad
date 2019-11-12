include <threads.scad>;
include <Write.scad>;

// Note - the set screws are too complex for OpenSCAD's renderer - in order to
// fix this, go into Preferences > Advanced and set "Turn off rendering at" to a
// larger number. This will make rendering much slower - to make rendering
// faster for interactive development, add `test=true` to the parameters for
// `metric_thread` - but be sure to remember to remove it before actually
// printing, otherwise the set screws won't work!

module adapter() {
    union() {
        translate([0,10,0]) import("./adapter.stl", convexity=4);
        cylinder(30, d=10);
    }
}

// untested
module set_screws() {
    for (i=[0:2]) {
        translate([0,0,6])
        rotate([90,0,i*120])
        // A standard M4 fine-pitch nut
        metric_thread(4, 0.5, 10, internal=true);
    }
}

module label(name) {
    rotate([180,0,0])
    translate([0,0,-2])
    writecylinder(name, radius=8.75, h=3.25);
}

// untested
module faber_castell_pitt_pen() {
    translate([0,0,-1]) cylinder(32, d1=9, d2=8.5, $fs=0.1);
    label("FABER-CASTELL PITT");
}

difference() {
    adapter();
    set_screws();
    faber_castell_pitt_pen();
}

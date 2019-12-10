include <Write.scad>;

module adapter() {
    union() {
        difference() {
            translate([0,10,0]) import("./adapter.stl", convexity=4);
            translate([0,0,-1]) cylinder(15, d=20); // remove base that is too wide
        };
        cylinder(30, d=11.5); // fill in center and extend to cover removed base

    }
}

module label(name) {
    rotate([180,0,0])
    translate([0,0,-4])
    writecylinder(name, radius=5.6, h=3.5);
}

// untested
module faber_castell_pitt_pen() {
    union() {
        difference() {
            adapter();
            translate([0,0,-1]) cylinder(32, d1=9.02, d2=8.55, $fs=0.1);
        }
        label("FABER-CASTELL");
    }
}

faber_castell_pitt_pen();

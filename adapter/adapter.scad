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
            translate([0,0,-1]) cylinder(32, d1=9, d2=8.5, $fs=0.1);
        }
        label("FABER-CASTELL");
    }
}

faber_castell_pitt_pen();
// translate([14, 0, 30]) rotate([0, 180, 0]) faber_castell_pitt_pen();
// translate([14, 14, 0]) faber_castell_pitt_pen();
// translate([0, 14, 30]) rotate([0, 180, 0]) faber_castell_pitt_pen();
// rotate([90, 0, 0]) translate([0, 1, -9]) cylinder(4, d=1.5);
// rotate([0, 90, 0]) translate([-1, 0, 5]) cylinder(4, d=1.5);
// rotate([90, 0, 0]) translate([13.5, 1, -9]) cylinder(4, d=1.5);
// rotate([0, 90, 0]) translate([-1, 14, 5]) cylinder(4, d=1.5);
// rotate([90, 0, 0]) translate([0, 29, -9]) cylinder(4, d=1.5);
// rotate([0, 90, 0]) translate([-29, 0, 5]) cylinder(4, d=1.5);
// rotate([90, 0, 0]) translate([13.5, 29, -9]) cylinder(4, d=1.5);
// rotate([0, 90, 0]) translate([-29, 14, 5]) cylinder(4, d=1.5);


union() {
    difference() {
        hull() {
            translate([20,0,0]) cylinder(2.5, d=10, $fn=40);
            cylinder(6, d=27, $fn=80);
        }
        translate([20, 0, 2.5]) cylinder(6.5, r=16, $fn=120);
        difference() {
            cylinder(20, d=6.35, center=true, $fn=30);
            translate([-11.5 , -3, 0]) cube([10, 6, 10]);
        }
    }
    translate([20, 0, 0]) cylinder(6.5, d=3.175, $fn=20);
    difference() {
        translate([0, 0, -1]) cylinder(2, d=9, $fn=30);
        cylinder(5, d=6.35, center=true, $fn=30);
    }
}
#![feature(or_patterns)]
use hpgl::{parse_commands, canonicalize, canonical_commands_to_string, CanonicalCommand, Command, Point};
use std::collections::HashMap;
use std::fs::read_to_string;

// possible optimizations:
// * remove duplicate PU and PD commands (done)
// * coalesce multiple PA, PU, and PD commands into one (is a superset of the above)
// * merge lines into one
// * remove redundant PU/PD commands (different from removing duplicates, although maybe we can do that in the same pass?)
// * only draw overlapping line segments once
// * omit/truncate lines that draw outside of the drawable area
// * remove all of the superfluous spaces in commands :)

// pen plotter behaviour to verify:
// * how does PD choose to use absolute vs relative coords?

// this code is not ready yet, and currently has many bugs and just overall doesn't work!
// you probably shouldn't try to use it!

fn main() {
    let parsed_commands = parse_commands(std::fs::read_to_string("input.hpgl").unwrap()).unwrap();
    let canonical_commands = canonicalize(parsed_commands);
    let unopt_shape_map = canonical_commands_to_shapes(canonical_commands);
    let opt_shape_map = optimize_all_colors(unopt_shape_map);
    let opt_commands = shapes_to_commands(opt_shape_map);
    println!(
        "{}",
        canonical_commands_to_string(opt_commands));
}

fn optimize_all_colors(color_to_shapes: HashMap<u8, Vec<Shape>>) -> HashMap<u8, Vec<Shape>>{
    let mut out = HashMap::new();
    for (color, shapes) in color_to_shapes.iter() {
        out.insert(*color, order_shapes(shapes.clone()));
    }
    out
}

fn shapes_to_commands(color_to_shapes: HashMap<u8, Vec<Shape>>) -> Vec<CanonicalCommand>{
    let mut out = vec![];
    for (color, shapes) in color_to_shapes {
        if color != 0 {
            out.push(CanonicalCommand::SelectPen(color));
            for shape in shapes {
                out.push(CanonicalCommand::PenUp);
                for (i, point) in shape.iter().enumerate() {
                    out.push(CanonicalCommand::PlotAbsolute(*point));
                    if i == 0 {
                        out.push(CanonicalCommand::PenDown);
                    }
                }
            }
        }
    }
    out.push(CanonicalCommand::PenUp);
    out.push(CanonicalCommand::SelectPen(0));

    out
}

type Shape = Vec<Point>;

fn canonical_commands_to_shapes(cmds: Vec<CanonicalCommand>) -> HashMap<u8, Vec<Shape>> {
    let mut active_color = 0;
    let mut current_shape = vec![];
    let mut is_pen_down = false;
    let mut color_to_shapes = HashMap::new();
    for cmd in cmds{
        match cmd {
            CanonicalCommand::SelectPen(color) => {
                active_color = color;
                // selecting pen ends current shape
                if !color_to_shapes.contains_key(&active_color){
                    color_to_shapes.insert(active_color, vec![]);
                }
                if current_shape.len() > 0 {
                    color_to_shapes.get_mut(&active_color).unwrap().push(current_shape);
                    current_shape = vec![];
                }
            }
            CanonicalCommand::PlotAbsolute(point) => current_shape.push(point),
            CanonicalCommand::PenDown => is_pen_down = true,
            CanonicalCommand::PenUp => {
                is_pen_down = false;
                if !color_to_shapes.contains_key(&active_color){
                    color_to_shapes.insert(active_color, vec![]);
                }
                if current_shape.len() > 0 {
                    color_to_shapes.get_mut(&active_color).unwrap().push(current_shape);
                    current_shape = vec![];
                }
            }
            CanonicalCommand::Initialize => panic!("What is this even for?"),
        }
    }
    color_to_shapes
}

/// Euclidean
fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt(((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64)
}

/// Given a Point finds the Shape whose starting point is nearest the given point.
fn find_closest(p: &Point, shapes: &Vec<Shape>) -> Option<usize> {
    let mut best_shape = None;
    let mut best_distance: f64 = f64::INFINITY;
    for (i, shape) in shapes.iter().enumerate() {
        let point = shape[0];
        let dist = distance(&point, &p);
        if dist < best_distance {
            best_distance = dist;
            best_shape = Some(i);
        }
    }

    best_shape
}

/// Takes all shapes to draw with one pen color and figures out a better order to draw them in
/// Starts with first shape
/// After finishing drawing a shape finds nearest undrawn shape to draw next
/// Possible improvements:
///     * use Vec<Option<Shape>> instead of .remove
///     * use some kind of tree datastructure
///     * change order in which an individual shape is drawn (start at a different point)
///     * write unit tests
fn order_shapes(mut shapes: Vec<Shape>) -> Vec<Shape> {
    if shapes.len() == 0 {
        return vec![];
    }
    let mut prev_shape = shapes.remove(0);
    let mut out = vec![prev_shape.clone()];

    while shapes.len() > 0 {
        let next_shape = find_closest(prev_shape.last().unwrap(), &shapes).unwrap();
        prev_shape = shapes.remove(next_shape);
        out.push(prev_shape.clone());
    }

    out
}



fn remove_duplicate_pen_commands(cmds: Vec<Command>) -> Vec<Command> {
    let mut out = vec![];
    let mut previous: Option<Command> = None;
    for cmd in cmds.iter() {
        match (previous, cmd) {
            (None, cmd) => previous = Some(cmd.clone()),
            (Some(Command::PenUp(_)) | Some(Command::PenDown(_)), Command::PenUp(pu)) => {
                previous = Some(Command::PenUp(pu.to_vec()))
            }
            (Some(Command::PenUp(_)) | Some(Command::PenDown(_)), Command::PenDown(pd)) => {
                previous = Some(Command::PenDown(pd.to_vec()))
            }
            (Some(prev), new) => {
                out.push(prev);
                previous = Some(new.clone());
            }
        }
    }
    if let Some(prev) = previous {
        out.push(prev);
    }
    out
}

#[test]
fn test_duplicates() {
    let test_commands = parse_commands(String::from("IN;PU;PD;PA1000,1000;PU;PU;")).unwrap();
    let expected_commands = parse_commands(String::from("IN;PD;PA1000,1000;PU;")).unwrap();
    let deduped = remove_duplicate_pen_commands(test_commands);
    assert_eq!(deduped, expected_commands);
}

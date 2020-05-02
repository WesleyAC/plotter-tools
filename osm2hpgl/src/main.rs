#![feature(try_trait)]

use std::fs;
use std::collections::HashMap;

fn get_tags(e: &minidom::Element) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for node in e.children() {
        if node.name() == "tag" {
            let k = node.attr("k");
            let v = node.attr("v");
            match (k, v) {
                (Some(k), Some(v)) => {
                    let _ = out.insert(k.to_string(), v.to_string());
                },
                _ => {}
            }
        }
    }
    out
}

fn get_id(e: &minidom::Element) -> Option<u64> {
    Some(e.attr("id")?.parse::<u64>().map_err(|_| std::option::NoneError)?)
}

fn parse_node(e: &minidom::Element) -> Option<Node> {
    Some(Node {
        lat: e.attr("lat")?.parse::<f64>().map_err(|_| std::option::NoneError)?,
        lon: e.attr("lon")?.parse::<f64>().map_err(|_| std::option::NoneError)?,
        tags: get_tags(e),
    })
}

fn parse_relation(e: &minidom::Element) -> Option<Relation> {
    let mut members = vec![];
    for c in e.children() {
        if c.name() == "member" {
            members.push(c.attr("ref")?.parse::<u64>().map_err(|_| std::option::NoneError)?);
        }
    }
    Some(Relation {
        members,
        tags: get_tags(e),
    })
}

fn parse_way(e: &minidom::Element) -> Option<Way> {
    let mut nodes = vec![];
    for c in e.children() {
        if c.name() == "nd" {
            nodes.push(c.attr("ref")?.parse::<u64>().map_err(|_| std::option::NoneError)?);
        }
    }
    Some(Way {
        nodes,
        tags: get_tags(e),
    })
}

fn parse_bounds(e: &minidom::Element) -> Option<Bounds> {
    Some(Bounds {
        minlat: e.attr("minlat")?.parse::<f64>().map_err(|_| std::option::NoneError)?,
        minlon: e.attr("minlon")?.parse::<f64>().map_err(|_| std::option::NoneError)?,
        maxlat: e.attr("maxlat")?.parse::<f64>().map_err(|_| std::option::NoneError)?,
        maxlon: e.attr("maxlon")?.parse::<f64>().map_err(|_| std::option::NoneError)?,
    })
}

fn parse_osm(root: &minidom::Element) -> Option<Map> {
    let mut elements = HashMap::new();
    let mut bounds = None;

    for node in root.children() {
        let e = match node.name() {
            "node" => parse_node(node).map(|x| Element::Node(x)),
            "way" => parse_way(node).map(|x| Element::Way(x)),
            "relation" => parse_relation(node).map(|x| Element::Relation(x)),
            "bounds" => {
                if let Some(b) = parse_bounds(node) {
                    bounds = Some(b);
                }
                None
            },
            _ => None,
        };
        let id = get_id(node);
        match (id, e) {
            (Some(id), Some(e)) => { elements.insert(id, e); },
            _ => {},
        }
    }
    Some(Map {
        elements,
        bounds: bounds?,
    })
}

#[derive(Debug)]
struct Node {
    lat: f64,
    lon: f64,
    tags: HashMap<String, String>,
}

#[derive(Debug)]
struct Way {
    nodes: Vec<u64>,
    tags: HashMap<String, String>,
}

#[derive(Debug)]
struct Relation {
    members: Vec<u64>,
    tags: HashMap<String, String>,
}

#[derive(Debug)]
enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

#[derive(Debug)]
struct Bounds {
    minlat: f64,
    minlon: f64,
    maxlat: f64,
    maxlon: f64,
}

#[derive(Debug)]
struct Map {
    elements: HashMap<u64, Element>,
    bounds: Bounds,
}

fn in_bounds(p: &Node, b: &Bounds) -> bool {
    p.lat > b.minlat && p.lat < b.maxlat && p.lon > b.minlon && p.lon < b.maxlon
}

fn transform(p: &Node, b: &Bounds) -> (i64, i64) {
    let scalex = 10300.0 / (b.maxlat - b.minlat);
    let scaley = 7650.0 / (b.maxlon - b.minlon);

    (
        10300 - ((p.lat - b.minlat) * scalex) as i64,
        ((p.lon - b.minlon) * scaley) as i64,
    )
}

fn print_way(way: u64, map: &Map) {
    if let Some(Element::Way(way)) = map.elements.get(&way) {
        let mut first = true;
        for (i, node_id) in way.nodes.iter().enumerate() {
            if let Some(Element::Node(node)) = map.elements.get(&node_id) {
                // this code is really gross :(
                // we're checking if the adjacent points are in bounds, and if so, we draw this
                // point
                let mut print_point = in_bounds(node, &map.bounds);
                if i > 0 {
                    if let Some(Element::Node(prev)) = map.elements.get(&way.nodes.get(i-1).unwrap()) {
                        print_point |= in_bounds(prev, &map.bounds);
                    }
                }
                if let Some(Element::Node(next)) = map.elements.get(&way.nodes.get(i+1).unwrap_or(node_id)) {
                    print_point |= in_bounds(next, &map.bounds);
                }

                if print_point {
                    let (x, y) = transform(node, &map.bounds);
                    if first {
                        println!("PU;");
                        first = false;
                    }
                    println!("PA {},{};PD;", x, y);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_str = fs::read_to_string("map.osm")?;
    let root: minidom::Element = map_str.parse().unwrap();

    // relation types:
    // * public_transport
    // * restriction
    // * route
    // * multipolygon
    // * route_master

    let map = parse_osm(&root).unwrap();

    let ways = map.elements.iter().filter_map(|(k, v)| {
        match v {
            Element::Way(v) => Some((k, v)),
            _ => None,
        }
    });

    for (id, way) in ways {
        if way.tags.get("building").is_some() {
            println!("SP1;");
        } else if way.tags.get("leisure") == Some(&"park".to_string()) ||
            way.tags.get("leisure") == Some(&"playground".to_string()) { 
            println!("SP2;");
        } else if way.tags.get("highway").is_some() {
            println!("SP3;");
        } else {
            println!("SP0;");
        }
        print_way(*id, &map);
    }

    println!("SP0;");

    Ok(())
}

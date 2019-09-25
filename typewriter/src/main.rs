use std::fs::File;
use std::io::Read;
use rusttype::{self, FontCollection, PositionedGlyph, Segment, Contour, Curve, Line, Scale};

fn error(msg: &str) -> ! {
    eprintln!("{}", msg);
    eprintln!("usage: typewriter <x> <y> <font size> \"my text here\" /path/to/font/file.otf");
    std::process::exit(1);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn convert_point(point: rusttype::Point<f32>) -> Point {
    Point {
        x: point.x as i64,
        y: point.y as i64,
    }
}

fn segment_to_points(segment: Segment) -> Vec<Point> {
    match segment {
        Segment::Line(line) => {
            vec![line.p[0], line.p[1]].into_iter().map(convert_point).collect()
        }
        Segment::Curve(curve) => {
            let mut res = vec![];
            for i in 0..11 {
                let t = i as f32 * 0.1;
                let x = (1.0 - t) * (1.0 - t) * curve.p[0].x + 2.0 * (1.0 - t) * t * curve.p[1].x + t * t * curve.p[2].x;
                let y = (1.0 - t) * (1.0 - t) * curve.p[0].y + 2.0 * (1.0 - t) * t * curve.p[1].y + t * t * curve.p[2].y;
                let point = Point {
                    x: x as i64,
                    y: y as i64,
                };
                res.push(point);
            }
            res
        }
    }
}

fn countour_to_points(contour: Contour) -> Vec<Point> {
    let mut res = vec![];
    for segment in contour.segments {
        res.extend(segment_to_points(segment));
    }
    let mut dedup_res = vec![];
    for item in res {
        if item.x < 0 || item.x > 10300 || item.y < 0 || item.y > 7650 {
            eprintln!("WARNING: point {:?} is out of bounds", &item);
        }
        if Some(&item) != dedup_res.last() {
            dedup_res.push(item);
        }
    }
    dedup_res
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 6 {
        error("expected 5 arguments");
    }
    let x: i64 = args[1].parse().unwrap_or_else(|_| error("expected first arg to be int"));
    let y: i64 = args[2].parse().unwrap_or_else(|_| error("expected second arg to be int"));
    let font_size: i64 = args[3].parse().unwrap_or_else(|_| error("expected third arg to be int"));
    let msg: &str = &args[4];
    let path: &str = &args[5];

    let mut font_data: Vec<u8> = Vec::new();
    {
        let mut file = File::open(path).unwrap_or_else(|_| error("failed to open font file"));
        file.read_to_end(&mut font_data).unwrap_or_else(|_| error("failed to read font file"));
    }

    let collection = FontCollection::from_bytes(&font_data).unwrap_or_else(|e| {
        error(&format!("error constructing a FontCollection from bytes: {}", e));
    });
    let font = collection
        .into_font() // only succeeds if collection consists of one font
        .unwrap_or_else(|e| {
            error(&format!("error turning FontCollection into a Font: {}", e));
        });

    let glyphs: Vec<PositionedGlyph<'_>> = font.layout(msg, Scale::uniform(font_size as f32), rusttype::Point {x: x as f32, y: y as f32}).collect();
    for glyph in glyphs {
        let contours = match glyph.shape() {
            None => continue,
            Some(v) => v,
        };
        for contour in contours {
            let points = countour_to_points(contour);
            if points.len() == 0 {
                continue;
            }
            println!("PU;");
            println!("PA {}, {};", points[0].x, points[0].y);
            println!("PD;");
            for point in points {
                println!("PA {}, {};", point.x, point.y);
            }
            println!("PU;");
        }
    }
}

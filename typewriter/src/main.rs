use rusttype::{self, Contour, FontCollection, PositionedGlyph, Scale, Segment};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

fn error(msg: &str) -> ! {
    eprintln!("{}", msg);
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
        Segment::Line(line) => vec![line.p[0], line.p[1]]
            .into_iter()
            .map(convert_point)
            .collect(),
        Segment::Curve(curve) => {
            let mut res = vec![];
            for i in 0..11 {
                let t = i as f32 * 0.1;
                let x = (1.0 - t) * (1.0 - t) * curve.p[0].x
                    + 2.0 * (1.0 - t) * t * curve.p[1].x
                    + t * t * curve.p[2].x;
                let y = (1.0 - t) * (1.0 - t) * curve.p[0].y
                    + 2.0 * (1.0 - t) * t * curve.p[1].y
                    + t * t * curve.p[2].y;
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

#[derive(Debug, StructOpt)]
#[structopt(name = "typewriter", about = "A program to convert text to HPGL.")]
struct Args {
    x: i64,
    y: i64,
    font_size: i64,
    message: String,
    #[structopt(parse(from_os_str))]
    font_path: PathBuf,
    #[structopt(
        long,
        default_value = "1",
        help = "Makes plotting faster, but less accurate. Higher numbers are faster"
    )]
    rescale: i64,
}

fn main() {
    let args = Args::from_args();

    let mut font_data: Vec<u8> = Vec::new();
    {
        let mut file =
            File::open(args.font_path).unwrap_or_else(|_| error("failed to open font file"));
        file.read_to_end(&mut font_data)
            .unwrap_or_else(|_| error("failed to read font file"));
    }

    let collection = FontCollection::from_bytes(&font_data).unwrap_or_else(|e| {
        error(&format!(
            "error constructing a FontCollection from bytes: {}",
            e
        ));
    });
    let font = collection
        .into_font() // only succeeds if collection consists of one font
        .unwrap_or_else(|e| {
            error(&format!("error turning FontCollection into a Font: {}", e));
        });

    let glyphs: Vec<PositionedGlyph<'_>> = font
        .layout(
            &args.message,
            Scale::uniform(args.font_size as f32),
            rusttype::Point {
                x: args.x as f32,
                y: args.y as f32,
            },
        )
        .collect();
    for glyph in glyphs {
        let contours = match glyph.shape() {
            None => continue,
            Some(v) => v,
        };
        for contour in contours {
            let mut points = countour_to_points(contour);
            if points.len() == 0 {
                continue;
            }
            if args.rescale != 1 {
                let r = args.rescale;
                points = points
                    .iter()
                    .map(|p| Point {
                        x: ((p.x + r - 1) / r) * r,
                        y: ((p.y + r - 1) / r) * r,
                    })
                    .collect();
                points.dedup();
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

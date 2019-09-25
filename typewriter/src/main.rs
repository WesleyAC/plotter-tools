fn error(msg: &str) -> ! {
    eprintln!("{}", msg);
    eprintln!("usage: typewriter <x> <y> <font size> \"my text here\"");
    std::process::exit(1);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 5 {
        error("expected 4 arguments");
    }
    let x: i64 = args[1].parse().unwrap_or_else(|_| error("expected first arg to be int"));
    let y: i64 = args[2].parse().unwrap_or_else(|_| error("expected second arg to be int"));
    let font_size: i64 = args[3].parse().unwrap_or_else(|_| error("expected third arg to be int"));
    let msg: &str = &args[4];
    println!("({:?}, {:?}) @ {:?}px: {:?}", x, y, font_size, msg);
}

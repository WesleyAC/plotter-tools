fn main() {
    let file = std::fs::read_to_string(std::env::args().collect::<Vec<String>>()[1].clone()).unwrap();
    let cmds = hpgl::parse_commands(file).unwrap();
    let canonical_cmds = hpgl::canonicalize(cmds);
    println!("{}", hpgl::canonical_commands_to_string(canonical_cmds));
}

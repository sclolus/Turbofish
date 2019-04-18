use std::io::Write;

#[derive(Copy, Clone, Debug)]
struct Symbol<'a> {
    addr: &'a str,
    t: &'a str,
    name: &'a str,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 && args.len() != 2 {
        panic!("bad argument number, args: {:?}", args);
    }
    let mut res_file = std::fs::File::create(&args[1]).unwrap();
    let nm_file = std::fs::read_to_string(&args[2]).unwrap();
    let symbols: Vec<Symbol> = nm_file
        .lines()
        .map(|x| {
            let mut s = x.split_whitespace();
            Symbol {
                addr: s.next().unwrap(),
                t: s.next().unwrap(),
                name: s.next().unwrap_or(""),
            }
        })
        .filter(|s| s.t != "U")
        .collect();
    let nb_symbol = symbols.len();
    res_file
        .write_fmt(format_args!("#define FN_DIR_LEN	{}\n", nb_symbol))
        .unwrap();
    res_file
        .write_fmt(format_args!(
            "static struct symbol_entry function_directory[{}] = {{",
            nb_symbol,
        ))
        .unwrap();
    for s in symbols {
        res_file
            .write_fmt(format_args!(
                "\t{{0x{}, '{}', \"{}\"}},\n",
                s.addr, s.t, s.name
            ))
            .unwrap();
    }
    res_file.write_fmt(format_args!("}};\n")).unwrap();
}

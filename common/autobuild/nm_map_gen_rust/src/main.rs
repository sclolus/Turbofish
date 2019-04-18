use std::io::Write;

#[derive(Copy, Clone, Debug)]
struct Symbol<'a> {
    addr: &'a str,
    t: &'a str,
    name: &'a str,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2 || args.len() == 3, "bad number of args");
    let mut res_file = std::fs::File::create(&args[1]).unwrap();
    if args.len() == 2 {
        let nb_symbol = 0;
        write!(res_file, "#define FN_DIR_LEN	{}\n", nb_symbol).unwrap();
        write!(res_file, "static struct symbol_entry function_directory[{}] = {{}};\n", nb_symbol,).unwrap();
    } else if args.len() == 3 {
        let nm_file = std::fs::read_to_string(&args[2]).unwrap();
        let symbols: Vec<Symbol> = nm_file
            .lines()
            .map(|x| {
                let mut s = x.split_whitespace();
                Symbol { addr: s.next().unwrap(), t: s.next().unwrap(), name: s.next().unwrap_or("") }
            })
            .filter(|sym| sym.t != "U")
            .collect();
        let nb_symbol = symbols.len();
        write!(res_file, "#define FN_DIR_LEN	{}\n", nb_symbol).unwrap();
        write!(res_file, "static struct symbol_entry function_directory[{}] = {{", nb_symbol,).unwrap();
        for s in symbols {
            write!(res_file, "\t{{0x{}, '{}', \"{}\"}},\n", s.addr, s.t, s.name).unwrap();
        }
        write!(res_file, "}};\n").unwrap();
    }
}

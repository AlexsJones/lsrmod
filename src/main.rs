mod types;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format, Attr, color};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use clap::Parser;

const MODULE_PATH: &str = "/proc/modules";
/*
Module information format
The first column contains the name of the module.
The second column refers to the memory size of the module, in bytes.
The third column lists how many instances of the module are currently loaded. A value of zero represents an unloaded module.
The fourth column states if the module depends upon another module to be present in order to function, and lists those other modules.
The fifth column lists what load state the module is in: Live, Loading, or Unloading are the only possible values.
The sixth column lists the current kernel memory offset for the loaded module. This information can be useful for debugging purposes, or for profiling tools such as oprofile.
 */
fn populate(l: &str) -> types::Module {
    let s: Vec<&str> = l.split(' ').collect::<Vec<&str>>();
    types::Module {
        name: s[0].to_string(),
        memory: s[1].to_string(),
        instances: s[2].to_string().parse::<i32>().unwrap(),
        depends_on: s[3].to_string().replace(" ",""),
        state: types::ModuleState::from_str(s[4]).unwrap(),
        memory_offset: s[5].to_string(),
    }
}
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json: bool
}
fn main() {
    let a = Args::parse();
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[format::LinePosition::Top,
            format::LinePosition::Bottom],
                    format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);
    let path = Path::new(MODULE_PATH);
    table.set_titles(row!["Name", "Memory(b)", "Instances", "Depends on","State","Memory Offset"]);
    let r = fs::read_to_string(path).unwrap();
    let mut processed_modules = r.lines().map(|x | populate(&x)).collect::<Vec<types::Module>>();

    if !a.json {
        for t in processed_modules {
            table.add_row(Row::new(vec![
                Cell::new(t.name.as_str()).with_style(Attr::ForegroundColor(color::BLUE)),
                Cell::new(t.memory.as_str()),
                Cell::new(t.instances.to_string().as_str()),
                Cell::new(t.depends_on.as_str()),
                Cell::new(t.state.to_string().as_str()),
                Cell::new(t.memory_offset.as_str())
            ]));
        }
        table.printstd();
    }else {
        let json = serde_json::to_string(&processed_modules).unwrap();
        println!("{}", json);
    }
}

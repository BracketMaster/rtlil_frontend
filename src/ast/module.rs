use super::*;
use getset::*;

#[derive(Debug)]
pub enum ModuleStmt {
    Empty,
    Param(String),
    ParamVal((String, Const)),
    Wire(Wire),
    Memory(Memory),
    Cell(Cell),
    Connect(Connect),
    Process(Process),
}

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Module {
    ident: String,
    attrs: HashMap<String, Const>,
    params: HashMap<String, Const>,
    wires: Vec<Wire>,
    cells: Vec<Cell>,
    processes: Vec<Process>,
    memories: Vec<Memory>,
    connects: Vec<Connect>,
}

impl Module {
    pub fn new(ident: String, stmts: Vec<ModuleStmt>) -> Self {
        let mut r = Self {
            ident,
            ..Self::default()
        };
        for stmt in stmts {
            match stmt {
                ModuleStmt::Param(n) => {
                    r.params.insert(n, Const::Empty);
                },
                ModuleStmt::ParamVal((k, v)) => {
                    r.params.insert(k, v);
                },
                ModuleStmt::Wire(n) => r.wires.push(n),
                ModuleStmt::Cell(n) => r.cells.push(n),
                ModuleStmt::Process(n) => r.processes.push(n),
                ModuleStmt::Memory(n) => r.memories.push(n),
                _ => (),
            };
        }
        r
    }
}

use super::*;
use getset::*;

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct ProcessSwitchCase {
    sigs: Vec<SigSpec>,
    assign: Vec<(SigSpec, SigSpec)>,
    switch: Vec<ProcessSwitch>,
    attrs: HashMap<String, Const>,
}

impl ProcessSwitchCase {
    pub fn new(sigs: Vec<SigSpec>, stmts: Vec<ProcessStmt>) -> Self {
        let mut r = Self {
            sigs,
            ..Self::default()
        };
        for stmt in stmts {
            match stmt {
                ProcessStmt::Assign(v) => {
                    r.assign.push(v);
                }
                ProcessStmt::Switch(v) => {
                    r.switch.push(v);
                }
                _ => (),
            }
        }
        r
    }
}

#[derive(Debug, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct ProcessSwitch {
    sig: SigSpec,
    cases: Vec<ProcessSwitchCase>,
    attrs: HashMap<String, Const>,
}

impl ProcessSwitch {
    pub fn new(sig: SigSpec, cases: Vec<ProcessSwitchCase>) -> Self {
        Self {
            sig,
            cases,
            attrs: HashMap::new(),
        }
    }
}

pub enum ProcessStmt {
    Empty,
    Assign((SigSpec, SigSpec)),
    Switch(ProcessSwitch),
}

#[derive(Debug)]
pub enum ProcessSyncType {
    Always,
    Global,
    Init,
    Low(SigSpec),
    High(SigSpec),
    Posedge(SigSpec),
    Negedge(SigSpec),
    Edge(SigSpec),
}

#[derive(Debug, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct ProcessSync {
    tp: ProcessSyncType,
    updates: Vec<(SigSpec, SigSpec)>,
    attrs: HashMap<String, Const>,
}

impl ProcessSync {
    pub fn new(tp: ProcessSyncType, updates: Vec<(SigSpec, SigSpec)>) -> Self {
        Self {
            tp,
            updates,
            attrs: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Process {
    id: String,
    assign: Vec<(SigSpec, SigSpec)>,
    switch: Vec<ProcessSwitch>,
    syncs: Vec<ProcessSync>,
    attrs: HashMap<String, Const>,
}

impl Process {
    pub fn new(id: String, stmts: Vec<ProcessStmt>, syncs: Vec<ProcessSync>) -> Self {
        let mut r = Self {
            id,
            syncs,
            ..Self::default()
        };
        for stmt in stmts {
            match stmt {
                ProcessStmt::Assign(v) => {
                    r.assign.push(v);
                }
                ProcessStmt::Switch(v) => {
                    r.switch.push(v);
                }
                _ => (),
            }
        }
        r
    }
}

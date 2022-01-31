// Copyright (c) 2020 xhe
// Copyright (c) 2020 Yehowshua

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::AtomicUsize;

pub static AUTOIDX: AtomicUsize = AtomicUsize::new(0);

mod design;
pub use design::*;

mod module;
pub use module::*;

mod wire;
pub use wire::*;

mod memory;
pub use memory::*;

mod cell;
pub use cell::*;

mod process;
pub use process::*;

mod connect;
pub use connect::*;

mod sigspec;
pub use sigspec::*;

mod constant;
pub use constant::*;

mod signal;
pub use signal::*;


pub enum Node<'a> {
    Design(&'a Design),
    Module(&'a Module),
    ModuleStmt(&'a ModuleStmt),
    Wire(&'a Wire),
    Memory(&'a Memory),
    Cell(&'a Cell),
    Process(&'a Process),
    ProcessSync(&'a ProcessSync),
    ProcessSwitch(&'a ProcessSwitch),
    ProcessSwitchCase(&'a ProcessSwitchCase),
    Connect(&'a Connect),
    Signal(&'a Signal),
}
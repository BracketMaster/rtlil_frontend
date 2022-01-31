use super::ast::*;
use anyhow::Result;

pub trait Visitor {
	fn enter(&mut self, n: Node) -> Result<()>;
	fn exit(&mut self, n: Node) -> Result<()>;
}

pub trait Visit {
	fn visit(&self, ctx: &mut dyn Visitor) -> Result<()>;
}

impl Visit for Design {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
	    ctx	.enter(Node::Design(&self))?;
		for n in self.modules() {
			n.visit(ctx)?;
		}
		ctx.exit(Node::Design(&self))?;
		Ok(())
	}
}

impl Visit for Module {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
        ctx.enter(Node::Module(&self))?;
        for n in self.wires() {
            n.visit(ctx)?;
        }
        for n in self.cells() {
            n.visit(ctx)?;
        }
        for n in self.processes() {
            n.visit(ctx)?;
        }
        ctx.exit(Node::Module(&self))?;
        Ok(())
    }
}

impl Visit for Wire {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
        ctx.enter(Node::Wire(self))?;
        ctx.exit(Node::Wire(self))?;
        Ok(())
    }
}

impl Visit for Cell {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
        ctx.enter(Node::Cell(self))?;
        ctx.exit(Node::Cell(self))?;
        Ok(())
    }
}

impl Visit for Process {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
        ctx.enter(Node::Process(self))?;
        for s in self.switch() {
            s.visit(ctx)?;
        }
        for s in self.syncs() {
            s.visit(ctx)?;
        }
        ctx.exit(Node::Process(self))?;
        Ok(())
    }
}

impl Visit for ProcessSync {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
        ctx.enter(Node::ProcessSync(self))?;
        ctx.exit(Node::ProcessSync(self))?;
        Ok(())
    }
}

impl Visit for ProcessSwitch {
    fn visit(&self, ctx: &mut dyn Visitor) -> Result<()> {
       ctx .enter(Node::ProcessSwitch(&self))?;
        for c in self.cases() {
            c.visit(ctx)?;
        }
       ctx .exit(Node::ProcessSwitch(&self))?;
        Ok(())
    }
}

impl Visit for ProcessSwitchCase {
    fn visit(&self, ctx : &mut dyn Visitor) -> Result<()> {
       ctx .enter(Node::ProcessSwitchCase(&self))?;
        for s in self.switch() {
            s.visit(ctx)?;
        }
       ctx .exit(Node::ProcessSwitchCase(&self))?;
        Ok(())
    }
}
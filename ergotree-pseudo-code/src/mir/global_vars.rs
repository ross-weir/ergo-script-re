use ergotree_ir::mir::{expr::Expr, global_vars::GlobalVars};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for GlobalVars {
    fn pseudo_code(
        &self,
        _ctx: &mut GeneratorContext,
        _stack: &mut Vec<&Expr>,
    ) -> Result<String, PseudoCodeError> {
        let code = match self {
            GlobalVars::Inputs => "INPUTS",
            GlobalVars::Outputs => "OUTPUTS",
            GlobalVars::Height => "HEIGHT",
            GlobalVars::SelfBox => "SELF",
            GlobalVars::MinerPubKey => todo!(),
            GlobalVars::GroupGenerator => todo!(),
        };

        Ok(code.to_string())
    }
}

impl Element for GlobalVars {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_global_vars(self)
    }
}

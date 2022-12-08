use ergotree_ir::mir::{expr::Expr, val_use::ValUse};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ValUse {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        _stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let code = ctx.dfa.name_for_id(self.val_id.0);

        Ok(code)
    }
}

impl Element for ValUse {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_val_use(self)
    }
}

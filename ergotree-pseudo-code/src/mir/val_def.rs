use ergotree_ir::mir::{expr::Expr, val_def::ValDef};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ValDef {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let var_name = ctx.dfa.name_for_id(self.id.0);
        let rhs = self.rhs.pseudo_code(ctx, stack)?;

        // TODO: we could add the type of varible by using rhs
        Ok(format!("val {var_name} = {rhs}"))
    }
}

impl Element for ValDef {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_val_def(self)
    }
}

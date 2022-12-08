use ergotree_ir::mir::{atleast::Atleast, expr::Expr};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for Atleast {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let bound_code = self.bound.pseudo_code(ctx, stack)?;
        let input_code = self.input.pseudo_code(ctx, stack)?;

        Ok(format!("atLeast({bound_code}, {input_code})"))
    }
}

impl Element for Atleast {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_atleast(self)
    }
}

use ergotree_ir::mir::{coll_size::SizeOf, expr::Expr};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for SizeOf {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;

        Ok(format!("{input_code}.size"))
    }
}

impl Element for SizeOf {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_size_of(self)
    }
}

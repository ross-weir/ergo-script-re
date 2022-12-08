use ergotree_ir::mir::{coll_by_index::ByIndex, expr::Expr};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ByIndex {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;
        let idx_code = self.index.pseudo_code(ctx, stack)?;

        let code = if let Some(default) = &self.default {
            let default_expr = &*default;
            let default_code = default_expr.pseudo_code(ctx, stack)?;

            format!("{input_code}.getOrElse({idx_code}, {default_code})")
        } else {
            format!("{input_code}({idx_code})")
        };

        Ok(code)
    }
}

impl Element for ByIndex {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_by_index(self)
    }
}

use ergotree_ir::mir::{block::BlockValue, expr::Expr};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for BlockValue {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let stmts: Vec<String> = self
            .items
            .iter()
            .map(|e| e.pseudo_code(ctx, stack).unwrap())
            .collect();
        // result_code can produce pseudo_vars which are written out next
        let result_code = self.result.pseudo_code(ctx, stack)?;
        let mut pseudo_vars = ctx
            .pseudo_var_decls
            .drain(..)
            .collect::<Vec<_>>()
            .join("\n");
        if pseudo_vars.len() > 0 {
            pseudo_vars.insert_str(0, "\n");
        }
        let stmts_code = stmts.join("\n");
        let code = format!("{{\n{stmts_code}{pseudo_vars}\n\n{result_code}\n}}");

        Ok(code)
    }
}

impl Element for BlockValue {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_block_value(self);
        self.items.iter().for_each(|e| e.accept(visitor))
    }
}

use ergotree_ir::mir::{
    expr::Expr,
    func_value::{FuncArg, FuncValue},
};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    ErgoScript, GeneratorContext, PseudoCode,
};

impl PseudoCode for FuncArg {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        _stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let arg_name = ctx.dfa.name_for_id(self.idx.0);
        let tpe = self.tpe.ergo_script();

        Ok(format!("{arg_name}: {tpe}"))
    }
}

impl PseudoCode for FuncValue {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let args = self
            .args()
            .iter()
            .map(|e| e.pseudo_code(ctx, stack).unwrap())
            .collect::<Vec<_>>()
            .join(", ");
        let body = self.body().pseudo_code(ctx, stack)?;

        // TODO handle ctx.pseudo_var_decls having values
        // if we get here and pseudo_var_decls exist it means we had an inline if IR node with no block IR node in the func body
        // wrap the func body in a block and write out the var decls

        Ok(format!("{{ ({args}) => {body} }}"))
    }
}

impl Element for FuncValue {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_func_value(self)
    }
}

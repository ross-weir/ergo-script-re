use ergotree_ir::mir::func_value::{FuncArg, FuncValue};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    ErgoScript, GeneratorContext, PseudoCode,
};

impl PseudoCode for FuncArg {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let arg_name = ctx.dfa.name_for_id(self.idx.0);
        let tpe = self.tpe.ergo_script();

        Ok(format!("{arg_name}: {tpe}"))
    }
}

impl PseudoCode for FuncValue {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let args = self
            .args()
            .iter()
            .map(|e| e.pseudo_code(ctx).unwrap())
            .collect::<Vec<_>>()
            .join(", ");
        let body = self.body().pseudo_code(ctx)?;

        Ok(format!("{{ ({args}) => {body} }}"))
    }
}

impl Element for FuncValue {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_func_value(self)
    }
}

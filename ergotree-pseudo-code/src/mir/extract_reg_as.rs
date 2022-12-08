use ergotree_ir::mir::{expr::Expr, extract_reg_as::ExtractRegisterAs};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    ErgoScript, GeneratorContext, PseudoCode,
};

impl PseudoCode for ExtractRegisterAs {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;
        let reg = self.register_id;
        let tpe = self.elem_tpe.ergo_script();

        Ok(format!("{input_code}.R{reg}[{tpe}]"))
    }
}

impl Element for ExtractRegisterAs {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_extract_reg_as(self)
    }
}

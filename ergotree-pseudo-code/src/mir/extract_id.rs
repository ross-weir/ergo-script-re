use ergotree_ir::mir::{expr::Expr, extract_id::ExtractId};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ExtractId {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;

        Ok(format!("{input_code}.id"))
    }
}

impl Element for ExtractId {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_extract_id(self)
    }
}

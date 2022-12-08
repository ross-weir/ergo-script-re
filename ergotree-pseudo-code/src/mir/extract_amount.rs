use ergotree_ir::mir::{expr::Expr, extract_amount::ExtractAmount};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ExtractAmount {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;

        Ok(format!("{input_code}.value"))
    }
}

impl Element for ExtractAmount {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_extract_amount(self)
    }
}

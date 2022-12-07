use ergotree_ir::mir::extract_amount::ExtractAmount;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ExtractAmount {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx)?;

        Ok(format!("{input_code}.value"))
    }
}

impl Element for ExtractAmount {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_extract_amount(self)
    }
}

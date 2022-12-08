use ergotree_ir::mir::{expr::Expr, option_get::OptionGet};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for OptionGet {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;

        Ok(format!("{input_code}.get"))
    }
}

impl Element for OptionGet {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_option_get(self)
    }
}

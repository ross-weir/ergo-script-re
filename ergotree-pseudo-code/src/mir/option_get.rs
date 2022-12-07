use ergotree_ir::mir::option_get::OptionGet;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for OptionGet {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx)?;

        Ok(format!("{input_code}.get"))
    }
}

impl Element for OptionGet {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_option_get(self)
    }
}

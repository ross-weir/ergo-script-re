use ergotree_ir::mir::coll_filter::Filter;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for Filter {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx)?;
        let cond_code = self.condition.pseudo_code(ctx)?;

        Ok(format!("{input_code}.filter({cond_code})"))
    }
}

impl Element for Filter {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_filter(self)
    }
}

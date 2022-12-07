use ergotree_ir::mir::val_use::ValUse;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for ValUse {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let code = ctx.dfa.name_for_id(self.val_id.0);

        Ok(code)
    }
}

impl Element for ValUse {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_val_use(self)
    }
}

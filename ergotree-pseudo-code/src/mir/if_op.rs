use ergotree_ir::mir::if_op::If;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for If {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let cond_code = self.condition.pseudo_code(ctx)?;
        let true_code = self.true_branch.pseudo_code(ctx)?;
        let false_code = self.false_branch.pseudo_code(ctx)?;

        Ok(format!(
            "if ({cond_code}) {{ {true_code} }} else {{ {false_code} }}"
        ))
    }
}

impl Element for If {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_if(self)
    }
}

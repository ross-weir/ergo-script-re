use ergotree_ir::mir::{constant::ConstantPlaceholder, expr::Expr};

use crate::{error::PseudoCodeError, GeneratorContext, PseudoCode};

impl PseudoCode for ConstantPlaceholder {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        _stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let v = ctx
            .tree
            .get_constant(self.id.try_into().unwrap())
            .unwrap()
            .unwrap();
        Ok(format!("{:?}", v))
    }
}

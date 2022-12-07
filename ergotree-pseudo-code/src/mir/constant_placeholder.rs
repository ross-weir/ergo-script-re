use ergotree_ir::mir::constant::ConstantPlaceholder;

use crate::{error::PseudoCodeError, GeneratorContext, PseudoCode};

impl PseudoCode for ConstantPlaceholder {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let v = ctx
            .tree
            .get_constant(self.id.try_into().unwrap())
            .unwrap()
            .unwrap();
        Ok(format!("{:?}", v))
    }
}

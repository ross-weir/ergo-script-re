use ergotree_ir::mir::property_call::PropertyCall;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for PropertyCall {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let obj_code = self.obj.pseudo_code(ctx)?;
        let prop_name = self.method.name();

        Ok(format!("{obj_code}.{prop_name}"))
    }
}

impl Element for PropertyCall {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_property_call(self)
    }
}

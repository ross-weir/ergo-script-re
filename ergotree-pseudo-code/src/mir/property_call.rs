use ergotree_ir::mir::{expr::Expr, property_call::PropertyCall};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for PropertyCall {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let obj_code = self.obj.pseudo_code(ctx, stack)?;
        let prop_name = self.method.name();

        Ok(format!("{obj_code}.{prop_name}"))
    }
}

impl Element for PropertyCall {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_property_call(self)
    }
}

use ergotree_ir::mir::{expr::Expr, select_field::SelectField};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for SelectField {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let input_code = self.input.pseudo_code(ctx, stack)?;
        // workaround: the index is private, use a pub method to get zero based index then add 1 for the tuple field
        let idx = self.field_index.zero_based_index() + 1;

        Ok(format!("{input_code}._{idx}"))
    }
}

impl Element for SelectField {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_select_field(self)
    }
}

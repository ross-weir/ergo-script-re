use ergotree_ir::mir::block::BlockValue;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for BlockValue {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let stmts: Vec<String> = self
            .items
            .iter()
            .map(|e| e.pseudo_code(ctx).unwrap())
            .collect();
        let stmts_code = stmts.join("\n\t");
        let result_code = self.result.pseudo_code(ctx)?;
        let code = format!("{{\n\t{stmts_code}\n\n\t{result_code}\n}}");

        Ok(code)
    }
}

impl Element for BlockValue {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_block_value(self);
        self.items.iter().for_each(|e| e.accept(visitor))
    }
}

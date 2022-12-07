use ergotree_ir::mir::collection::Collection;

use crate::{error::PseudoCodeError, GeneratorContext, PseudoCode};

impl PseudoCode for Collection {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        let vals: Vec<String> = match self {
            Collection::BoolConstants(bools) => bools.iter().map(|b| b.to_string()).collect(),
            Collection::Exprs { elem_tpe: _, items } => {
                items.iter().map(|e| e.pseudo_code(ctx).unwrap()).collect()
            }
        };
        let joined_vals = vals.join(", ");

        Ok(format!("Coll({joined_vals})"))
    }
}

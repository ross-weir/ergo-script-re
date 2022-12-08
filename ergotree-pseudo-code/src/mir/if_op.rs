use ergotree_ir::mir::{expr::Expr, if_op::If};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for If {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        let cond_code = self.condition.pseudo_code(ctx, stack)?;
        let true_code = self.true_branch.pseudo_code(ctx, stack)?;
        let false_code = self.false_branch.pseudo_code(ctx, stack)?;
        let if_code = format!("if ({cond_code}) {{ {true_code} }} else {{ {false_code} }}");
        let parent = stack.get(stack.len() - 2);

        if let Some(p) = parent {
            if let Expr::BinOp(_) = p {
                // lift the if op into a var decl since ifs cannot be inlined in a bin op
                let var_name = ctx.dfa.new_pseudo_var();
                let var_decl = format!("val {var_name} = {if_code}");
                ctx.pseudo_var_decls.push(var_decl);

                Ok(var_name)
            } else {
                Ok(if_code)
            }
        } else {
            Ok(if_code)
        }
    }
}

impl Element for If {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_if(self)
    }
}

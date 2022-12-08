use ergotree_ir::mir::{constant::Constant, expr::Expr};

use crate::{error::PseudoCodeError, GeneratorContext, PseudoCode};

impl PseudoCode for Constant {
    fn pseudo_code(
        &self,
        _ctx: &mut GeneratorContext,
        _stack: &mut Vec<&Expr>,
    ) -> Result<String, PseudoCodeError> {
        Ok(format!("{:?}", self.v))
    }
}

#[cfg(test)]
mod tests {
    use ergotree_ir::mir::expr::Expr;

    use super::*;

    #[test]
    fn test_constant_bool_true() {
        let expr: Expr = true.into();
        let mut ctx = GeneratorContext::from_expr(expr.clone());

        assert_eq!(expr.pseudo_code(&mut ctx, &mut vec![]).unwrap(), "true")
    }

    #[test]
    fn test_constant_bool_false() {
        let expr: Expr = false.into();
        let mut ctx = GeneratorContext::from_expr(expr.clone());

        assert_eq!(expr.pseudo_code(&mut ctx, &mut vec![]).unwrap(), "false")
    }
}

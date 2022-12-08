use ergotree_ir::mir::{bool_to_sigma::BoolToSigmaProp, expr::Expr};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for BoolToSigmaProp {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        self.input.pseudo_code(ctx, stack)
    }
}

impl Element for BoolToSigmaProp {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_bool_to_sigma_prop(self);
        self.input.accept(visitor)
    }
}

#[cfg(test)]
mod tests {
    use ergotree_ir::mir::{
        bin_op::{BinOp, BinOpKind, LogicalOp},
        expr::Expr,
    };

    use super::*;

    #[test]
    fn test_sigma_prop() {
        let left: Expr = true.into();
        let right: Expr = false.into();
        let input: Expr = BinOp {
            kind: BinOpKind::Logical(LogicalOp::And),
            left: Box::new(left),
            right: Box::new(right),
        }
        .into();
        let expr: Expr = BoolToSigmaProp {
            input: Box::new(input),
        }
        .into();
        let mut ctx = GeneratorContext::from_expr(expr.clone());

        assert_eq!(
            expr.pseudo_code(&mut ctx, &mut vec![]).unwrap(),
            "true && false"
        )
    }
}

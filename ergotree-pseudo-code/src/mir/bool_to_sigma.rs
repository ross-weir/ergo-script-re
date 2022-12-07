use ergotree_ir::mir::bool_to_sigma::BoolToSigmaProp;

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for BoolToSigmaProp {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        self.input.pseudo_code(ctx)
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
        let ctx = GeneratorContext::from_expr(expr.clone());

        assert_eq!(expr.pseudo_code(&ctx).unwrap(), "true && false")
    }
}

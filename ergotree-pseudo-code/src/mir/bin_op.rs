use ergotree_ir::mir::{bin_op::BinOp, expr::Expr};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for BinOp {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError> {
        // TODO: this should probably have a ToString impl in ergotree_ir instead of this match statement
        let op = match self.kind {
            ergotree_ir::mir::bin_op::BinOpKind::Arith(e) => match e {
                ergotree_ir::mir::bin_op::ArithOp::Plus => "+",
                ergotree_ir::mir::bin_op::ArithOp::Minus => "-",
                ergotree_ir::mir::bin_op::ArithOp::Multiply => "*",
                ergotree_ir::mir::bin_op::ArithOp::Divide => "/",
                ergotree_ir::mir::bin_op::ArithOp::Max => todo!(),
                ergotree_ir::mir::bin_op::ArithOp::Min => todo!(),
                ergotree_ir::mir::bin_op::ArithOp::Modulo => todo!(),
            },
            ergotree_ir::mir::bin_op::BinOpKind::Relation(e) => match e {
                ergotree_ir::mir::bin_op::RelationOp::Eq => "==",
                ergotree_ir::mir::bin_op::RelationOp::NEq => "!=",
                ergotree_ir::mir::bin_op::RelationOp::Ge => ">=",
                ergotree_ir::mir::bin_op::RelationOp::Gt => ">",
                ergotree_ir::mir::bin_op::RelationOp::Le => "<",
                ergotree_ir::mir::bin_op::RelationOp::Lt => "<=",
            },
            ergotree_ir::mir::bin_op::BinOpKind::Logical(e) => match e {
                ergotree_ir::mir::bin_op::LogicalOp::And => "&&",
                ergotree_ir::mir::bin_op::LogicalOp::Or => "||",
                ergotree_ir::mir::bin_op::LogicalOp::Xor => todo!(),
            },
            ergotree_ir::mir::bin_op::BinOpKind::Bit(e) => match e {
                ergotree_ir::mir::bin_op::BitOp::BitOr => "|",
                ergotree_ir::mir::bin_op::BitOp::BitAnd => "&",
                ergotree_ir::mir::bin_op::BitOp::BitXor => todo!(),
            },
        };
        let left = &*self.left;
        let left_code = if let Expr::BinOp(_) = left {
            format!("({})", left.pseudo_code(ctx)?)
        } else {
            left.pseudo_code(ctx)?
        };

        let right = &*self.right;
        let right_code = if let Expr::BinOp(_) = right {
            format!("({})", right.pseudo_code(ctx)?)
        } else {
            right.pseudo_code(ctx)?
        };

        Ok(format!("{left_code} {op} {right_code}").to_string())
    }
}

impl Element for BinOp {
    fn accept<T: Visitor>(&self, visitor: &mut T) {
        visitor.visit_bin_op(self)
    }
}

#[cfg(test)]
mod tests {
    use ergotree_ir::mir::{
        bin_op::{BinOpKind, LogicalOp},
        expr::Expr,
    };

    use super::*;

    #[test]
    fn test_and_bin_op() {
        let left: Expr = true.into();
        let right: Expr = false.into();
        let expr: Expr = BinOp {
            kind: BinOpKind::Logical(LogicalOp::And),
            left: Box::new(left),
            right: Box::new(right),
        }
        .into();
        let ctx = GeneratorContext::from_expr(expr.clone());

        assert_eq!(expr.pseudo_code(&ctx).unwrap(), "true && false")
    }
}

use ergotree_ir::mir::{
    bin_op::{ArithOp, BinOp, BinOpKind, BitOp, LogicalOp, RelationOp},
    expr::Expr,
};

use crate::{
    error::PseudoCodeError,
    visitor::{Element, Visitor},
    GeneratorContext, PseudoCode,
};

impl PseudoCode for BinOp {
    fn pseudo_code<'a>(
        &'a self,
        ctx: &mut GeneratorContext,
        stack: &mut Vec<&'a Expr>,
    ) -> Result<String, PseudoCodeError> {
        // TODO: this should probably have a ToString impl in ergotree_ir instead of this match statement
        let op = match self.kind {
            BinOpKind::Arith(e) => match e {
                ArithOp::Plus => "+",
                ArithOp::Minus => "-",
                ArithOp::Multiply => "*",
                ArithOp::Divide => "/",
                ArithOp::Max => "max",
                ArithOp::Min => "min",
                ArithOp::Modulo => "%",
            },
            BinOpKind::Relation(e) => match e {
                RelationOp::Eq => "==",
                RelationOp::NEq => "!=",
                RelationOp::Ge => ">=",
                RelationOp::Gt => ">",
                RelationOp::Le => "<",
                RelationOp::Lt => "<=",
            },
            BinOpKind::Logical(e) => match e {
                LogicalOp::And => "&&",
                LogicalOp::Or => "||",
                LogicalOp::Xor => "^",
            },
            BinOpKind::Bit(e) => match e {
                BitOp::BitOr => "|",
                BitOp::BitAnd => "&",
                BitOp::BitXor => todo!(),
            },
        };

        if let BinOpKind::Arith(arith) = self.kind {
            match arith {
                ArithOp::Max | ArithOp::Min => {
                    return Ok(format!(
                        "{op}({}, {})",
                        self.left.pseudo_code(ctx, stack)?,
                        self.right.pseudo_code(ctx, stack)?
                    ))
                }
                _ => (),
            }
        }

        let left = &*self.left;
        let left_code = if let Expr::BinOp(_) = left {
            format!("({})", left.pseudo_code(ctx, stack)?)
        } else {
            left.pseudo_code(ctx, stack)?
        };

        let right = &*self.right;
        let right_code = if let Expr::BinOp(_) = right {
            format!("({})", right.pseudo_code(ctx, stack)?)
        } else {
            right.pseudo_code(ctx, stack)?
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
        let mut ctx = GeneratorContext::from_expr(expr.clone());

        assert_eq!(
            expr.pseudo_code(&mut ctx, &mut vec![]).unwrap(),
            "true && false"
        )
    }
}

use crate::{error::PseudoCodeError, visitor::Element, GeneratorContext, PseudoCode};
use ergotree_ir::{ergo_tree::ErgoTree, mir::expr::Expr};

pub struct PseudoCodeGenerator {}

impl PseudoCodeGenerator {
    pub fn generate_for_ergotree(&self, tree: &ErgoTree) -> Result<String, PseudoCodeError> {
        let mut ctx = GeneratorContext::from_tree(tree.clone());
        let mut stack: Vec<&Expr> = vec![];
        let expr = tree.proposition().unwrap();

        // perform data flow analysis by recursively visiting all IR nodes in the tree
        expr.accept(&mut ctx.dfa);
        let code = expr.pseudo_code(&mut ctx, &mut stack)?;

        self.finalize(&mut ctx, code)
    }

    fn finalize(
        &self,
        ctx: &mut GeneratorContext,
        code: String,
    ) -> Result<String, PseudoCodeError> {
        // if ctx.pseudo_var_decls contains values it means we encountered IR nodes that required
        // pseudo vars but the ergo tree didn't contain a Block IR node.
        // wrap the code in { } and write out the var decls
        if ctx.pseudo_var_decls.len() > 0 {
            let pseudo_vars = ctx
                .pseudo_var_decls
                .drain(..)
                .collect::<Vec<_>>()
                .join("\n");

            Ok(format!("{{\n{pseudo_vars}\n\n{code}\n}}"))
        } else {
            Ok(code)
        }
    }
}

#[cfg(test)]
mod tests {
    use ergotree_ir::serialization::SigmaSerializable;
    use pretty_assertions::assert_eq;

    use super::*;

    fn tree_from_hex(s: &str) -> ErgoTree {
        let tree_bytes = base16::decode(s.as_bytes()).unwrap();
        ErgoTree::sigma_parse_bytes(&tree_bytes).unwrap()
    }

    fn check_gen(tree_hex: &str, expected_code: &str) {
        let tree = tree_from_hex(tree_hex);
        let gen = PseudoCodeGenerator {};

        assert_eq!(gen.generate_for_ergotree(&tree).unwrap(), expected_code)
    }

    fn check_gen_trim(tree_hex: &str, expected_code: &str) {
        let expected = expected_code.replace("\t", "").replace(" ", "");
        let tree = tree_from_hex(tree_hex);
        let gen = PseudoCodeGenerator {};
        let actual = gen
            .generate_for_ergotree(&tree)
            .unwrap()
            .replace("\t", "")
            .replace(" ", "");

        assert_eq!(expected.trim(), actual.trim());
    }

    #[test]
    fn test_basic_bool_tree() {
        // {
        //     val a = true
        //     val b = false

        //     a && b
        // }
        check_gen("1000d1ed8501", "true && false")
    }

    #[test]
    fn test_atleast_with_constant_placeholders() {
        // {
        // atLeast(
        //   3,
        //   Coll(
        //     PK("9f8ZQt1Sue6W5ACdMSPRzsHj3jjiZkbYy3CEtB4BisxEyk4RsNk"),
        //     PK("9hFWPyhCJcw4KQyCGu4yAGfC1ieRAKyFg24FKjLJK2uDgA873uq"),
        //     PK("9fdVP2jca1e5nCTT6q9ijZLssGj6v4juY8gEAxUhp7YTuSsLspS"),
        //     PK("9gAKeRu1W4Dh6adWXnnYmfqjCTnxnSMtym2LPPMPErCkusCd6F3"),
        //     PK("9gmNsqrqdSppLUBqg2UzREmmivgqh1r3jmNcLAc53hk3YCvAGWE")
        //   )
        //  )
        // }
        check_gen(
            "1006040608cd02509a6378da277877eabe7fe56a7157e72ec87697d7ef92574684f3d50e5a21da08cd0367c585c655e6e34b2c0c79cff24a997e60686c9770b6a1f7f8689c993c5340c708cd02924b14b8df230f350970e0b735780b41fd65798c1a5243e3352854a6fc048ca308cd02d84d2e1ca735ce23f224bd43cd43ed67053356713a8a6920965b3bde933648dc08cd0327e65711a59378c59359c3e1d0f7abe906479eccb76094e50fe79d743ccc15e698730083050873017302730373047305",
            "atLeast(3, Coll(SigmaProp(ProofOfKnowledge(ProveDlog(ProveDlog { h: EC:02509a6378da277877eabe7fe56a7157e72ec87697d7ef92574684f3d50e5a21da }))), SigmaProp(ProofOfKnowledge(ProveDlog(ProveDlog { h: EC:0367c585c655e6e34b2c0c79cff24a997e60686c9770b6a1f7f8689c993c5340c7 }))), SigmaProp(ProofOfKnowledge(ProveDlog(ProveDlog { h: EC:02924b14b8df230f350970e0b735780b41fd65798c1a5243e3352854a6fc048ca3 }))), SigmaProp(ProofOfKnowledge(ProveDlog(ProveDlog { h: EC:02d84d2e1ca735ce23f224bd43cd43ed67053356713a8a6920965b3bde933648dc }))), SigmaProp(ProofOfKnowledge(ProveDlog(ProveDlog { h: EC:0327e65711a59378c59359c3e1d0f7abe906479eccb76094e50fe79d743ccc15e6 })))))"
        )
    }

    #[test]
    fn test_inline_if_is_hoisted_to_var_decl_despite_no_block_ir() {
        // {
        // val a = if (SELF.value > 1000) { 1000 } else { 1500 }

        // a > 505
        // }

        let expected = r#"
{
       val pvar_1 = if (SELF.value > 1000) { 1000 } else { 1500 }

       pvar_1 > 505
}
"#;
        check_gen_trim(
            "100405d00f04d00f04b81704f207d1919591c1a77300730173027303",
            expected,
        )
    }

    #[test]
    fn test_script_pseudo_code3() {
        //{
        //     val a = OUTPUTS(0).R4[Byte].get
        //     val b = OUTPUTS(0).R5[Byte].get

        //     a != b
        // }
        let expected = r#"
{
    val var_1 = OUTPUTS(0)
    
    var_1.R4[Byte].get != var_1.R5[Byte].get
}
"#;
        check_gen_trim(
            "10010400d1d801d601b2a573000094e4c672010402e4c672010502",
            expected,
        )
    }

    #[test]
    fn test_script_pseudo_code4() {
        // {
        //     val recreatedBox = OUTPUTS.getOrElse(0, SELF)

        //     recreatedBox.tokens(0)._2 > 0L
        // }
        check_gen(
            "1003040004000500d1918cb2db6308b2a5730001a7730100027302",
            "OUTPUTS.getOrElse(0, SELF).tokens(0)._2 > 0",
        )
    }

    #[test]
    fn test_script_pseudo_code5() {
        // {
        //    val isRecreatedBabelFeeBox = {(outputBox: Box) => (
        //         outputBox.propositionBytes == SELF.propositionBytes &&
        //         outputBox.R6[Coll[Byte]].get == SELF.id
        //     )}

        //     val recreatedBox = OUTPUTS.filter(isRecreatedBabelFeeBox).getOrElse(0, SELF)
        //     val babelTokensDifference = recreatedBox.tokens(0)._2 - 1
        //     val nanoErgsDifference = SELF.value - recreatedBox.value

        //     babelTokensDifference * SELF.R5[Long].get >= nanoErgsDifference
        // }
        let expected = r#"
{
       val var_1 = OUTPUTS.filter({ (var_1: Box) => (var_1.propositionBytes == SELF.propositionBytes) && (var_1.R6[Coll[Byte]].get == SELF.id) }).getOrElse(0, SELF)

       ((var_1.tokens(0)._2 - 1) * SELF.R5[Long].get) >= (SELF.value - var_1.value)
}
"#;
        check_gen_trim("1003040004000502d1d801d601b2b5a5d9010163ed93c27201c2a793e4c67201060ec5a7730001a7929c998cb2db63087201730100027302e4c6a7050599c1a7c17201", expected)
    }

    #[test]
    fn test_script_pseudo_code6() {
        // {
        //         val babelFeeBoxCreator = SELF.R4[SigmaProp].get

        //         val isRecreatedBabelFeeBox = {(outputBox: Box) => (
        //             outputBox.propositionBytes == SELF.propositionBytes &&
        //             outputBox.R4[SigmaProp].get == SELF.R4[SigmaProp].get &&
        //             outputBox.R5[Long].get == SELF.R5[Long].get &&
        //             outputBox.R6[Coll[Byte]].get == SELF.id
        //         )}

        //         val recreatedBox = OUTPUTS.filter(isRecreatedBabelFeeBox).getOrElse(0, SELF)

        //         val nanoErgsDifference = SELF.value - recreatedBox.value
        //         val babelTokensBefore = if(SELF.tokens.size > 0){ SELF.tokens(0)._2 }else{ 0L }
        //         val babelTokensDifference = recreatedBox.tokens(0)._2 - babelTokensBefore
        //         val exchangeOK = babelTokensDifference * SELF.R5[Long].get >= nanoErgsDifference

        //         exchangeOK
        // }
        let expected = r#"
{
       val var_1 = OUTPUTS.filter({ (var_1: Box) => (((var_1.propositionBytes == SELF.propositionBytes) && (var_1.R4[SigmaProp].get == SELF.R4[SigmaProp].get)) && (var_1.R5[Long].get == SELF.R5[Long].get)) && (var_1.R6[Coll[Byte]].get == SELF.id) }).getOrElse(0, SELF)
       val var_2 = SELF.tokens
       val pvar_1 = if (var_2.size > 0) { var_2(0)._2 } else { 0 }

       ((var_1.tokens(0)._2 - pvar_1) * SELF.R5[Long].get) >= (SELF.value - var_1.value)
}"#;

        check_gen_trim("100504000400040004000500d1d802d601b2b5a5d9010163ededed93c27201c2a793e4c672010408e4c6a7040893e4c672010505e4c6a7050593e4c67201060ec5a7730001a7d602db6308a7929c998cb2db63087201730100029591b1720273028cb27202730300027304e4c6a7050599c1a7c17201", expected)
    }
}

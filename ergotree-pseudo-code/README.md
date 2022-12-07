## ergotree-pseudo-code

Pseudo code generator for compiled ergo trees. Attempts to create pseudo code roughly representing the ergo script that produced the tree. In some cases it can produce a script that is the equivilent (pseudo code compiles to the same ergo tree) but this is on a best effort basis.

Examples:

Original script:

```
{
    val a = OUTPUTS(0).R4[Byte].get
    val b = OUTPUTS(0).R5[Byte].get

    a != b
}
```

Pseudo code of the compiled ergo tree:

```
{
    val var_1 = OUTPUTS(0)
    
    var_1.R4[Byte].get != var_1.R5[Byte].get
}
```

Original script:

```
{
    val recreatedBox = OUTPUTS.getOrElse(0, SELF)

    recreatedBox.tokens(0)._2 > 0L
}
```

Pseudo code of the compiled ergo tree:

```
OUTPUTS.getOrElse(0, SELF).tokens(0)._2 > 0
```

Original script:

```
{
    val isRecreatedBabelFeeBox = {(outputBox: Box) => (
        outputBox.propositionBytes == SELF.propositionBytes &&
        outputBox.R6[Coll[Byte]].get == SELF.id
    )}

    val recreatedBox = OUTPUTS.filter(isRecreatedBabelFeeBox).getOrElse(0, SELF)
    val babelTokensDifference = recreatedBox.tokens(0)._2 - 1
    val nanoErgsDifference = SELF.value - recreatedBox.value

    babelTokensDifference * SELF.R5[Long].get >= nanoErgsDifference
}
```

Pseudo code of the compiled ergo tree:

```
{
       val var_1 = OUTPUTS.filter({ (var_1: Box) => (var_1.propositionBytes == SELF.propositionBytes) && (var_1.R6[Coll[Byte]].get == SELF.id) }).getOrElse(0, SELF)

       ((var_1.tokens(0)._2 - 1) * SELF.R5[Long].get) >= (SELF.value - var_1.value)
}
```

# MoveScanner

MoveScanner is a bytecode based move static analysis tool written in rust.

## QuickStart

**Step 1.** Run `build.sh`:

```shell
cd MoveScanner-v1
./build.sh
```

**Step 2.** Configure Shell (Option)

`build.sh` will automatically configure MoveScanner for your default shell.

If you wish to use MoveScanner on another shell, you should add the following to shell configuration file.

```shell
export MOVESCANNER_ROOT="$HOME/.MoveScanner"
export PATH="$MOVESCANNER_ROOT/bin:$PATH"
```

Otherwise, you can skip `Step 2`.

**Step 3.** If you want to update MoveScanner:

```
git pull
./build.sh
```

**Step 4.** Start a new terminal session, enjoy!

```shell
cd res/demo
MoveScanner -p .
```

## Usage

```
$ MoveScanner -h
A static analysis tool based on bytecode for move smart contracts.

Usage: MoveScanner [OPTIONS] --path <PATH> [COMMAND]

Commands:
  printer
  detector
  help      Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>        Path to input dir/file
  -o, --output <OUTPUT>    Path to output file [default: result.json]
  -n, --none               Print nothing on terminal
  -i, --ir-type <IR_TYPE>  IR type [possible values: sb, cm, cfg, du, fs, cg]
  -h, --help               Print help
  -V, --version            Print version
```

### Detector

**detector is default executor**, so you can omit it when using it.

You can input a complete project, MoveScanner will automatically identify the project type (Aptos/Sui), then compile the project and detect vulnerabilities.

```shell
# Tips: Make sure your project can be compiled successfully.
MoveScanner -p <package_path>
```
Additionally, you can directly provide bytecode files or a folder containing bytecode files.

```shell
MoveScanner -p <bytecode_file_path>
MoveScanner -p <bytecode_dir_path>
```

The result is output to `result.json` by default, you can customize the output file name and path by running `-o`：

```shell
MoveScanner -p <path> -o <output_path>
```

If you don't want to output results on the command line, use `-n`.

```shell
MoveScanner -p <path> -n
```

### Printer

The printer can output some intermediate representations:

- `sb`: Stackless Bytecode
- `cm`: Compile Module
- `cfg`: Control Flow Graph
- `du`: Tempindex def and use
- `fs`: Function Signatures
- `cg`: Function Call Graph

```shell
# example
MoveScanner -p <path> -i sb printer
```

## Detector Define

| id  | Detector                  | Define                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| --- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | InfiniteLoop              | Infinite loop refers to a situation in a contract where a loop construct exists, but the loop body fails to satisfy the termination condition, resulting in an infinite repetition of the loop.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| 2   | Overflow                  | The term "overflow" used here specifically refers to the situation where an overflow occurs due to the SHL instruction.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| 3   | PrecisionLoss             | Precision loss refers to a situation in which a significant discrepancy arises between the calculated result and the actual result due to the occurrence of operations involving multiplication, division, and square root in an order where division or square root is performed before multiplication.In the case of consecutive multiplication and division operations, the result obtained by performing multiplication first and then division is closer to the true result compared to performing division first and then multiplication. This defect also occurs in consecutive square root multiplication operations, where performing the square root operation before the multiplication operation results in greater precision loss. |
| 4   | UncheckedReturn           | Unchecked return refers to the situation that when a function call occurs in the move contract, the called function has return values, but the caller does not receive the return value or only receives part of the return value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| 5   | UnnecessaryBoolJudgment   | Unnecessary bool judgment defect is another form of meaningless code construction. It refers to the situation where a boolean variable is compared for equality or inequality with a boolean constant in conditional statements, which is equivalent to directly using the boolean variable as the conditional expression.                                                                                                                                                                                                                                                                                                                                                                                                                      |
| 6   | UnnecessaryTypeConversion | Unnecessary type conversion refers to the situation where a typecast operation is performed on an integer variable, even though the original and target types are the same. This results in an unnecessary "cast" instruction in the bytecode, leading to gas wastage.                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 7   | UnusedConstant            | Unused constants refer to constants that are defined within a module but remain unused in the module's code. It is important to note that the module mentioned here does not include test code because the bytecode generated by test code is not included in the bytecode deployed on the blockchain. The main consequence of the Unused Constants defect is the increase in gas costs during module deployment, leading to gas wastage.                                                                                                                                                                                                                                                                                                       |
| 8   | UnusedPrivateFunctions    | Similar to unused constants, unused private functions refer to the situation where private functions are declared and defined within a module but remain unused within the same module. This renders the declared and defined private functions inaccessible and essentially dead code. This not only leads to gas wastage similar to Unused Constants but also suggests the possibility of the programmer mistakenly setting the function's visibility.                                                                                                                                                                                                                                                                                        |
| 9   | RecursiveFunctionCall     | A "RecursiveFunctionCall" refers to the occurrence of both single-function recursion and cyclic calls between multiple functions within a single module. Due to the absence of cross-module circular dependencies in the Move language, any such cases would result in compile-time errors, and therefore, they are not subject to detection.                                                                                                                                                                                                                                                                                                                                                                                                   |
| 10  | RepeatedFunctionCall      | Within a function, the same function of the same module is called twice or more with the same parameters passed in                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

## Contribute

Contributions welcome! Click [here](https://movebit1.yuque.com/xcnnsm/cf_records/gdxeo9s6r5miv4pm) to make suggestions to make MoveScanner better!

# MoveScanner

工具编译

```bash
cargo build --release //编译后的位置在./target/release/MoveScanner
caego build // 仅测试的话可以用这个,编译后的位置在./target/debug/MoveScanner
```

工具运行

```bash
MoveScanner --filedir <FILEDIR> detection [DETECTION]
<FILEDIR> 可以是单个字节码文件路径，也可以是字节码文件目录
detection subcomand 是对输入文件进行defect check，输出为检测结果
[DETECTION]
possible values: 
unchecked-return, 
overflow, 
precision-loss, 
infinite-loop, 
unused-constant, 
unused-private-functions, 
unnecessary-type-conversion, 
unnecessary-bool-judgment,
None(缺省时会检测执行上述所有检测)

MoveScanner --filedir <FILEDIR> printer [PRINTER]
<FILEDIR> 可以是单个字节码文件路径，也可以是字节码文件目录
printer subcomand 是对输入文件进行分析结果的输出
[PRINTER]
possible values: 
ir, 
compile-module, 
cfg, 
def-use, 
function-visibility
```

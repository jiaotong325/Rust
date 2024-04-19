## 猜数字
https://rustwiki.org/zh-CN/book/ch02-00-guessing-game-tutorial.html
### use关键字
导入包使用use关键字,导入io包用以读取.
~~~rust
use std::io;
~~~

### mut关键字
rust默认所有变量不可变，加mut关键字为可变.
~~~rust
let a = 10;//不可变
let mut b =12;//可变
~~~

### io库
io::stdin().read_line(&mut guess)
要求guess可变，放入一个字符串型的变量中，
返回一个Result类型的变量，枚举(enum)类型{OK,ERR}
OK表示返回成功附带成功生成的值，Err表示失败原因并附带失败原因

expect()是io::Result的一个方法，Result是Err，expect()会使得程序崩溃，并显示expect()的参数；是OK，expect会获取Ok中的值原样返回.
(错误了就打印)

~~~rust
io::stdin()
        .read_line(&mut guess);
~~~
也是可以的运行的，不过会出现警告.

### println!宏


### crates包



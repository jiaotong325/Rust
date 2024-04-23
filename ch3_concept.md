## 3.1变量和可变性

变量默认不可变！

```rust
let x = 5;//不可变
let mut y =5;//可变

x = 6;//error!
y = 6;//ok
```

### 常量(*constant*)

不同于不可变的变量(即无mut修饰的变量).

使用const关键词修饰.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### 遮蔽

第一个变量被第二个变量**遮蔽**(*shadow*);

```rust
fn main() {
    let x = 5;
    let x = x + 1;
	{
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
```

```rust
The value of x in the inner scope is: 12
The value of x is: 6
```

注意这里重新赋值时使用了 `let`关键字，这就完成了遮蔽的效果；

若仅为：x = x + 1;则是重新赋值.

**遮蔽其实是重新创建了个新的变量，可以改变值的类型**

```rust
let spaces = "   ";
let spaces = spaces.len();
```

## 3.2数据类型

rust是一种静态类型的语言，在编译期间就需要知道变量类型.

但大多数情况下编译器可以**自行推断**出变量类型；但类型有多种情况时就必须指定变量类型;

```rust
let x = 5;
//必须指定一个变量类型，否则有可能为u32或f32等.
let guess: u32 = "42".parse().expect("Not a number!");
```

### 标量类型

整型、浮点型、布尔型和字符.

#### 整数类型

| 长度  | 有符号类型 | 无符号类型 |
| ----- | ---------- | ---------- |
| 8位   | i8         | u8         |
| 16位  | i16        | u16        |
| 32位  | i32        | u32        |
| 64位  | i64        | u64        |
| 128位 | i128       | u128       |
| arch  | isize      | usize      |

isize与usize取决于计算机，32位架构为32位；64位架构为64位.

**默认会使用i32.**

#### 浮点类型

f32和f64，默认为f64.

#### 布尔类型

true or false.

#### 字符类型

char 4字节 Unicode.

### 复合类型

元组(tuple)和数组(array)

#### 元组类型

可将**多个类型**组合在一起，元组长度**固定**.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

获取元组值：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;//解构

let x1 = tup.0;
let x2 = tup.1;
let x3 = tup.2;
```

#### 数组类型

类型相同，且数组长度固定.

```rust
let a = [1, 2, 3, 4, 5];

//定义i32类型的数组，其中有5个元素
let a: [i32; 5] = [1, 2, 3, 4, 5];

//指定初始值，5个3
let a = [3; 5];//[3,3,3,3,3]
```

访问数组:

a[0], a[1]...

## 3.3函数

### 语句和表达式

语句：没有返回值

表达式：有返回值

```rust
let x = 3;//语句
y = 6;在其他语言中会返回6，但在rust中无返回
y+1 //表达式
6 //表达式
```

```rust
let y = {
        let x = 3;//语句
        x + 1 //表达式
};//整体是语句
```

### 带有返回值的函数

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}//表达式 直接可以当返回值使用；否则要使用return关键字
```

## 3.4注释

## 3.5控制流

### if表达式

条件必须是bool类型的值否则会抛出错误.

以下代码在其他语言中或许是正确的，但在rust中会抛出错误

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

正确：

```rust
if number != 0 {
	println!("number was something other than zero");
}
```

#### 在let语句中使用if

```rust
let condition: bool = true;
let number = if condition { 5 } else { 6 };
```

### 循环(loop while for)

#### 使用loop循环

若出现嵌套循环.引入了循环标签(loop lable)，break或continue能够停止指定的标签的循环.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

```
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

#### for循环

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
//3!
//2!
//1!
//LIFTOFF!!!
//(1..4)表示1-4的左闭右开，rev()函数反转区间-> 4到1的左开右闭
```

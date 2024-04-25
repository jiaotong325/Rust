## 什么是所有权

管理内存的一种方式，编译时根据规则进行检查.

目的：

* 跟踪代码在哪些部分在使用堆上的数据
* 最小化堆上的重复数据量
* 清理堆上从未使用过的数据以避免空间不足

### 所有权规则

* rust中每一个值都有一个被称为其**所有者**的变量.
* 值在任意时刻有且只有一个所有者
* 当所有者(变量)离开作用域，这个值将被丢弃.

### 变量作用域

```rust
{                      // s 在这里无效, 它尚未声明
let s = "hello";   // 从此处起，s 开始有效

// 使用 s
} // 此作用域已结束，s 不再有效
```

### String类型(堆上)

```rust
{
    let s = String::from("hello"); // 从此处起，s 开始有效
    // 使用 s
}                                  // 此作用域已结束，
// s 不再有效
```

创建s后，当是离开作用域，rust调用一个特殊的函数，drop().

### 变量与数据交互的方式(一)：移动

```rust
let x = 5;
let y = x;
//将5赋值给x，再生成一个x的拷贝赋值给y，这样就有了两个5都被放在栈上
```

```rust
let s1 = String::from("hello");
let s2 = s1;
//s1和s2其实都是指针，都指向同一个内存区域.
```

观察以上两种，前者被创建在栈上，后者在堆上.

考虑后者，若过了作用域，需要释放内存，**那么“hello”这片内存会被释放两次！**为了确保安全，在 `let s2 = s1` 之后，认为**`s1`不再有效**，s1离开作用域之后也不会在进行释放内存.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);//error!
```

可以理解为s1被移动到了s2中.

s1无效，s2有效，当s2离开作用域，释放内存.

### 变量与数据交互的方式(二)：克隆

如果确实需要深度复制String堆上的数据，需使用clone()，

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### 所有权与函数

将值传递给函数在语义上与给变量赋值相似，向函数传递值可能会**移动或者复制**。

```rust
fn main() {
  let s = String::from("hello");  // s 进入作用域

  takes_ownership(s);             // s 的值移动到函数里 ...
                                  // ... 所以到这里不再有效

  let x = 5;                      // x 进入作用域

  makes_copy(x);                  // x 应该移动函数里，
                                  // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
  println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

个人理解：s赋值给了takes_ownership()函数的形参，发生移动.

### 返回值与作用域

返回值也可以转移所有权

```rust
fn main() {
  let s1 = gives_ownership();         // gives_ownership 将返回值
                                      // 移给 s1
  let s2 = String::from("hello");     // s2 进入作用域
  let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                      // takes_and_gives_back 中,
                                      // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
                                           // 调用它的函数
  let some_string = String::from("yours"); // some_string 进入作用域
  some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
  a_string  // 返回 a_string 并移出给调用的函数
}
```

总结：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 `drop` 被清理掉，除非数据被移动为另一个变量所有。

## 引用与借用

如果在一个函数中，想将String作为参数传递给函数，但却不想获取其所有权，这时就需要用到引用。

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

`&`引用符号允许使用值而不获得所有权，此后就能够再次使用s1了.

![引用](https://rustwiki.org/zh-CN/book/img/trpl04-05.svg)

称之为**借用**，用完之后还得还回去.



### 修改借用的值

​	使用可变引用

```rust
fn main() {
    let mut s = String::from("hello");//mut
    change(&mut s);//mut
}

fn change(some_string: &mut String) {//mut
    some_string.push_str(", world");
}
```

为了防止**数据竞争**，同一时间**只能有一个**针对于特定变量的**可变引用**：

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;//错误

println!("{}, {}", r1, r2);
```

**读写锁**

```
两个或更多指针同时访问同一数据。
至少有一个指针被用来写入数据。
没有同步数据访问的机制。
三者同时发生了！！！
```

正确：并未在同一时间出现

```rust
let mut s = String::from("hello");
{
let r1 = &mut s;
} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
```

错误：看起来没什么大问题，但使用者并不想看到**不可变引用发生了变化！**

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);
```

但是，由于r1和r2(不可变引用)在r3出现前就没有在使用过，**作用域已过**，故以下代码可以编译并允许.

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{} and {}", r1, r2);
// 此位置之后 r1 和 r2 不再使用

let r3 = &mut s; // 没问题
println!("{}", r3);
```

### 垂悬引用

垂悬指针：释放内存时保留指向它的指针而错误的生成一个垂悬指针，其指向的内存可能已经被分配给其他持有者.

rust编译是报错：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
```

在dangle()函数结束后，s被释放留下&s.

如何修改？

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

## 切片slice

slince类型并没有所有权，slice即引用集合中一段连续的元素序列.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```




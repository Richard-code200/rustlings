fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: 用一条语句解构元组 `cat`，使下面的 println 能正常运行。
    // let /* 在这里填写模式 */ = cat;
    let (name, age) = cat;

    println!("{name} 今年 {age} 岁");
}

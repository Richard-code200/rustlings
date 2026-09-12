// 结构体持有引用时，也需要使用生命周期。

// TODO: 修复与此结构体有关的编译错误。
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("《{}》，作者：{}", book.title, book.author);
}

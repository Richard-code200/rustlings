// TODO: 修复调用私有函数时出现的编译错误。
mod sausage_factory {
    // 不要让此模块外部的代码访问它！
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("香肠！");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

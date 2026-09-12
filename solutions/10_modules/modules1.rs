mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 在 `fn` 前添加 `pub`，使模块外部也能访问此函数。
    pub fn make_sausage() {
        get_secret_recipe();
        println!("香肠！");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

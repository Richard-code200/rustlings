struct ColorRegularStruct {
    // TODO: 添加测试 `regular_structs` 所需的字段。
    // 这些字段应该使用什么类型？RGB 颜色分量的最小值和最大值分别是多少？
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(
    /* TODO: 添加测试 `tuple_structs` 所需的字段 */
    u8,
    u8,
    u8,
);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 实例化一个普通结构体。
        // let green =

        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构体。
        // let green =

        let green = ColorTupleStruct(0, 255, 0);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个单元结构体。
        // let unit_struct =
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}

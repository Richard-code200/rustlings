// 本测验考查：
// - 泛型
// - trait
//
// 一所虚构的魔法学校有了一套用 Rust 编写的新成绩单生成系统！
// 目前，系统只支持创建以数字表示成绩的成绩单
// （例如 1.0 到 5.5）。然而，
// 学校也会使用字母成绩（A+ 到 F-），
// 因此系统需要能够打印这两种成绩单！
//
// 请对 `ReportCard` 结构体及其 impl 块做必要的修改，
// 使其在支持数字成绩单的同时，也支持字母成绩单。

// TODO: 按照上面的说明调整结构体。
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: 按照上面的说明调整 impl 块。
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}

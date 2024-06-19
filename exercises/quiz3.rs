// quiz3.rs
//
// 本測驗涵蓋:
// - 泛型 (Generics)
// - 特徵 (Traits)
//
// 一所虛構的魔法學校有一個用 Rust 編寫的新報告卡生成系統！目前該系統僅支持創建成績以數字表示的報告卡（例如 1.0 -> 5.5）。
// 然而，學校也發佈了字母成績（A+ -> F-），需要能夠打印這兩種類型的報告卡！
//
// 在 ReportCard 結構和 impl 塊中做出必要的代碼更改，以支持字母報告卡。
// 在第二個測試中將 Grade 更改為 "A+"，以表明您的更改允許字母成績。
//
// 執行 `rustlings hint quiz3` 或使用 `hint` watch 子命令獲取提示。

// I AM NOT DONE

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
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
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: 完成練習後，請確保在這裡更改成績。
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}

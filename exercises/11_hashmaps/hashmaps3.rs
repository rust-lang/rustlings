// hashmaps3.rs
//
// 給定了一場足球比賽的比分列表（每行一個）。每行的格式為 : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// 例如：England,France,4,2（英格蘭隊打進4球，法國隊打進2球）。
//
// 您需要構建一個包含球隊名稱、球隊總進球數和球隊總失球數的比分表。一種構建比分表的方法是使用雜湊表。
// 解決方案部分使用了雜湊表，完成它以通過測試。
//
// 讓我通過測試！
//
// 執行 `rustlings hint hashmaps3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::collections::HashMap;

// 一個存儲球隊進球詳細信息的結構體。
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // 球隊的名稱是鍵，其關聯的結構體是值。
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: 使用從當前行提取的詳細信息填充比分表。請記住，team_1 進的球數將是 team_2 的失球數，反之亦然。
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}

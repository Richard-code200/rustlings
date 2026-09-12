// 给定一份足球比赛比分列表，每行一场比赛，格式为：
// "<球队1名称>,<球队2名称>,<球队1进球数>,<球队2进球数>"
// 例如："England,France,4,2"（英格兰进 4 球，法国进 2 球）。
//
// 请构建一张统计表，记录每支球队的名称、
// 总进球数
// 和总失球数。

use std::collections::HashMap;

// 用于存储一支球队进球和失球详情的结构体。
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // 键为球队名称，值为对应的结构体。
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 注意：我们还没有学习错误处理，所以这里使用 `unwrap`。
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: 将提取出的详细信息填入统计表。
        // 注意：球队 1 的进球数就是球队 2 的失球数。
        // 同样，球队 2 的进球数
        // 就是球队 1 的失球数。
    }

    scores
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(
            ["England", "France", "Germany", "Italy", "Poland", "Spain"]
                .into_iter()
                .all(|team_name| scores.contains_key(team_name))
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}

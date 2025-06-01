use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::io;

fn main() {
    let member_data: String = readfile().expect("failed to read file");
    let member_list: Vec<&str> = member_data.split(',').map(|name| name.trim()).collect();

    //基本的な数値の取得（チームの数、チーム員数、タスク数）
    let num_of_team = info_init_team();
    let num_of_task_runs = info_init_task();
    let member_per_team = gen_team(member_list.len() as u32, num_of_team);
    let mut table_of_member: Vec<Vec<u32>> = vec![vec![0u32; member_list.len()]; member_list.len()];

    println!("計算条件：");
    println!("チーム数は {} です", num_of_team);
    println!("課題数は {} です", num_of_task_runs);
    println!("全体の人員数は {} です", member_list.len());

    cal_alg(
        num_of_team,
        &member_per_team,
        num_of_task_runs,
        &member_list,
        &mut table_of_member,
    );
}

fn readfile() -> io::Result<String> {
    let file_path = "./sample.txt";
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

fn info_init_team() -> u32 {
    //チーム数の入力を受ける
    //正しいチーム数が入力されるまで、無限ループ

    loop {
        println!("チーム数を教えてください。");

        // input 定義、その後、IOで入力
        let mut input_team_num = String::new();

        io::stdin()
            .read_line(&mut input_team_num)
            .expect("failed to read number");

        // Stringをu32に変換（空白も削除）、1以上であれば返す。それ以外は無限ループに入る。
        match input_team_num.trim().parse::<u32>() {
            Ok(num) => {
                if num > 0 {
                    return num;
                } else {
                    println!("チーム数をマイナスに設定することはできません")
                }
            }
            Err(_) => {
                println!("有効な数字ではありません");
            }
        }
    }
}

fn info_init_task() -> u32 {
    //タスク数の入力を受ける
    //正しい課題数が入力されるまで、無限ループ

    loop {
        println!("課題の回数を教えてください。");

        // input 定義、その後、IOで入力
        let mut input_task_num = String::new();

        io::stdin()
            .read_line(&mut input_task_num)
            .expect("failed to read number");

        // Stringをu32に変換（空白も削除）、1以上であれば返す。それ以外は無限ループに入る。
        match input_task_num.trim().parse::<u32>() {
            Ok(num) => {
                if num > 0 {
                    return num;
                } else {
                    println!("課題数をマイナスに設定することはできません")
                }
            }
            Err(_) => {
                println!("有効な数字ではありません");
            }
        }
    }
}

fn gen_team(list: u32, num: u32) -> Vec<u32> {
    //チームの人員を決定し、ベクトルで返す
    let base_num: u32 = list / num; // 基本人員
    let remain_num: u32 = list % num; // 残り
    let mut output: Vec<u32> = Vec::with_capacity(num as usize); // アウトプット生成
    for i in 0..num {
        let mut number_of_this_team: u32 = base_num;
        if i < remain_num {
            number_of_this_team += 1;
        }
        output.push(number_of_this_team);
    }
    output
}

fn remove_value(vec: &mut Vec<u32>, value_to_remove: &u32) {
    vec.retain(|element| element != value_to_remove);
}

fn cal_alg(
    num_of_team: u32,
    member_per_team: &Vec<u32>,
    num_of_task_runs: u32,
    member_list: &Vec<&str>,
    table_of_member: &mut Vec<Vec<u32>>,
) {
    let num_iterations_per_round = 10000;

    for i in 0..num_of_task_runs {
        println!("\n--- ラウンド {} シミュレーション ---", i + 1);

        let mut best_round_teams: Option<Vec<Vec<u32>>> = None;
        let mut best_round_fitness = (0, f64::MAX); // 低いほど良いと判断

        // ラウンドの基準 table_of_member スナップショット
        let table_of_member_snapshot_for_round = table_of_member.clone();

        for _iteration in 0..num_iterations_per_round {
            // 1. available_members 生成
            let mut available_members: Vec<u32> = (0..member_list.len() as u32).collect();
            let mut rng = thread_rng();
            available_members.shuffle(&mut rng);

            // 2. 臨時テーブル生成
            let mut current_iteration_round_teams: Vec<Vec<u32>> =
                Vec::with_capacity(num_of_team as usize);
            for k in 0..num_of_team as usize {
                current_iteration_round_teams.push(Vec::with_capacity(member_per_team[k] as usize));
            }

            // 臨時 available_members
            let mut temp_available_members = available_members.clone();

            // 3. リーダー選択
            let leader_of_team: Vec<u32> =
                select_min_member(&table_of_member_snapshot_for_round, num_of_team);

            let mut possible_to_assign_leaders = true;
            for l_idx in 0..num_of_team as usize {
                // リーターを temp_available_membersにいるか確認後、削除
                if let Some(pos) = temp_available_members
                    .iter()
                    .position(|&x| x == leader_of_team[l_idx])
                {
                    let leader_id = temp_available_members.remove(pos);
                    current_iteration_round_teams[l_idx].push(leader_id);
                } else {
                    possible_to_assign_leaders = false;
                    break;
                }
            }
            if !possible_to_assign_leaders {
                continue;
            }

            // 4. select_best_member
            for j in 0..num_of_team as usize {
                // select_best_memberは table_of_member_snapshot_for_round スナップショットで評価
                select_best_member(
                    &mut current_iteration_round_teams[j],
                    &mut temp_available_members,
                    j as u32,
                    &member_per_team,
                    &table_of_member_snapshot_for_round,
                );
            }

            // 5. 評価パート
            let current_fitness = calculate_round_fitness(
                &current_iteration_round_teams,
                &table_of_member_snapshot_for_round,
            );

            if current_fitness.0 < best_round_fitness.0
                || (current_fitness.0 == best_round_fitness.0
                    && current_fitness.1 < best_round_fitness.1)
            {
                best_round_fitness = current_fitness;
                best_round_teams = Some(current_iteration_round_teams);
            }
        }

        // 6. 最善の結果を table_of_memberに反映
        if let Some(final_round_teams) = best_round_teams {
            println!(
                "\n--- ラウンド {} 結果 (新しい組: {}, 標準偏差: {:.4}) ---",
                i + 1,
                -best_round_fitness.0,
                best_round_fitness.1
            );
            for (team_idx, team) in final_round_teams.iter().enumerate() {
                let team_names: Vec<String> = team
                    .iter()
                    .map(|&member_id| member_list[member_id as usize].to_string())
                    .collect();
                println!("チーム {}: {:?}", team_idx + 1, team_names);

                //  table_of_member アップデート
                for member1_pos in 0..team.len() {
                    for member2_pos in (member1_pos + 1)..team.len() {
                        let id1 = team[member1_pos] as usize;
                        let id2 = team[member2_pos] as usize;
                        if id1 < table_of_member.len() && id2 < table_of_member[id1].len() {
                            table_of_member[id1][id2] += 1;
                            table_of_member[id2][id1] += 1;
                        }
                    }
                }
            }
        } else {
            println!("\n--- ラウンド {} でエラー発生 ---", i + 1);
        }

        println!("\n ラウンド {} 後 全体の状況", i + 1);
        for zentai in 0..table_of_member.len() {
            println!("{:?}", table_of_member[zentai]);
        }
    } // End of task runs (rounds)

    println!("最終結果");
    for zentai in 0..table_of_member.len() {
        println!("{:?}", table_of_member[zentai]);
    }
}

//チームの数分、会った回数が少ない人を選び、その結果を返す（ベクトルで）
fn select_min_member(table_of_member: &Vec<Vec<u32>>, num_of_team: u32) -> Vec<u32> {
    if table_of_member.len() % num_of_team as usize != 0 {
        //総会った回数のベクトルを生成
        let sum_of_meet: Vec<u32> = table_of_member
            .iter()
            .map(|inner_vec| inner_vec.iter().sum())
            .collect();
        //ベクトルを整列し、会った回数が少ない人のインデックスを返す。
        let mut indexed_sums: Vec<(usize, u32)> = sum_of_meet
            .iter()
            .enumerate()
            .map(|(index, &sum_val)| (index, sum_val))
            .collect();
        //整列
        indexed_sums.sort_by_key(|&(_index, sum_val)| sum_val);

        let leader_selected: Vec<u32> = indexed_sums
            .iter()
            .take(num_of_team as usize)
            .map(|&(index, _sum_val)| index as u32)
            .collect();

        leader_selected
    } else {
        let mut leader_selected: Vec<u32> = (0..table_of_member.len() as u32).collect();
        let mut rng = thread_rng();
        leader_selected.shuffle(&mut rng);

        leader_selected
    }


}

fn select_best_member(
    status_of_team: &mut Vec<u32>,
    available_members: &mut Vec<u32>,
    team_number: u32,
    member_per_team: &Vec<u32>,
    table_of_member: &Vec<Vec<u32>>,
) {
    loop {
        // メンバーがない時は終了
        if status_of_team.len() >= (member_per_team[team_number as usize] as usize) {
            break;
        }
        if available_members.is_empty() {
            break;
        }

        let mut score_list: Vec<(u32, u32, u32)> = Vec::new(); // (優先順位,会った回数の合計, メンバーID)
        for &candidate_member_id in available_members.iter() {
            let mut sum_of_meetings_with_current_team: u32 = 0;
            let mut has_met_anyone_in_current_team = false;

            for &member_in_current_team_id in status_of_team.iter() {
                // 有効性チェック
                if (candidate_member_id as usize) < table_of_member.len()
                    && (member_in_current_team_id as usize)
                        < table_of_member[candidate_member_id as usize].len()
                {
                    let meetings = table_of_member[candidate_member_id as usize]
                        [member_in_current_team_id as usize];
                    if meetings > 0 {
                        has_met_anyone_in_current_team = true;
                    }
                    sum_of_meetings_with_current_team += meetings;
                }
            }

            //優先順位付与
            let priority_score = if has_met_anyone_in_current_team { 1 } else { 0 };

            score_list.push((
                priority_score,
                sum_of_meetings_with_current_team.pow(2),
                candidate_member_id,
            ));
        }

        if score_list.is_empty() {
            break;
        }

        // min_by_keyで整列
        let best_candidate_tuple_option = score_list
            .iter()
            .min_by_key(|&(priority, squared_sum, _member_id)| (priority, squared_sum));

        if let Some(&best_candidate_tuple) = best_candidate_tuple_option {
            let chosen_member_id = best_candidate_tuple.2;
            status_of_team.push(chosen_member_id);
            remove_value(available_members, &chosen_member_id);
        } else {
            break;
        }
    }
}

fn calculate_round_fitness(
    round_teams: &Vec<Vec<u32>>,
    table_of_member_snapshot: &Vec<Vec<u32>>,
) -> (i32, f64) {
    let mut temp_table = table_of_member_snapshot.clone();
    let mut new_encounters_count = 0;

    for team in round_teams.iter() {
        for m1_idx in 0..team.len() {
            for m2_idx in (m1_idx + 1)..team.len() {
                let id1 = team[m1_idx] as usize;
                let id2 = team[m2_idx] as usize;

                if id1 < temp_table.len() && id2 < temp_table[id1].len() {
                    if table_of_member_snapshot[id1][id2] == 0 {
                        new_encounters_count += 1;
                    }
                    temp_table[id1][id2] += 1;
                    temp_table[id2][id1] += 1;
                }
            }
        }
    }

    let mut all_meeting_counts: Vec<u32> = Vec::new();
    for r in 0..temp_table.len() {
        for c in (r + 1)..temp_table[r].len() {
            all_meeting_counts.push(temp_table[r][c]);
        }
    }

    if all_meeting_counts.is_empty() {
        return (0, f64::MAX);
    }

    let sum: u32 = all_meeting_counts.iter().sum();
    let mean = sum as f64 / all_meeting_counts.len() as f64;
    let variance = all_meeting_counts
        .iter()
        .map(|value| {
            let diff = mean - (*value as f64);
            diff * diff
        })
        .sum::<f64>()
        / all_meeting_counts.len() as f64;

    (-new_encounters_count, variance.sqrt()) // 新しい組と標準偏差を返す
}

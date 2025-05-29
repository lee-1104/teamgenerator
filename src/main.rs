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
    //関数が呼ばれた時に、全体の会った回数を初期化する

    //ここからラウンドごとに実行し、チームを決める
    for i in 0..num_of_task_runs {
        //ラウンド設定。
        let mut available_members: Vec<u32> = (0..member_list.len() as u32).collect();

        //ラウンド用のチーム枠生成
        let mut table_of_team_round: Vec<Vec<u32>> = Vec::with_capacity(num_of_team as usize);
        for k in 0..num_of_team as usize {
            let inner_team_vec: Vec<u32> = Vec::with_capacity(member_per_team[k] as usize);
            table_of_team_round.push(inner_team_vec);
        }

        //ここでチーム長を決める。select_min_member関数を呼び出しする
        let leader_of_team: Vec<u32> = select_min_member(&table_of_member, num_of_team);

        //各チームにリーダーを割り当てる
        for l in 0..num_of_team as usize {
            remove_value(&mut available_members, &leader_of_team[l]);
            table_of_team_round[l].push(leader_of_team[l]);
        }

        for j in 0..num_of_team as usize {
            //チーム単位で実行
            select_best_member(
                &mut table_of_team_round[j],
                &mut available_members,
                j as u32,
                &member_per_team,
                &table_of_member,
            );
        }
        println!("\n--- ラウンド {} 結果 ---", i + 1);
        for (team_idx, team) in table_of_team_round.iter().enumerate() {
            let team_names: Vec<String> = team
                .iter()
                .map(|&member_id| member_list[member_id as usize].to_string())
                .collect();
            println!("チーム {}: {:?}", team_idx + 1, team_names);

            // table_of_member 업데이트
            for member1_pos in 0..team.len() {
                for member2_pos in (member1_pos + 1)..team.len() {
                    let id1 = team[member1_pos] as usize;
                    let id2 = team[member2_pos] as usize;
                    if id1 < table_of_member.len() && id2 < table_of_member[id1].len() {
                        // インデックス確認
                        table_of_member[id1][id2] += 1;
                        table_of_member[id2][id1] += 1; // 会った回数反映
                    }
                }
            }
        }

        //ラウンド結果出力
        println!("\n ラウンド {} 全体状況", i + 1);
        for zentai in 0..table_of_member.len() {
            println!("{:?}", table_of_member[zentai]);
        }
    }

    println!("最終結果");
    //全体の結果出力
    for zentai in 0..table_of_member.len() {
        println!("{:?}", table_of_member[zentai]);
    }
}

//チームの数分、会った回数が少ない人を選び、その結果を返す（ベクトルで）
fn select_min_member(table_of_member: &Vec<Vec<u32>>, num_of_team: u32) -> Vec<u32> {
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

        let mut score_list: Vec<(u32, u32)> = Vec::new(); // (会った回数の合計, メンバーID)

        for &candidate_member_id in available_members.iter() {
            let mut current_sum_of_meetings: u32 = 0;
            for &member_in_current_team_id in status_of_team.iter() {
                // table_of_member 有効性評価
                if (candidate_member_id as usize) < table_of_member.len()
                    && (member_in_current_team_id as usize) < table_of_member.len()
                    && (member_in_current_team_id as usize)
                        < table_of_member[candidate_member_id as usize].len()
                    && (candidate_member_id as usize)
                        < table_of_member[member_in_current_team_id as usize].len()
                {
                    current_sum_of_meetings += table_of_member[candidate_member_id as usize]
                        [member_in_current_team_id as usize];
                }
            }
            score_list.push((current_sum_of_meetings.pow(2), candidate_member_id));
        }

        if score_list.is_empty() {
            break;
        }

        let best_candidate_tuple_option = score_list
            .iter()
            .min_by_key(|(current_score, _member_id)| current_score);

        if let Some(&best_candidate_tuple) = best_candidate_tuple_option {
            let chosen_member_id = best_candidate_tuple.1;
            status_of_team.push(chosen_member_id);
            remove_value(available_members, &chosen_member_id);
        } else {
            break;
        }
    }
}

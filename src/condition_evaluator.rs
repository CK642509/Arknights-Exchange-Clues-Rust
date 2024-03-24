pub struct ConditionEvaluator {
    all_wants: Vec<Vec<i32>>,
    n_players: usize,
    n_clues: usize,
    num_conf: usize,
    condition_count: i32,
    best_condition: Option<Vec<i32>>,
    best_chg: Option<i32>,
    best_rank: Vec<i32>,
    n_valid: usize,
    valid_spot: Vec<(usize, usize)>,
    temp_wants: Vec<Vec<i32>>,
    conf: Vec<Vec<usize>>,
}

impl ConditionEvaluator {
    pub fn new(all_wants: Vec<Vec<i32>>) -> Self {
        let n_players = all_wants.len();
        let n_clues = all_wants[0].len();
        let num_conf = n_players * (n_players - 1);
        let condition_count = 0;
        let best_condition = None;
        let best_chg = None;
        let best_rank = vec![n_players as i32, 0, 0, 0, 0];

        // count valid spots (want == 0)
        let mut n_valid = 0;
        let mut valid_spot = Vec::new();
        for i in 0..n_players {
            for j in 0..n_clues {
                if all_wants[i][j] == 0 {
                    n_valid += 1;
                    valid_spot.push((i, j));
                }
            }
        }

        // copy wants to temp_wants
        let temp_wants = all_wants.clone();

        let mut conf = vec![vec![0, 0, 0]; num_conf];
        let mut count = 0;
        for i in 0..n_players {
            for j in 0..n_players {
                if i != j {
                    conf[count][1] = i;
                    conf[count][2] = j;
                    count += 1;
                }
            }
        }

        Self {
            all_wants,
            n_players,
            n_clues,
            num_conf,
            condition_count,
            best_condition,
            best_chg,
            best_rank,
            n_valid,
            valid_spot,
            temp_wants,
            conf,
        }
    }

    pub fn generate_conditions(&mut self, index: usize) -> Vec<Vec<i32>> {
        let x = 2usize.pow(self.n_valid as u32);
        if index == self.n_valid {
            self.condition_count += 1;
            println!("{}/{}", self.condition_count, x);
            self.temp_wants.clone()
        } else {
            let (i, j) = self.valid_spot[index];
            self.temp_wants[i][j] = 1;
            let mut conditions = self.generate_conditions(index + 1);

            self.temp_wants[i][j] = -1;
            conditions.extend(self.generate_conditions(index + 1));

            conditions
        }
    }

    pub fn evaluate(&mut self) {
        for condition in self.generate_conditions(0) {
            let tmp_rank = self.try_condition(&condition, &self.conf);
            if let Some(tmp_rank) = tmp_rank {
                for (&tmp_score, &best_score) in tmp_rank.iter().zip(self.best_rank.iter()) {
                    if tmp_score < best_score {
                        self.best_rank = tmp_rank.clone();
                        self.best_condition = Some(condition.clone());
                        self.best_chg = Some(tmp_rank[5]);
                        break;
                    }
                }
            }
        }
    }

    // fn try_condition(&self, condition: &Vec<Vec<i32>>, conf: &Vec<Vec<i32>>) -> Option<Vec<i32>> {
    //     let mut tmp_how = vec![vec![0]; self.n_clues];
    //     let mut tmp_rank = vec![0; 5];
    //     let num_conf = self.n_players * (self.n_players - 1);
    //     let mut tmp_vote = vec![0; num_conf];
    //     let mut tmp_chg = Vec::new();
    //     let mut tmp_num_chg = 0;

    //     // ... same as before ...

    //     fn log(j: usize, clue_no: usize, p1: i32, p2: i32, tmp_vote: &mut Vec<i32>, tmp_chg: &mut Vec<Vec<i32>>, tmp_num_chg: &mut i32) {
    //         tmp_vote[j] += 1;
    //         *tmp_num_chg += 1;
    //         tmp_chg.push(vec![clue_no as i32, p1, p2]);
    //     }

    //     // ... same as before ...

    //     fn n_way_exchange(n: i32, tmp_how: &Vec<Vec<i32>>, tmp_vote: &mut Vec<i32>, tmp_chg: &mut Vec<Vec<i32>>, tmp_num_chg: i32, conf: &Vec<Vec<i32>>, num_conf: usize) -> (Vec<i32>, Vec<Vec<i32>>, i32) {
    //         // get_derangement is not defined in your Python code.
    //         // Here is a placeholder definition. You will need to replace it with the actual implementation.
    //         fn get_derangement(n: i32) -> Vec<Vec<i32>> {
    //             vec![vec![0; n as usize]]
    //         }

    //         // ... same as before ...

    //         (tmp_vote.clone(), tmp_chg.clone(), tmp_num_chg)
    //     }

    //     // ... same as before ...

    //     Some(tmp_rank)
    // }

    fn try_condition(&self, condition: &Vec<i32>, conf: &Vec<Vec<usize>>) -> Option<Vec<i32>> {
        // Replace this with the actual implementation.
        None
    }
}
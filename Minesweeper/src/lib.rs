pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let r = minefield.len();

    if r == 0 {
        return vec![];
    }

    let c = minefield[0].len();

    let mut res = vec![];

    if c == 0 {
        res.push("".to_string());
        return res;
    }

    for(i, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for(j, elem) in row.chars().enumerate() {
            let num_ad = count_ad(minefield, i, j);
            let tmp = if elem == ' ' && num_ad > 0 {
                num_ad.to_string().chars().nth(0).unwrap_or(' ')
            } else {
                elem
            };
            new_row.push(tmp);
        }
        res.push(new_row);
    }
    res
}

fn count_ad(minefield: &[&str], row: usize, col: usize) -> u32 {
    let r = minefield.len();
    let c = minefield[0].len();

    let mut cnt = 0;

    for i in (row.saturating_sub(1))..=(row + 1).min(r - 1) {
        for j in (col.saturating_sub(1))..=(col + 1).min(c - 1) {
            if i != row || j != col {
                if minefield[i].chars().nth(j).unwrap_or(' ') == '*' {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    /* unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield); */
    let mut result: Vec<String> = Vec::new();

    for y in 0..minefield.len() {
        let mut row = String::new();

        for x in 0..minefield[y].len() {
            row.push(annotate_box(x, y, minefield));
        }
        result.push(row)
    }
    result
}

fn annotate_box(x: usize, y: usize, board: &[&str]) -> char {
    let curr_box = board[y].chars().nth(x).unwrap();
    if curr_box == '*' {
        return curr_box;
    }

    let count = count_mine(x, y, board);
    if count > 0 {
        count.to_string().chars().nth(0).unwrap()
    } else {
        ' '
    }
}

fn count_mine(x_size: usize, y_size: usize, board: &[&str]) -> usize {
    let mut count = 0;
    let x = x_size as i8;
    let y = y_size as i8;

    for xs in x - 1..x + 2 {
        for ys in y - 1..y + 2 {
            count += mines_at(xs, ys, board);
        }
    }
    count
}

fn mines_at(x: i8, y: i8, board: &[&str]) -> usize {
    if x < 0 || y < 0 {
        return 0;
    } //out board

    let x_size = x as usize;
    let y_size = y as usize;
    if y_size >= board.len() {
        return 0;
    } //out board

    match board[y_size].chars().nth(x_size) {
        Some(ch) => {
            if ch == '*' {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}

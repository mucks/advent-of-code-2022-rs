use crate::util::get_input;

fn get_visible_tree_count(width: usize, height: usize, grid: Vec<Vec<u32>>) -> u32 {
    let mut visible_tree_count = 0;

    for y in 0..height {
        for x in 0..width {
            // Part one
            // trees are visible cause they're on the edge of the forest
            if y == 0 || x == 0 || x == width - 1 || y == height - 1 {
                visible_tree_count += 1;
                continue;
            }

            let mut should_add = vec![];
            let current_tree = grid[x][y];

            // all x behind tree on row
            should_add.push((0..x).all(|x| grid[x][y] < current_tree));
            // all x after tree on row
            should_add.push((x + 1..width).all(|x| grid[x][y] < current_tree));
            // all y behind tree on column
            should_add.push((0..y).all(|y| grid[x][y] < current_tree));
            // all y after tree on column
            should_add.push((y + 1..height).all(|y| grid[x][y] < current_tree));

            if should_add.iter().any(|s| *s) {
                //println!("x:{}, y:{}, val:{}", x, y, current_tree);
                visible_tree_count += 1;
            }
        }
    }
    visible_tree_count
}

pub fn run() {
    let input = get_input("day_eight");

    let mut height = 0;
    let mut width = 0;

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for (y, line) in input.split('\n').enumerate() {
        grid.insert(y, vec![]);
        let row = grid.get_mut(y).unwrap();
        for c in line.chars() {
            let x: u32 = c.to_string().parse().unwrap();
            row.push(x);
        }
        //IMPORTANT: assign width!
        width = row.len();
    }
    //IMPORTANT: assign height!
    height = grid.len();

    let mut scenic_scores = vec![];

    for y in 0..height {
        for x in 0..width {
            let current_tree = grid[x][y];
            let mut left_score = 0;
            let mut right_score = 0;
            let mut top_score = 0;
            let mut down_score = 0;

            for x_rev in (0..x).rev() {
                left_score += 1;
                if grid[x_rev][y] >= current_tree {
                    break;
                }
            }
            for xx in (x + 1)..width {
                right_score += 1;
                if grid[xx][y] >= current_tree {
                    break;
                }
            }
            for y_rev in (0..y).rev() {
                top_score += 1;
                if grid[x][y_rev] >= current_tree {
                    break;
                }
            }
            for yy in (y + 1)..height {
                down_score += 1;
                if grid[x][yy] >= current_tree {
                    break;
                }
            }
            let scenic_score = left_score * right_score * top_score * down_score;
            scenic_scores.push(scenic_score);
        }
    }

    println!(
        "{} trees are visible from the outside",
        get_visible_tree_count(width, height, grid)
    );

    println!(
        "Highest scenic score: {}",
        scenic_scores.iter().max().unwrap()
    )
}

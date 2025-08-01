use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Row {
    numbers: Vec<i32>,
    difference: i32,
}

impl Row {
    fn new(numbers: Vec<i32>) -> Self {
        let difference = (numbers[0] - numbers[1]).abs();
        Row { numbers, difference }
    }
}

fn sum_of_differences(rows: &[Row]) -> i32 {
    rows.iter().map(|row| row.difference).sum()
}

fn process_input(filename: &str) -> Result<Vec<Row>, Box<dyn std::error::Error>> {
    let content = read_to_string(filename)?;
    
    let rows: Vec<Row> = content
        .lines()
        .filter_map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();
            
            if numbers.len() == 2 {
                Some(Row::new(numbers))
            } else {
                None
            }
        })
        .collect();
    
    Ok(rows)
}

fn sort_columns(rows: &mut [Row]) {
    let num_columns = rows.first().map_or(0, |row| row.numbers.len());
    
    let mut columns: Vec<Vec<i32>> = vec![Vec::new(); num_columns];
    
    for row in rows.iter() {
        for (i, &num) in row.numbers.iter().enumerate() {
            columns[i].push(num);
        }
    }
    
    for column in &mut columns {
        column.sort_unstable();
    }
    
    let mut differences: Vec<i32> = vec![0; num_columns.min(columns[1].len())];
    
    // Calculate the absolute difference between each pair of adjacent numbers
    for (i, col) in columns.iter().enumerate() {
        if i < columns.len() - 1 && i < col.len() && i < columns[i + 1].len() {
            let prev_col = &columns[i + 1];
            differences[i] = col[col.len() - 1 - i].abs_diff(prev_col[i]);
        }
    }
    
    // Update the difference of each Row
    for row in rows.iter_mut() {
        row.difference = differences.iter().map(|&x| x).sum();
    }
}


fn print_results(rows: &[Row], title: &str) {
    println!("\n{}:", title);
    //for (i, row) in rows.iter().enumerate() {
    //    println!("Row {}: {:?} = {}", i + 1, row.numbers, row.sum);
    //}
    println!("Sum of all differences: {}", sum_of_differences(rows));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rows = process_input("input.txt")?;
    print_results(&rows, "Presort Distance");
    
    let mut part2_rows = rows.clone();
    sort_columns(&mut part2_rows);
    print_results(&part2_rows, "Post-sort distance");
    
    Ok(())
}

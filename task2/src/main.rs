use std::io;

fn convert(s: String, num_rows: usize) -> String {
    if num_rows == 1 || num_rows >= s.len() {
        return s;
    }

    let mut rows: Vec<String> = vec![String::new(); num_rows];
    let mut current_row = 0;
    let mut going_down = false;

    for c in s.chars() {
        rows[current_row].push(c);
        if current_row == 0 {
            going_down = true;
        } else if current_row == num_rows - 1 {
            going_down = false;
        }
        current_row = if going_down { current_row + 1 } else { current_row - 1 };
    }

    rows.concat()
}

fn main() {
    let mut input_string = String::new();
    let mut num_rows_input = String::new();

    // Ввод строки
    println!("Введите строку:");
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let input_string = input_string.trim().to_string();

    println!("Введите количество строк:");
    io::stdin().read_line(&mut num_rows_input).expect("Не удалось прочитать количество строк");
    
    let num_rows: usize = num_rows_input.trim().parse().expect("Введите корректное число");

    let result = convert(input_string, num_rows);
    println!("Результат: {}", result);
}

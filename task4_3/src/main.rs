use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();

    loop {
        println!("Введите команду add..to.., list.., list all или exit' для выхода:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if let Some((name, department)) = parse_command(input) {
            departments
                .entry(department.clone()) // Используем clone здесь
                .or_insert_with(Vec::new)
                .push(name.clone()); // Клонируем имя для хранения в векторе
            println!("Добавлено: {} в отдел {}", name, department);
        } else if input.eq_ignore_ascii_case("list all") {
            list_all_employees(&departments);
        } else if let Some(department) = input.strip_prefix("list ") {
            list_department_employees(&departments, department.trim()); // Передаем ссылку
        } else {
            println!("Неверная команда. Попробуйте снова.");
        }
    }
}

fn parse_command(command: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }
    if parts[0].eq_ignore_ascii_case("add") && parts[2].eq_ignore_ascii_case("to") {
        let name = parts[1].to_string();
        let department = parts[3..].join(" ");
        return Some((name, department));
    }
    None
}

fn list_all_employees(departments: &HashMap<String, Vec<String>>) {
    let mut sorted_departments: Vec<_> = departments.keys().collect();
    sorted_departments.sort();

    for department in sorted_departments {
        println!("Отдел {}:", department);
        let employees = &departments[department];
        for employee in employees {
            println!(" - {}", employee);
        }
    }
}

fn list_department_employees(departments: &HashMap<String, Vec<String>>, department: &str) { // Изменяем тип на &str
    match departments.get(department) {
        Some(employees) => {
            println!("Сотрудники отдела {}:", department);
            for employee in employees {
                println!(" - {}", employee);
            }
        }
        None => {
            println!("Отдел '{}' не найден.", department);
        }
    }
}

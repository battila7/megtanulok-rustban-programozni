use std::io;
use std::io::Write;
use std::collections::BTreeMap;

fn main() {
    let mut employees_by_department: BTreeMap<String, Vec<String>> = BTreeMap::new();

    println!("Available commands:\n  * Add <Name> to <Department>\n  * List [Department]\n  * Quit");

    loop {
        print!("> ");

        io::stdout().flush();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match &line {
            g if g.starts_with("Add") => add_employee(&mut employees_by_department, &line),
            g if g.starts_with("List") => list_employees(&employees_by_department, &line),
            g if g.starts_with("Quit") => break,
            _ => println!("Invalid command!"),
        }
    }
}

fn add_employee(employees_by_department: &mut BTreeMap<String, Vec<String>>, line: &str) {
    let segments: Vec<_> = line.split_ascii_whitespace().collect();

    if segments.len() != 4 {
        return
    }

    let employee = segments[1];
    let department = segments[3];

    let department_entry = employees_by_department.entry(department.to_owned()).or_insert(vec![]);
    department_entry.push(employee.to_owned());
}

fn list_employees(employees_by_department: &BTreeMap<String, Vec<String>>, line: &str) {
    let segments: Vec<_> = line.split_ascii_whitespace().collect();

    match segments.len() {
        2 => list_employees_of_department(employees_by_department, segments[1]),
        1 => list_employees_of_all_departments(employees_by_department),
        _ => (),
    }
}

fn list_employees_of_department(employees_by_department: &BTreeMap<String, Vec<String>>, department: &str) {
    println!("Employees of the {} department:", department);

    if let Some(employees) = employees_by_department.get(department) {
        let mut sorted_employees = employees.to_owned();
        sorted_employees.sort();

        println!("{:?}", sorted_employees);
    }
}

fn list_employees_of_all_departments(employees_by_department: &BTreeMap<String, Vec<String>>) {
    employees_by_department
        .keys()
        .for_each(|department| list_employees_of_department(employees_by_department, department));
}

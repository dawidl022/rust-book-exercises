use std::collections::HashMap;

pub enum Command {
    ShowAll,
    ShowInDepartment(String),
    AddEmployeeToDepartment(String, String),
}

impl Command {
    pub fn execute(&self, employees: &mut HashMap<String, Vec<String>>) {
        match self {
            Self::ShowAll => print_all(employees),
            Self::ShowInDepartment(department) => print_all_in_department(employees, department),
            Self::AddEmployeeToDepartment(employee_name, department) => {
                add_to_department(employees, employee_name.clone(), department)
            }
        }
    }

    pub fn success_message(self) -> String {
        match self {
            Self::AddEmployeeToDepartment(employee_name, department) => format!(
                "Successfully added {} to {} department!",
                employee_name, department
            ),
            _ => "".to_string(),
        }
    }
}

fn print_all(employees: &HashMap<String, Vec<String>>) {
    for (department, employees) in employees {
        print_in_department(department, employees);
    }

    if employees.is_empty() {
        println!("No employees found!")
    }
}

fn print_all_in_department(employees: &mut HashMap<String, Vec<String>>, department: &str) {
    if employees.contains_key(department) {
        print_in_department(department, &employees[department]);
    } else {
        println!("No employees in department {}!", department);
    }
}

fn print_in_department(department: &str, employees: &Vec<String>) {
    let mut employees = employees.clone();
    employees.sort();

    println!("Employees in {}:", department);

    for emp in employees {
        println!("{}", emp);
    }
    println!();
}

fn add_to_department(
    employees: &mut HashMap<String, Vec<String>>,
    employee_name: String,
    department: &str,
) {
    employees
        .entry(department.to_string())
        .or_insert(Vec::new())
        .push(employee_name);
}

// -------------------------------------------
// 			Iterator
// -------------------------------------------

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

//iterator is a trait that allows you to iterate over a sequence of items
// intoiter is a trait that allows you to iterate over a sequence of items immutably

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = Employee;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };

    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    for employee in emp_db {
        println!("{:? }", employee);
    }
}

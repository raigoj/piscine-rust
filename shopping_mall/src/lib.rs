pub mod mall;
use std::cmp::Ordering;
use crate::mall::floor::store::employee::Employee;
use crate::mall::floor::store::Store;
use crate::mall::guard::Guard;
use crate::mall::Mall;
pub use crate::mall::floor::store;
pub use crate::mall::floor;
pub fn biggest_store(m: Mall) -> Store {
    m.floors.into_iter()
        .flat_map(|floor| floor.stores.into_iter())
        .max_by_key(|store| store.square_meters).unwrap()
}
pub fn highest_paid_employee(m: Mall) -> Vec<Employee> {
    let mut employees: Vec<_> = m.floors.into_iter()
        .flat_map(|floor| floor.stores.into_iter())
        .flat_map(|store| store.employees.into_iter())
        .collect();
    employees.sort_by(|a, b| b.salary.partial_cmp(&a.salary).unwrap_or(Ordering::Equal));
    if employees.is_empty() {
        return employees;
    }
    let max = employees[0].salary;
    employees.into_iter().take_while(|e| e.salary == max).collect()
}
pub fn nbr_of_employees(m: Mall) -> usize {
    let employees = m.floors.iter()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|store| store.employees.iter())
        .count();
    let guards = m.guards.len();
    employees + guards
}
pub fn fire_old_securities(m: &mut Mall) {
    let old_bois: Vec<_> = m.guards.iter()
        .filter(|&guard| guard.age >= 50)
        .map(|guard| guard.name.clone())
        .collect();
    for name in old_bois {
        m.fire_guard(name)
    }
}
pub fn check_for_securities(m: &mut Mall, mut guards: Vec<Guard>) {
    let total_space = m.floors.iter()
        .fold(0, |v, floor| v + floor.size_limit);
    let mut guarded_space = 200 * m.guards.len() as u64;
    while guarded_space < total_space && !guards.is_empty() {
        m.hire_guard(guards.pop().unwrap());
        guarded_space += 200;
    }
}
pub fn cut_or_raise(m: &mut Mall) {
    let employees = m.floors.iter_mut()
        .flat_map(|floor| floor.stores.iter_mut())
        .flat_map(|store| store.employees.iter_mut());
    for employ in employees {
        let amount = employ.salary * 0.1;
        if employ.working_hours.1 - employ.working_hours.0 > 10 {
            employ.raise(amount)
        } else {
            employ.cut(amount)
        };
    }
}
#[cfg(test)]
mod tests {
    use super::{biggest_store, highest_paid_employee, nbr_of_employees, fire_old_securities, check_for_securities, cut_or_raise};
    use crate::mall::Mall;
    use crate::mall::guard::Guard;
    use crate::mall::floor::Floor;
    use crate::mall::floor::store::Store;
    use crate::mall::floor::store::employee::Employee;
    #[test]
    fn biggest_store_test() {
        let mall = default_mall();
        assert_eq!("Pretail", biggest_store(mall).name)
    }
    #[test]
    fn highest_paid_test() {
        let mall = default_mall();
        let mut result = highest_paid_employee(mall);
        assert_eq!(1, result.len(), "Nobody shares the highest salary in this case");
        assert_eq!("Abdallah Stafford", result[0].name);
        let mut mall = default_mall();
        mall.floors[0].stores[0].hire_employee(result.remove(0));
        let result = highest_paid_employee(mall);
        assert_eq!(2, result.len(), "Two people share the same highest salary, so both should be returned")
    }
    #[test]
    fn nbr_of_employees_test() {
        let mall = default_mall();
        assert_eq!(36, nbr_of_employees(mall))
    }
    #[test]
    fn fire_old_securities_test() {
        let mut mall = default_mall();
        fire_old_securities(&mut mall);
        let expected = vec![
            Guard::new("John Oliver", 34, 7),
            Guard::new("Logan West", 23, 2),
            // Guard::new("Bob Schumacher", 53, 15),
        ];
        assert_eq!(expected, mall.guards);
    }
    #[test]
    fn check_for_securities_test() {
        let mut mall = default_mall();
        let spare_guards = vec![
            Guard::new("1John Oliver", 34, 7),
            Guard::new("1Logan West", 23, 2),
            Guard::new("1Bob Schumacher", 53, 15),
            Guard::new("2John Oliver", 34, 7),
            Guard::new("2Logan West", 23, 2),
            Guard::new("2Bob Schumacher", 53, 15),
        ];
        let prev = mall.guards.len();
        check_for_securities(&mut mall, spare_guards);
        assert_eq!(9, mall.guards.len(), "Guards before: {}, after: {}, expected: 9", prev, mall.guards.len())
    }
    #[test]
    fn cut_or_raise_test() {
        let mut mall = default_mall();
        let old_cut = find_employee(&mall, "Finbar Haines").unwrap();
        let old_raise = find_employee(&mall, "Sienna-Rose Penn").unwrap();
        cut_or_raise(&mut mall);
        let new_cut = find_employee(&mall, "Finbar Haines").unwrap();
        let new_raise = find_employee(&mall, "Sienna-Rose Penn").unwrap();
        assert_eq!(old_cut.salary * 0.9, new_cut.salary, "Finbar Haines needs to be cut: (old: {}, new: {})", old_cut.salary, new_cut.salary);
        assert_eq!(old_raise.salary * 1.1, new_raise.salary, "Sienna-Rose Penn needs to be raised: (old: {}, new: {})", old_raise.salary, new_raise.salary);
    }
    fn find_employee(mall: &Mall, name: &str) -> Option<Employee> {
        mall.floors.iter()
            .flat_map(|floor| floor.stores.iter())
            .flat_map(|store| store.employees.iter())
            .find(|&emp| emp.name == name)
            .cloned()
    }
    fn default_mall() -> Mall {
        let secs = vec![
            Guard::new("John Oliver", 34, 7),
            Guard::new("Logan West", 23, 2),
            Guard::new("Bob Schumacher", 53, 15),
        ];
        let footzo_emp = vec![
            Employee::new("Finbar Haines", 36, 9, 14, 650.88),
            Employee::new("Roksanna Rocha", 45, 13, 22, 772.00),
            Employee::new("Sienna-Rose Penn", 26, 9, 22, 1000.43),
        ];
        let swashion_emp = vec![
            Employee::new("Abdallah Stafford", 54, 8, 22, 1234.21),
            Employee::new("Marian Snyder", 21, 8, 14, 831.90),
            Employee::new("Amanda Mclean", 29, 13, 22, 1222.12),
            Employee::new("Faizaan Castro", 32, 11, 18, 1106.43),
        ];
        let pizbite_emp = vec![
            Employee::new("Juniper Cannon", 21, 16, 23, 804.35),
            Employee::new("Alena Simon", 28, 9, 15, 973.54),
            Employee::new("Yasemin Collins", 29, 9, 19, 986.33),
            Employee::new("Areeb Roberson", 54, 9, 22, 957.82),
            Employee::new("Rocco Amin", 44, 13, 23, 689.21),
        ];
        let grill_emp = vec![
            Employee::new("Rhian Crowther", 45, 9, 15, 841.18),
            Employee::new("Nikkita Steadman", 52, 14, 22, 858.61),
            Employee::new("Reginald Poole", 32, 9, 22, 1197.64),
            Employee::new("Minnie Bull", 54, 14, 22, 1229.73),
        ];
        let sumo_emp = vec![
            Employee::new("Chantelle Barajas", 20, 8, 22, 969.22),
            Employee::new("Hywel Rudd", 49, 12, 22, 695.74),
            Employee::new("Marianne Beasley", 55, 8, 14, 767.83),
        ];
        let supermaket_emp = vec![
            Employee::new("Amara Schaefer", 23, 9, 14, 796.21),
            Employee::new("Yara Wickens", 39, 9, 14, 853.42),
            Employee::new("Tomi Boyer", 64, 9, 14, 881.83),
            Employee::new("Greta Dickson", 42, 9, 14, 775.10),
            Employee::new("Caroline Finnegan", 41, 9, 14, 702.92),
            Employee::new("Indiana Baxter", 33, 13, 20, 991.71),
            Employee::new("Jadine Page", 48, 13, 20, 743.21),
            Employee::new("Husna Ryan", 43, 13, 20, 655.75),
            Employee::new("Tyler Hunt", 63, 13, 20, 668.25),
            Employee::new("Dahlia Caldwell", 56, 13, 20, 781.38),
            Employee::new("Chandler Mansell", 20, 19, 24, 656.75),
            Employee::new("Mohsin Mcgee", 30, 19, 24, 703.83),
            Employee::new("Antoine Goulding", 45, 19, 24, 697.12),
            Employee::new("Mark Barnard", 53, 19, 24, 788.81),
        ];
        let ground_stores = vec![
            Store::new("Footzo", 50, footzo_emp),
            Store::new("Swashion", 43, swashion_emp),
        ];
        let food_stores = vec![
            Store::new("PizBite", 60, pizbite_emp),
            Store::new("Chillout Grill", 50, grill_emp),
            Store::new("Sumo Food", 30, sumo_emp),
        ];
        let supermarket = vec![Store::new("Pretail", 950, supermaket_emp)];
        let floors = vec![
            Floor::new("Ground Floor", ground_stores, 300),
            Floor::new("Food Floor", food_stores, 500),
            Floor::new("Supermarket", supermarket, 1000),
        ];
        Mall::new("La Vie Funchal", secs, floors)
    }
}
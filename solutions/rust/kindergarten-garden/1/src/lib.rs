pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let sliced_diagram: Vec<char> =diagram.trim().chars().collect();
    let student_index = students.iter().position(|&x| x == student);
    if student_index.is_none() {
        return vec![];
    }
    let student_index = student_index.unwrap();
    let sliced_diagram_len = sliced_diagram.len();

    let mut solution: Vec<&'static str> = Vec::new();
    for i in 0..2 {
        let plant_index = (student_index * 2) + i;
        let plant_row_one = match sliced_diagram[plant_index] {
            'V' => "violets",
            'R' => "radishes",
            'C' => "clover",
            'G' => "grass",
            _ => "",
        };
        solution.push(plant_row_one);
    }
    for i in 0..2 {
        let plant_index = (student_index * 2) + i+1 + (sliced_diagram_len / 2);
        let plant_row_two = match sliced_diagram[plant_index] {
            'V' => "violets",
            'R' => "radishes",
            'C' => "clover",
            'G' => "grass",
            _ => "",
        };
        solution.push(plant_row_two);
    }
    solution
}

fn main() {
    let diagram = "VC
RC";
    let student = "Alice";
    let plants_for_alice = plants(diagram, student);
    println!("{:?}", plants_for_alice);
}
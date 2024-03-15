use std::io;

#[derive(Debug, PartialEq)]
enum Courses {
    Math,
    Biology,
    Physics,
    Chemistry,
}

static COURSES_ITER: [Courses; 4] = [
    Courses::Math,
    Courses::Biology,
    Courses::Physics,
    Courses::Chemistry,
];

#[derive(Debug, PartialEq)]
struct CourseGrade {
    name: Courses,
    grade: i64,
}
#[derive(Debug, PartialEq)]
struct Student {
    id: i64,
    name: String,
    courses: Vec<CourseGrade>,
}

fn main() {
    let mut students = vec![
        Student {
            id: 1,
            name: "Noorullah Shaik".to_string(),
            courses: vec![
                CourseGrade {
                    name: Courses::Biology,
                    grade: 99,
                },
                CourseGrade {
                    name: Courses::Math,
                    grade: 99,
                },
                CourseGrade {
                    name: Courses::Physics,
                    grade: 34,
                },
                CourseGrade {
                    name: Courses::Chemistry,
                    grade: 33,
                },
            ],
        },
        Student {
            id: 2,
            name: "Noorullah".to_string(),
            courses: vec![
                CourseGrade {
                    name: Courses::Biology,
                    grade: 90,
                },
                CourseGrade {
                    name: Courses::Math,
                    grade: 98,
                },
                CourseGrade {
                    name: Courses::Physics,
                    grade: 87,
                },
                CourseGrade {
                    name: Courses::Chemistry,
                    grade: 85,
                },
            ],
        },
        Student {
            id: 3,
            name: "Danish".to_string(),
            courses: vec![
                CourseGrade {
                    name: Courses::Biology,
                    grade: 99,
                },
                CourseGrade {
                    name: Courses::Math,
                    grade: 98,
                },
                CourseGrade {
                    name: Courses::Physics,
                    grade: 99,
                },
                CourseGrade {
                    name: Courses::Chemistry,
                    grade: 91,
                },
            ],
        },
    ];
    loop {
        println!(
            "Select Options\n1. Get Student Avg\n2.Add Student\n3.Delete Student\n4.Update Student"
        );
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        let option = option.trim();
        match option.parse() {
            Ok(1) => get_student_avg(&students),
            Ok(2) => add_student(&mut students),
            Ok(3) => delete_student(&mut students),
            Ok(4) => update_student_grades(&mut students),
            Err(_) => println!("Invalid Option!"),
            Ok(i32::MIN..=0_i32) | Ok(5_i32..=i32::MAX) => println!("Invalid Option!"),
        }
    }
}

fn get_student_avg(students: &Vec<Student>) {
    loop {
        let name = input_student_name();
        let student: Option<&Student> = find_student(&students, name);
        if student == None {
            println!("Cannot find student");
            continue;
        } else {
            let avg_grade = get_grades(&student);
            println!("Avg grade of {} is {}", student.unwrap().name, avg_grade);
            break;
        }
    }
}

fn input_student_name() -> String {
    println!("Please input Student Name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();
    return name;
}

fn find_student(students: &Vec<Student>, name: String) -> Option<&Student> {
    for student in students {
        if student.name.eq(&name) {
            return Some(&student);
        }
    }
    return None;
}
fn find_student_index(students: &Vec<Student>, name: String) -> Option<usize> {
    return students.iter().position(|n| n.name == name);
}

fn get_grades(student: &Option<&Student>) -> usize {
    let mut sum = 0;
    for s in &student.unwrap().courses {
        sum += s.grade;
    }
    return sum as usize / student.unwrap().courses.len();
}

fn add_student(mut students: &mut Vec<Student>) {
    let index = students.len() as i64;
    let mut courses = vec![
        CourseGrade {
            name: Courses::Biology,
            grade: 0,
        },
        CourseGrade {
            name: Courses::Math,
            grade: 0,
        },
        CourseGrade {
            name: Courses::Physics,
            grade: 0,
        },
        CourseGrade {
            name: Courses::Chemistry,
            grade: 0,
        },
    ];
    let name = input_student_name();
    input_courses_grade(&mut courses);
    let student = Student {
        id: index,
        name: name,
        courses: courses,
    };
    students.push(student);
}

fn update_student_grades(mut students: &mut Vec<Student>) {
    loop {
        let name = input_student_name();
        let mut student_index: Option<usize> = find_student_index(&students, name);
        if student_index == None {
            println!("Cannot find student");
            continue;
        } else {
            let student_index = student_index.expect("Not Found");
            let mut courses = vec![
                CourseGrade {
                    name: Courses::Biology,
                    grade: 0,
                },
                CourseGrade {
                    name: Courses::Math,
                    grade: 0,
                },
                CourseGrade {
                    name: Courses::Physics,
                    grade: 0,
                },
                CourseGrade {
                    name: Courses::Chemistry,
                    grade: 0,
                },
            ];
            input_courses_grade(&mut courses);
            students[student_index].courses = courses;
            break;
        }
    }
}

fn input_courses_grade(mut courses: &mut Vec<CourseGrade>) {
    let mut index = 0;
    for subject in &COURSES_ITER {
        'inner_loop: loop {
            let mut grade: String = String::new();
            println!("Enter Grade for {:?}", subject);
            io::stdin()
                .read_line(&mut grade)
                .expect("Failed to read line");
            let grade = grade.trim();
            let grade = match grade.parse::<i64>() {
                Ok(g) => g,
                Err(_) => -1,
            };
            if grade == -1 {
                println!("Invalid Grade. Please Enter Again!")
            } else {
                courses[index].grade = grade;
                index += 1;
                break 'inner_loop;
            }
        }
    }
}

fn delete_student(mut students: &mut Vec<Student>) {
    loop {
        let name = input_student_name();
        let student_index: Option<usize> = find_student_index(&students, name);
        if student_index == None {
            println!("Cannot find student");
            continue;
        } else {
            students.remove(student_index.expect("Cannot find student"));
            break;
        }
    }
}

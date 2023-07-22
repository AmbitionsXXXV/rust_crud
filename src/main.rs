use std::collections::HashMap;

// 定义学生结构
struct Student {
    id: u32,
    name: String,
    class_id: u32,
    courses: Vec<u32>,
}

// 定义班级结构
struct Class {
    id: u32,
    name: String,
    student_ids: Vec<u32>,
}

// 定义课程结构
struct Course {
    id: u32,
    name: String,
    student_ids: Vec<u32>,
}

// 定义社团结构
struct Club {
    id: u32,
    name: String,
    student_ids: Vec<u32>,
}

// 定义学生管理系统结构
struct StudentManagementSystem {
    clubs: HashMap<u32, Club>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
    students: HashMap<u32, Student>,
}

impl StudentManagementSystem {
    // 新建学生
    fn create_student(&mut self, id: u32, name: String, class_id: u32) {
        let student = Student {
            id,
            name,
            class_id,
            courses: Vec::new(),
        };
        self.students.insert(id, student);
    }

    // 删除学生
    fn delete_student(&mut self, id: u32) {
        self.students.remove(&id);
    }

    // 更新学生信息
    fn update_student(&mut self, id: u32, name: String, class_id: u32) {
        if let Some(student) = self.students.get_mut(&id) {
            student.name = name;
            student.class_id = class_id;
        }
    }

    // 查询学生信息
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    // 其他对于班级，课程，社团的操作可以类似实现...
}

fn main() {}

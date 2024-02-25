trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I Attend {}. My favarite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}


/// original code
struct MyMember { member_name: String, university_name: String, fav_language_name: String, git_username: String }
impl Person for MyMember {
    fn name(&self) -> String {
        self.member_name.clone()
    }
}

impl Student for MyMember {
    fn university(&self) -> String {
        self.university_name.clone()
    }
}

impl Programmer for MyMember {
    fn fav_language(&self) -> String {
        self.fav_language_name.clone()
    }
}

impl CompSciStudent for MyMember {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn main() {
    let member = MyMember {
        member_name:       "Peter".to_owned(),
        university_name:   "NewYork".to_owned(),
        fav_language_name: "C, Rust".to_owned(),
        git_username:      "peter".to_owned()
    };

    println!("{}", comp_sci_student_greeting(&member));
}


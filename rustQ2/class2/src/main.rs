// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u8,
//     address: String,
//     }
// impl Student {
//     // associated function
//     fn create_student(n:String,a:u8, add: String)-> Student// Function Signature
//     {
//         Student{
//             name:n,
//             age:a,
//             address:add
//         }
//     } 
     
//     fn view_student(self)-> String{
//         let info_student = format!(" name:{} \n Age:{} \n Address: {} \n", 
//                                    self.name,
//                                    self.age,
//                                    self.address,
//                                     );
//                                     info_student
//     }
// }

// fn main(){
//     let student_01=Student::create_student(
//         "abc".to_string(),
//         23,
//         "H # 13 north nazimabad ".to_string());
//         let student_02=Student::create_student(
//             "abcd".to_string(),
//             23,
//             "H # 1 north nazimabad ".to_string());

//         println!("{:?}",student_01);
//         println!("{}",student_01.view_student());

//         println!("{:?}",student_02);
//         println!("{}",student_02.view_student());
// }



//making trait for student struct just signature


#[derive(Debug)]
pub struct Student {
    name: String,
    age: String,
    address: String,
    }


pub trait inforamtion {
    fn info(&self)->String{
        "no vlue found".to_string()
    }
}

impl inforamtion for Student {
    fn info(&self)->String{
        let info_student = format!(
            " name:{} \n Age:{} \n Address: {} \n", 
                self.name,
                self.age,
                self.address
                  );
            info_student
            
        }
} 

 fn main(){
        let student_01=Student { name: String::from("Muneeb"),
            age:String::from("23"),
            address:String::from("H # 13 north nazimabad"),
          };
    println!("{}",student_01.info());
        }
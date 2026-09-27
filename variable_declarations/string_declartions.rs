

// program to understand the string declarations in rust.

fn main() {
   // the below statement throws compilation error
   // let name_str:String = "venu gopal";

   // correct declaration and definition
   let name_str:String = "Venu gopal".to_string();

   /* "Venu Gopal" is a string literal so its data type would 
	be &str which looks like:
    &str:
	┌──────────────┐
	│ pointer ─────┼──────→ "Venu Gopal"
	│ length = 10  │
	└──────────────┘ 

    So, assigning a &str type to String is not a valid syntax
    since both are different data types */
   
   // this generates a compilation error again as there is no
   // data type string in standard rust library of strings
   // maybe the data-type string is a derived data type from primitive data types ?
   //let name_str:string = "venu".to_string();

   println!("Name: {}", name_str);
}

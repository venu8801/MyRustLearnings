

fn main() {
   /*
	the .to_string() method here converts the &str type to
	String type internally this is what is happening:

        The string "VenuGopal" is a string constant
        and this gets stored in the .rodata section of the ELF
        when we call .to_string() methos to this string this gets
        copied to heap allocated memory of data type String
       
        Following shows a mental model of this:
                             ELF executable
                         │
                         ▼
                    .rodata
                 ┌──────────────┐
                 │ VenuGopal    │
                 └──────┬───────┘
                        │
                  .to_string()
                        │
                        │ copies bytes
                        ▼
                       Heap
                 ┌──────────────┐
                 │ VenuGopal    │
                 └──────┬───────┘
                        ▲
                        │
                     String
                 ┌──────────────┐
                 │ ptr          │─────┘
                 │ len = 10     │
                 │ capacity     │
                 └──────────────┘
   */
   let name_str:String = "VenuGopal".to_string();
   println!("name: {}", name_str);
}

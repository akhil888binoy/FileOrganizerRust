use std::{fs::{self, read_dir}, io};



fn main(){   
    readFiles();
    
}

fn readFiles(){
   let dirpath = "E:/Blockchain/Rust/projects/fileorganizer/files";
   match read_dir(dirpath) {
    Ok(entries) => {
        for entry in entries {
            match entry {
                Ok(entry)=>{
                    let path = entry.path();
                    println!("path is : {:?}", path);
                },
                Err(_)=>{
                    println!("Enter a valid input");
                }
            }
        }
    }
    Err(_)=>{
        println!("Enter a valid input");
    }
   }
}
use std::{env::consts::EXE_EXTENSION, fs::{self, read_dir, rename}, io, path::PathBuf};
fn main(){   
    println!("Moving .....");
    readFiles();
    println!("Moved....");
}
fn moveFiles(path : &PathBuf , destinationPath: &str){
    match fs::create_dir_all(destinationPath) {
        Ok(t)=>t,
        Err(e)=> {
            println!("Got an error baby");
        }
    }
    let filename = path.file_name().unwrap().to_str().unwrap();
    let destination = format!("{}/{}", destinationPath , filename);
    match rename(path.to_str().unwrap(), destination){
        Ok(t)=> t,
        Err(e)=>{
            println!("Got an error {} " , e);
        }
    }
    println!("Moved file {} ", filename);
}
fn readFiles(){
   let dirpath = "E:/Blockchain/Rust/projects/fileorganizer/files";
   match read_dir(dirpath) {
    Ok(entries) => {
        for entry in entries {
            match entry {
                Ok(entry)=>{
                    let path = entry.path();
                    let pathextn =  match path.extension() {
                        Some(t) => t,
                        None => {
                            println!("no extension");
                            continue;
                        }
                    };
                    let choicePath  = pathextn.to_str().unwrap();
                    let destinationpathJson = "E:/Blockchain/Rust/projects/fileorganizer/jsonfiles";
                    let destinationpathJS = "E:/Blockchain/Rust/projects/fileorganizer/JSfiles";
                    let destinationpathTS = "E:/Blockchain/Rust/projects/fileorganizer/TSfiles";
                    let destinationpathNone = "E:/Blockchain/Rust/projects/fileorganizer/NONEfiles";
                    let destinationpathTXT = "E:/Blockchain/Rust/projects/fileorganizer/TXTfiles";

                    match choicePath {
                        "json"=>{
                            moveFiles(&path, &destinationpathJson);
                            continue;
                        },
                        "ts"=>{
                           moveFiles(&path, &destinationpathTS);
                           continue;
                        },
                        "js" =>{
                            moveFiles(&path, &destinationpathJS); 
                            continue; 
                        },
                        "txt"=>{
                            moveFiles(&path, &destinationpathTXT);
                            continue;
                        }

                        _=>{
                            println!("Enter a valid choicePath");
                            continue;
                        }
                        
                    }
                    
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

extern crate clap;

use clap::Parser;
use std::sync::Mutex;
use std::{path::PathBuf, thread::scope};
#[derive(Parser, Debug)]
pub struct Grep {
    #[arg(short, long, value_name = "SEARCH TERM")]
    file_name: Option<PathBuf>,
    #[arg(short, long, value_name = "FILE")]
    search_term: Option<String>,
}

pub struct GrepImpl {
    grep_cli_instance: Grep,
}

impl GrepImpl {
    pub fn new(grep_cli_instance: Grep) -> Self {
        Self { grep_cli_instance }
    }

    fn file_exists(&self) -> &PathBuf {
        if let Some(term) = &self.grep_cli_instance.search_term {
            if term.len() > 0 && self.grep_cli_instance.file_name == None {
                println!("\x1b[31m Can not use search argument with file argument \x1b[0m")
            }
        }
        if let Some(path) = &self.grep_cli_instance.file_name {
            if !path.exists() {
                panic!(
                    "\x1b[31m No File or Directory with name: {} \x1b[0m",
                    path.to_str().unwrap()
                )
            } else {
                path
            }
        } else {
            panic!("\x1b[31m File path not specified \x1b[0m")
        }
    }

    fn read_file(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        std::fs::read_to_string(self.file_exists())
            .unwrap()
            .lines()
            .into_iter()
            .for_each(|line| output.push(line.to_string()));
        output
    }

    pub fn search(&self) {
        let result: Mutex<Vec<String>> = Mutex::new(vec![]);
        let work = self.read_file();
        let chunks: Vec<Vec<String>> = work.chunks(4).map(|chunk| chunk.to_vec()).collect();
        chunks.into_iter().for_each(|chunk| {
            match scope(|s| {
                s.spawn(|| {
                    if let Some(term) = &self.grep_cli_instance.search_term {
                        chunk.into_iter().for_each(|el| {
                            let mut lock = result.try_lock();
                            if let Ok(ref mut result) = lock{
                                if el.contains(term){
                                    result.push(el)
                                }
                            }
                        })
                    }
                })
                .join()
            }) {
                Ok(_) => {}
                Err(_) => {
                    panic!("\x1b[31m Error searching for string \x1b[0m")
                }
            }
        });
        let mut lock = result.try_lock();
        if let Ok(ref mut result) = lock{
            if result.len() <= 0{
                println!("\x1b[31m Search term does not exist in the file \x1b[0m");
            }else {
                result.iter()
                .for_each(|el|{
                    println!("\x1b[32m {el} \x1b[0m");
                })
            }
        }
    }
}

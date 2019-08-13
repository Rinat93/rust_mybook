pub mod Threads {
    use std::error::Error;
    use std::io::{self,BufReader,BufWriter};
    use std::io::prelude::*;
    use std::fs::File;
    use std::sync::mpsc::channel;
    use std::path::PathBuf;
    use std::thread::spawn;

    /// Ошибки
    fn proccess_starts<T>(documents: Vec<&T>) -> io::Result<()> {
        let (sender, receiver) = channel();

        let handle = spawn(move || {
            for filename in documents {
                let mut f = File::open(filename)?;
                let mut text = String::new();
                f.read_to_string(&mut text)?;
                if sender.send(text).is_err() {
                    break;
                }
            }
        });
        Ok(())
    }

    pub fn start() {
        let files: Vec<&str> = vec!["test12.txt","test12.txt","test12.txt","test12.txt","test12.txt","test12.txt"];
        let res = proccess_starts(files)?;
    }    
}
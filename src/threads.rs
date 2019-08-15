pub mod Threads {
    use std::error::Error;
    use std::io::{self,BufReader,BufWriter};
    use std::io::prelude::*;
    use std::fs::File;
    use std::sync::mpsc::channel;
    use std::path::PathBuf;
    use std::thread::spawn;
    use std::thread;
    use std::time::Duration;

    /// Ошибки
    fn proccess_starts(documents: Vec<&'static str>){
        
        let (sender, receiver) = channel();

        spawn(move || {
            let se = documents;
            for i in se{
                sender.send(i).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        
        // let received = receiver.recv().unwrap();
        for i in receiver  {
            println!("Got: {}", i);
        }
    }

    pub fn start() {
        let files: Vec<&str> = vec!["test12.txt","test12.txt","test12.txt","test12.txt","test12.txt","test12.txt"];
        let res = proccess_starts(files);
    }    
}
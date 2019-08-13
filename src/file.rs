
pub mod Files_book {

    use std::result::Result;
    use std::error::Error;
    use std::io::{self,BufReader,BufWriter};
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::PathBuf;

    fn grep<R>(target: &str, reader: R) -> io::Result<()> where R: BufRead{
        for line_result in reader.lines() {
            let lines = line_result?;
            if lines.contains(target) {
                println!("{}", lines);
            }
        }
        Ok(())
    }
    // Создание файла
    fn create_file(name: &PathBuf, text: &str) -> Result<(), Box<Error>> {
        let file = File::create(name)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}",text)?;
        Ok(())
    }

    // Поиск по файлам
    fn grep_main() -> Result<(), Box<Error>> {
        // Первый аргумент это строка поиска, остальные файлы
        let mut args = std::env::args().skip(1);
        let target = match args.next() {
            Some(s) => s,
            None => Err("usage: grep Pattern File...")?
        };
        // Преобразует в PathBuf все аргументы
        let files: Vec<PathBuf> = args.map(PathBuf::from).collect(); 
        if files.is_empty() {
            let stdin = io::stdin();
            grep(&target, stdin.lock())?;
        } else {
            for file in files {
                // Требуется полный путь... :(
                create_file(&file, &target)?;
                let f = File::open(&file)?;
                grep(&target, BufReader::new(f))?;
            }
        };
        Ok(())
    }


    pub fn start () {
        let res = grep_main();
        if let Err(err) = res {
            let _ = writeln!(io::stderr(),"{}",err);
        }
    }
}
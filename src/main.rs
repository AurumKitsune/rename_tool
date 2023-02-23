use std::fs;
use std::time::UNIX_EPOCH;
use chrono::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Directory where the files are located
   directory: String,

   /// What to name the files
   #[arg(short, long, default_value_t = String::from("file"))]
   name: String,

   /// Use datetime instead of incrementing numbers
   #[arg(short, long)]
   datetime: bool,

   /// Print output to console
   #[arg(short, long)]
   verbose: bool,
}

fn main() {
    let args = Args::parse();

    let dir_path = args.directory;
    let new_name = args.name;

    let files = fs::read_dir(dir_path.clone()).unwrap();

    let mut filenames = Vec::new();
    let mut file_times = Vec::new();

    for file in files {
        let unwrapped_file = file.unwrap();
        if !unwrapped_file.file_type().unwrap().is_file() {continue;}

        let metadata = fs::metadata(unwrapped_file.path()).unwrap();

        let time_created = metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let time_modified = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        let older_time = if time_created < time_modified {time_created} else {time_modified};

        filenames.push(unwrapped_file.file_name().into_string().unwrap());
        file_times.push(older_time);
    }

    if !args.datetime {
        selection_sort(&mut filenames, &mut file_times);

        for (i, filename) in filenames.iter().enumerate() {
            let (name, extension) = match filename.split_once('.') {
                Some((name, extension)) => (name, format!(".{extension}")),
                None => (filename.as_str(), String::new())
            };
            
            if args.verbose {
                println!("{dir_path}/{name}{extension} -> {dir_path}/{new_name}{i}{extension}");
            }

            fs::rename(format!("{dir_path}/{name}{extension}"), format!("{dir_path}/{new_name}{i}{extension}")).unwrap();
        }
    }
    else {
        for (i, filename) in filenames.iter().enumerate() {
            let (name, extension) = match filename.split_once('.') {
                Some((name, extension)) => (name, format!(".{extension}")),
                None => (filename.as_str(), String::new())
            };

            let naive = NaiveDateTime::from_timestamp_opt(file_times[i] as i64, 0).unwrap();
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

            if args.verbose {
                println!("{dir_path}/{name}{extension} -> {dir_path}/{new_name} {newdate}{extension}");
            }

            fs::rename(format!("{dir_path}/{name}{extension}"), format!("{dir_path}/{new_name} {newdate}{extension}")).unwrap();
        }
    }
}

fn selection_sort(filenames: &mut Vec<String>, file_times: &mut [u64]) {
    for i in 0..filenames.len() {
        let mut min = i;
        for j in i..filenames.len() {
            if file_times[j] < file_times[min] {
                min = j;
            }
        }

        filenames.swap(i, min);
        file_times.swap(i, min);
    }
}

use difference::Changeset;
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    fs,
    hash:: {Hash, Hasher},
    io::{prelude::*, BufReader},
    path::Path,
};
use url;

// file path to store previous website string for comparisons
const ARCHIVE_DIR: &str = "archive";

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() -> Result<(), Box<dyn Error>> {
    // load websites
    let websites = lines_from_file("websites");

    // create directory if not exist
    fs::create_dir_all(&ARCHIVE_DIR)?;

    for website in websites {
        println!("{:#?}", website);
        
        // read from url
        let url = url::Url::parse(&website)?;
        let host_str = url.host_str().unwrap();

        // construct path to read and write archive string
        let website_archive_path: String = format!("{}/{:?}", ARCHIVE_DIR, &host_str);

        // load existing string if exist
        let old_string: String = if Path::new(&website_archive_path).exists() {
            fs::read_to_string(&website_archive_path).expect("no such file")
        } else {
            String::new()
        };

        // send http request to get new string
        let new_string: String = reqwest::blocking::get(&website)?.text()?;

        // if hashes of the two strings are the same, continue
        if calculate_hash(&old_string) == calculate_hash(&new_string) {
            continue
        }
        
        // compare two strings to find diff
        // TODO print diff between the two strings
        let _changeset: Changeset = Changeset::new(&old_string, &new_string, "");
//        println!("{:#?}", changeset.diffs);

        // store new string
        println!("{:#?}", &website_archive_path);
        let mut file = fs::File::create(&website_archive_path)?;
        file.write_all(new_string.as_bytes())?;
        //fs::write(&website_archive_path, &new_string).expect("Unable to write file");
    }
    Ok(())
}
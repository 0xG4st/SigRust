use std::fs;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

struct Signature {
    name: String,
    signature: String,
}

struct Antivirus {
    signatures: HashMap<String, String>,
    real_time_protection: bool,
    email_protection: bool,
}

impl Antivirus {
    fn new(filename: &str) -> Antivirus {
        let mut signatures = HashMap::new();
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(":").collect();
            let name = parts[0].to_string();
            let signature = parts[1].to_string();
            signatures.insert(name, signature);
        }

        Antivirus {
            signatures,
            real_time_protection: false,
            email_protection: false,
        }
    }

    fn enable_real_time_protection(&mut self) {
        self.real_time_protection = true;
    }

    fn enable_email_protection(&mut self) {
        self.email_protection = true;
    }

    fn scan_file(&self, filename: &str) -> bool {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            for (name, signature) in &self.signatures {
                if line.contains(signature) {
                    println!("Virus detected: {} in file: {}", name, filename);
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let filename = "signatures.txt";
    let mut antivirus = Antivirus::new(filename);
    antivirus.enable_real_time_protection();
    antivirus.enable_email_protection();

    let paths = fs::read_dir(".").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            antivirus.scan_file(path.to_str().unwrap());
        }
    }
}

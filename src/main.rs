use std::fs;

use anyhow::Result;
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "edit-file")]
enum Opt {
    Add {
        #[structopt(short, long)]
        source_file: String,

        #[structopt(short, long)]
        fn_name: String,

        #[structopt(short, long)]
        expected_params_count: usize,

        #[structopt(short, long)]
        param_to_set: usize,

        #[structopt(short, long)]
        value_to_set: String,
    },
    Remove {
        #[structopt(short, long)]
        source_file: String,

        #[structopt(short, long)]
        fn_name: String,

        #[structopt(short, long)]
        expected_params_count: usize,

        #[structopt(short, long)]
        param_to_remove: usize,
    },
    Move {
        #[structopt(short, long)]
        source_file: String,

        #[structopt(short, long)]
        fn_name: String,

        #[structopt(short, long)]
        remove_index: usize,

        #[structopt(short, long)]
        insert_index: usize,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Add {
            source_file,
            fn_name,
            expected_params_count,
            param_to_set,
            value_to_set,
        } => {
            let source = fs::read_to_string(source_file)?;
            let re = Regex::new(&format!(r#"[^a-zA-Z_]{}\([^\)]+\)"#, fn_name))?;
            let split_re = Regex::new("(\n|\r\n)")?;
            let params_re = Regex::new(r#"\(([^\)]+)\)"#)?;
            let params_split_re = Regex::new(", ")?;

            for line in split_re.split(&source) {
                let mut matched = false;
                for cap in re.captures_iter(line) {
                    matched = true;

                    let m = cap.get(0).unwrap();

                    // print everything before match
                    print!("{}{}(", &line[0..=m.start()], fn_name);

                    let caps = params_re.captures(m.as_str()).unwrap();
                    let params_str = caps.get(1).unwrap().as_str();
                    let mut params = params_split_re.split(params_str).collect::<Vec<_>>();

                    if params.len() < expected_params_count {
                        params.insert(param_to_set - 1, &value_to_set);
                    }

                    // print new params
                    let new_params = params.join(", ");
                    print!("{})", new_params);

                    // print everything following the match
                    println!("{}", &line[m.end()..]);
                }

                if !matched {
                    println!("{}", line);
                }
            }
        }

        Opt::Remove {
            source_file,
            fn_name,
            expected_params_count,
            param_to_remove,
        } => {
            let source = fs::read_to_string(source_file)?;
            let re = Regex::new(&format!(r#"[^a-zA-Z_]{}\([^\)]+\)"#, fn_name))?;
            let split_re = Regex::new("(\n|\r\n)")?;
            let params_re = Regex::new(r#"\(([^\)]+)\)"#)?;
            let params_split_re = Regex::new(", ")?;

            for line in split_re.split(&source) {
                let mut matched = false;
                for cap in re.captures_iter(line) {
                    matched = true;

                    let m = cap.get(0).unwrap();

                    // print everything before match
                    print!("{}{}(", &line[0..=m.start()], fn_name);

                    let caps = params_re.captures(m.as_str()).unwrap();
                    let params_str = caps.get(1).unwrap().as_str();
                    let mut params = params_split_re.split(params_str).collect::<Vec<_>>();

                    if params.len() > expected_params_count {
                        params.remove(param_to_remove);
                    }

                    // print new params
                    let new_params = params.join(", ");
                    print!("{})", new_params);

                    // print everything following the match
                    println!("{}", &line[m.end()..]);
                }

                if !matched {
                    println!("{}", line);
                }
            }
        }

        Opt::Move {
            source_file,
            fn_name,
            remove_index,
            insert_index,
        } => {
            let source = fs::read_to_string(source_file)?;
            let re = Regex::new(&format!(r#"[^a-zA-Z_]{}\([^\)]+\)"#, fn_name))?;
            let split_re = Regex::new("(\n|\r\n)")?;
            let params_re = Regex::new(r#"\(([^\)]+)\)"#)?;
            let params_split_re = Regex::new(", ")?;

            for line in split_re.split(&source) {
                let mut matched = false;
                for cap in re.captures_iter(line) {
                    matched = true;

                    let m = cap.get(0).unwrap();

                    // print everything before match
                    print!("{}{}(", &line[0..=m.start()], fn_name);

                    let caps = params_re.captures(m.as_str()).unwrap();
                    let params_str = caps.get(1).unwrap().as_str();
                    let mut params = params_split_re.split(params_str).collect::<Vec<_>>();

                    let move_param = params.remove(remove_index);
                    params.insert(insert_index, move_param);

                    // print new params
                    let new_params = params.join(", ");
                    print!("{})", new_params);

                    // print everything following the match
                    println!("{}", &line[m.end()..]);
                }

                if !matched {
                    println!("{}", line);
                }
            }
        }
    }

    Ok(())
}

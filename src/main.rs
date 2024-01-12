//! <https://github.com/NotLeonian/not_leonian_ac_submit>  
//!   
//! Copyright (c) 2023 Not_Leonian  
//! Released under the MIT license  
//! <https://opensource.org/licenses/mit-license.php>  

use std::{env::{args, current_dir}, process::exit, io::Result, path::PathBuf, fs::read_to_string, collections::VecDeque};

use regex::Regex;

fn main() -> Result<()> {
    let args=args().collect::<Vec<String>>();
    if args.len()!=3 {
        eprintln!("コマンドライン引数に、not_leonian_ac_libのlib.rsのパスと提出したいbin名を指定してください。");
        exit(1);
    }
    let current_dir=current_dir()?;
    let lib_path=PathBuf::from(&args[1]);
    let lib=read_to_string(lib_path)?;
    let cargo_toml=read_to_string(&current_dir.join("Cargo.toml"))?;
    let pattern=format!("{}{}", args[2], " = \\{ alias = \"([^\"]+)\"");
    let re=Regex::new(&pattern).unwrap();
    let submit_file_name=re.captures(&cargo_toml).unwrap().get(1).unwrap().as_str();
    let submit_file=read_to_string(&current_dir.join("src").join("bin")
    .join(format!("{}.rs", submit_file_name)))?;
    let pattern="use not_leonian_ac_lib";
    if submit_file.contains(pattern) {
        let mut header_comment=String::new();
        let mut footer_comment=String::new();
        let mut blocks=Vec::<String>::new();
        for line in lib.lines() {
            if line=="" {
                blocks.push(String::new());
            } else {
                if line.contains("//!") {
                    header_comment.push_str(&line);
                    header_comment.push('\n');
                } else if line.contains("// not_leonian_ac_lib until this line") {
                    footer_comment.push_str(&line);
                    footer_comment.push('\n');
                } else {
                    blocks.last_mut().unwrap().push_str(&line);
                    blocks.last_mut().unwrap().push('\n');
                }
            }
        }
        header_comment.push('\n');
        let len=blocks.len();
        let mut needed=vec![false;len];
        let mut depends=vec![Vec::<usize>::new();len];
        let mut type_idents=Vec::new();
        for i in 0..len {
            let names_and_res=if {
                let pattern="(?m)^ *macro_rules[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let pattern="macro_rules *! *([^ {]+) *\\{";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("((?m)^|[^a-zA-Z_]){} *!", name);
                let re=Regex::new(&pattern).unwrap();
                if re.is_match(&submit_file) {
                    needed[i]=true;
                }
                type_idents.push(name);
                Some(vec![(name,re)])
            } else if {
                let pattern="((?m)^ *|[^a-zA-Z_])struct[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let pattern="struct *([^ <{(;]+) *(<|\\{|\\(|;|w)";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("[^a-zA-Z_:]{}[^a-zA-Z_]", name);
                let re=Regex::new(&pattern).unwrap();
                if re.is_match(&submit_file) {
                    needed[i]=true;
                }
                type_idents.push(name);
                Some(vec![(name,re)])
            } else if {
                let pattern="((?m)^ *|[^a-zA-Z_])trait[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let mut names_and_res=Vec::new();
                let pattern="trait *([^ <{]+) *(<|\\{|w)";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("[^a-zA-Z_]{}[^a-zA-Z_]", name);
                let re=Regex::new(&pattern).unwrap();
                names_and_res.push((name,re));
                let pattern="fn +([^ <(]+) *(<|\\(|w)";
                let re=Regex::new(&pattern).unwrap();
                for capture in re.captures_iter(&blocks[i]) {
                    let name=capture.get(1).unwrap().as_str();
                    let pattern=format!("[^a-zA-Z_]{} *\\(", name);
                    let re=Regex::new(&pattern).unwrap();
                    if re.is_match(&submit_file) {
                        needed[i]=true;
                    }
                    names_and_res.push((name,re));
                }
                type_idents.push(name);
                Some(names_and_res)
            } else if {
                let pattern="(?m)^ *impl[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                None
            } else if {
                let pattern="((?m)^ *|[^a-zA-Z_])type[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let pattern="type +([^ <=]+) *(<|=)";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("[^a-zA-Z_:]{}[^a-zA-Z_]", name);
                let re=Regex::new(&pattern).unwrap();
                if re.is_match(&submit_file) {
                    needed[i]=true;
                }
                type_idents.push(name);
                Some(vec![(name,re)])
            } else if {
                let pattern="((?m)^ *|[^a-zA-Z_])enum[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let pattern="enum *([^ <{]+) *(<|\\{|w)";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("[^a-zA-Z_:]{}[^a-zA-Z_]", name);
                let re=Regex::new(&pattern).unwrap();
                if re.is_match(&submit_file) {
                    needed[i]=true;
                }
                type_idents.push(name);
                Some(vec![(name,re)])
            } else if {
                let pattern="((?m)^ *|[^a-zA-Z_])fn[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let pattern="fn +([^ <(]+) *(<|\\(|w)";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("[^a-zA-Z_]{} *(\\(|:)", name);
                let re=Regex::new(&pattern).unwrap();
                if re.is_match(&submit_file) {
                    needed[i]=true;
                }
                type_idents.push(name);
                Some(vec![(name,re)])
            } else if {
                let pattern="((?m)^ *|[^a-zA-Z_])const[^a-zA-Z_]";
                let re=Regex::new(&pattern).unwrap();
                re.is_match(&blocks[i].lines().filter(|line| !line.contains("///"))
                .fold(String::new(), |lib, str| lib+str+"\n"))
            } {
                let pattern="const +([^ :]+) *:";
                let re=Regex::new(&pattern).unwrap();
                let name=re.captures(&blocks[i]).unwrap().get(1).unwrap().as_str();
                let pattern=format!("[^a-zA-Z_]{}[^a-zA-Z_]", name);
                let re=Regex::new(&pattern).unwrap();
                if re.is_match(&submit_file) {
                    needed[i]=true;
                }
                type_idents.push(name);
                Some(vec![(name,re)])
            } else {
                None
            };
            if let Some(names_and_res)=names_and_res {
                for j in 0..len {
                    if i==j {
                        continue;
                    }
                    for (name,re) in names_and_res.clone() {
                        if re.is_match(&blocks[j].lines().filter(|line| !line.contains("///")).fold(String::new(),
                        |lib, str| lib+str+"\n")) {
                            if {
                                let pattern="(?m)^ *impl[^a-zA-Z_]";
                                let re=Regex::new(&pattern).unwrap();
                                re.is_match(&blocks[j].lines().filter(|line| !line.contains("///")).fold(String::new(),
                                |lib, str| lib+str+"\n"))
                            } {
                                let pattern="impl(<[^>]+>)? +([^ <{]+) *(<|\\{|f|w)";
                                let re=Regex::new(&pattern).unwrap();
                                let impl_name=re.captures(&blocks[j]).unwrap().get(2).unwrap().as_str();
                                if name==impl_name {
                                    depends[i].push(j);
                                } else if {
                                    let pattern=format!("for +{}", name);
                                    let re=Regex::new(&pattern).unwrap();
                                    !type_idents.contains(&impl_name) && re.is_match(&blocks[j].lines().filter(|line| !line.contains("///"))
                                    .fold(String::new(), |lib, str| lib+str+"\n"))
                                } {
                                    depends[i].push(j);
                                } else {
                                    depends[j].push(i);
                                }
                            } else if {
                                let pattern=format!("(?m)^ *{} *!", name);
                                let re=Regex::new(&pattern).unwrap();
                                re.is_match(&blocks[j].lines().filter(|line| !line.contains("///")).fold(String::new(),
                                |lib, str| lib+str+"\n"))
                            } {
                                depends[i].push(j);
                            } else {
                                depends[j].push(i);
                            }
                        }
                    }
                }
            }
        }
        let mut queue=VecDeque::<usize>::new();
        for i in 0..len {
            if needed[i] {
                queue.push_front(i);
            }
        }
        while let Some(v)=queue.pop_back() {
            for &u in &depends[v] {
                if !needed[u] {
                    needed[u]=true;
                    queue.push_front(u);
                }
            }
        }
        let submit_file=format!("{}{}{}{}", header_comment, blocks.iter().enumerate().filter(|&(i,_)| needed[i])
        .fold(String::new(), |lib, (_,str)| lib+str+"\n"), footer_comment, submit_file.lines().filter_map(|line| {
            if line.contains(pattern) {
                None
            } else {
                Some(format!("{}\n", line))
            }
        }).collect::<String>());
        print!("{}", submit_file);
    } else {
        print!("{}", submit_file);
    }
    Ok(())
}

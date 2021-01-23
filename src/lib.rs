/// 2021.01.13 21:22:05 Log        -  [VRCApplicationSetup] System Info:
/// <-------date------> <type>        <Log::VRCApplicationSetup> <message>
use std::str::FromStr;

pub mod log;

#[derive(Debug)]
pub enum Type {
    Log(log::Type),
    Warning,
    Error,
    Exception,
    Unknown(String),
}

#[derive(Debug)]
pub struct Log {
    pub date: String,
    pub typ: Type,
    pub msg: Vec<String>,
}

pub fn from_str(s: &str) -> Result<Vec<Log>, ()> {
    let mut ret = Vec::<Log>::new();
    for log in s.split("\n\r\n") {
        let log = Log::from_str(log);

        if let Ok(log) = log {
            //println!("{:?}", log);
            ret.push(log);
        }
    }

    Ok(ret)
}

impl Log {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        if s.chars().nth(31) != Some('-') {
            println!("parse error: {}", s);
            return Err(());
        }

        let date = s[0..19].to_string();
        //println!("date: {}", date);

        let typ = &s[20..31].split_whitespace().next().unwrap();
        let content = &s[34..];

        let (mtyp, msg) = if content.chars().nth(0) == Some('[') {
            let n = content.find("]").unwrap();
            (Some(&content[1..n]), &content[n + 2..])
        } else {
            (None, content)
        };

        let mut msg: Vec<String> = msg
            .lines()
            .map(|s| s.trim_start_matches(' ').to_string())
            .collect();
        msg.retain(|s| !s.is_empty());
        let msg = msg;

        let typ = match typ {
            &"Log" => {
                let lt = if let Some(mtyp) = mtyp {
                    log::Type::from_str(mtyp).unwrap()
                } else {
                    log::Type::Message
                };
                Type::Log(lt)
            }
            &"Warning" => Type::Warning,
            &"Error" => Type::Error,
            &"Exception" => Type::Exception,
            _ => Type::Unknown(typ.to_string()),
        };

        let log = Log { typ, date, msg };

        Ok(log)
    }
}

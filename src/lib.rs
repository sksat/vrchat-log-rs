/// 2021.01.13 21:22:05 Log        -  [VRCApplicationSetup] System Info:
/// <-------date------> <type>        <Log::VRCApplicationSetup> <message>
use chrono::NaiveDateTime as DateTime;
use enum_as_inner::EnumAsInner;

pub mod log;
pub mod world;

pub use log::Log;
pub use world::{Instance, InstanceLog, InstanceLogList};

#[derive(Debug, EnumAsInner)]
pub enum LogEnum {
    Log(Log),
    Warning { date: DateTime, msg: Vec<String> },
    Error { date: DateTime, msg: Vec<String> },
    Exception { date: DateTime, msg: Vec<String> },
    Unknown(String),
}

pub fn from_str(s: &str) -> Result<Vec<LogEnum>, ()> {
    let mut ret = Vec::<LogEnum>::new();
    for log in s.split("\n\r\n") {
        let log = LogEnum::from_str(log);

        if let Ok(log) = log {
            //println!("{:?}", log);
            ret.push(log);
        }
    }

    Ok(ret)
}

impl LogEnum {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        if s.chars().nth(31) != Some('-') {
            //println!("parse error: {}", s);
            return Err(());
        }

        let date = s[0..19].to_string();
        let date = DateTime::parse_from_str(&date, "%Y.%m.%d %H:%M:%S").unwrap();
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

        let log = match typ {
            &"Log" => LogEnum::Log(Log::new(date, mtyp, msg)),
            &"Warning" => LogEnum::Warning { date, msg },
            &"Error" => LogEnum::Error { date, msg },
            &"Exception" => LogEnum::Exception { date, msg },
            _ => LogEnum::Unknown(typ.to_string()),
        };

        Ok(log)
    }
}

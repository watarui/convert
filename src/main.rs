use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use regex::Regex;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = env::args().nth(1).unwrap();
    let input = File::open(file_name).unwrap();

    let reader = BufReader::new(input);

    let output = File::create("./output.csv").unwrap();
    let mut buf = BufWriter::new(output);

    let header = [
        "",
        "Attendance",
        "total_elapsed_time",
        "max_elapsed_time",
        "AttendancePreflight",
        "total_elapsed_time",
        "max_elapsed_time",
        "Login",
        "total_elapsed_time",
        "max_elapsed_time",
        "LoginPreflight",
        "total_elapsed_time",
        "max_elapsed_time",
        "Registration",
        "total_elapsed_time",
        "max_elapsed_time",
        "RegistrationPreflight",
        "total_elapsed_time",
        "max_elapsed_time",
        "VerifySession",
        "total_elapsed_time",
        "max_elapsed_time",
        "VerifySessionPreflight",
        "total_elapsed_time",
        "max_elapsed_time",
    ]
    .join(",");
    writeln!(buf, "{}", header)?;

    for line in reader.lines() {
        let l = match line {
            Ok(l) => l,
            Err(e) => panic!("There was a problem opening the file: {:?}", e),
        };

        let re =
            Regex::new(r"^[\d-]+ ([\d:.]+) \[INFO\] \[[a-z/.:\d]+\] - (\{[^\} ][^$]+)").unwrap();

        if let Some(m) = re.captures(&l) {
            let time = &m[1];
            let json = &m[2];

            let mut data = vec![String::from(time)];
            let val: Value = serde_json::from_str(json)?;

            [
                &val["Attendance"]["AggByStatusCode"]["200"],
                &val["AttendancePreflight"]["AggByStatusCode"]["204"],
                &val["Login"]["AggByStatusCode"]["200"],
                &val["LoginPreflight"]["AggByStatusCode"]["204"],
                &val["Registration"]["AggByStatusCode"]["200"],
                &val["RegistrationPreflight"]["AggByStatusCode"]["204"],
                &val["VerifySession"]["AggByStatusCode"]["200"],
                &val["VerifySessionPreflight"]["AggByStatusCode"]["204"],
            ]
            .iter()
            .for_each(|v| {
                let ss = if !v.is_null() {
                    (
                        v["count"].to_string(),
                        v["total_elapsed_time"].to_string(),
                        v["max_elapsed_time"].to_string(),
                    )
                } else {
                    (String::new(), String::new(), String::new())
                };
                data.append(&mut vec![ss.0, ss.1, ss.2])
            });

            writeln!(buf, "{}", data.join(","))?;
        }
    }
    Ok(())
}

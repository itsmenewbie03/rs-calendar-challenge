use chrono::{Datelike, Utc, Weekday};
use datetimeutils::{days_in_month, month_from_index};
use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    print!("Enter Year: ");
    io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut year)
        .expect("Failed to read input");
    let year_trimmed = year.trim();
    let year_num = year_trimmed.parse::<i32>().expect("Failed to convert year");
    println!("{:-^27}", &format!("| {year_trimmed} Calendar |"));
    for x in 1..=12 {
        let date = Utc::now()
            .with_year(year_num)
            .unwrap()
            .with_month(x)
            .unwrap();
        let days = days_in_month(
            date.year() as u64,
            month_from_index(date.month() as u64).unwrap(),
        );
        let mut days_vec = vec![vec![0; 7]; 6];
        let mut cur_week = 0;
        for day in 1..=days {
            let temp_date = date.with_day(day as u32).unwrap();
            let weekday = temp_date.weekday();
            match weekday {
                Weekday::Sun => days_vec[cur_week][0] = day,
                Weekday::Mon => days_vec[cur_week][1] = day,
                Weekday::Tue => days_vec[cur_week][2] = day,
                Weekday::Wed => days_vec[cur_week][3] = day,
                Weekday::Thu => days_vec[cur_week][4] = day,
                Weekday::Fri => days_vec[cur_week][5] = day,
                Weekday::Sat => {
                    days_vec[cur_week][6] = day;
                    cur_week += 1;
                }
            };
        }
        println!();
        println!(
            "{} {}",
            month_from_index(date.month() as u64).unwrap(),
            date.year()
        );
        println!("Sun Mon Tue Wed Thu Fri Sat");
        for dw in days_vec {
            for x in dw {
                match x {
                    0 => print!("    "),
                    _ => print!("{:>3} ", x.to_string()),
                }
            }
            println!();
        }
    }
}

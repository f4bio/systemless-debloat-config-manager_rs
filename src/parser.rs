use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use regex::RegexSetBuilder;

pub fn find_system_apps(s: &str) -> IResult<&str, &str> {
  tag("System apps, not debloated:")(s)
}

pub fn find_system_packages(s: &str) -> IResult<&str, &str> {
  tag("System packages:")(s)
}

pub fn find_apps(s: &str) -> IResult<&str, (&str, &str)> {
  separated_pair(find_system_apps, char('\n'), find_system_packages)(s)
}

pub fn find_with_regex(s: &str) {
  let set = RegexSetBuilder::new(&[
    r#"Input-DebloatList="[[:alpha:]]*""#,
    r#"System apps, not debloated:.*"#,
  ])
  .case_insensitive(true)
  .build()
  .unwrap();

  s.lines()
    .filter_map(|line| line.parse::<String>().ok())
    .filter(|line| set.is_match(line.as_str()))
    .for_each(|x| println!("{}", x))
}

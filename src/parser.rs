use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;

pub fn find_system_apps(s: &str) -> IResult<&str, &str> {
  tag("System apps, not debloated:")(s)
}

pub fn find_system_packages(s: &str) -> IResult<&str, &str> {
  tag("System packages:")(s)
}

pub fn find_apps(s: &str) -> IResult<&str, (&str, &str)> {
  separated_pair(find_system_apps, char('\n'), find_system_packages)(s)
}

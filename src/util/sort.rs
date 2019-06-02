#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SortDirection {
    ASC,
    DESC,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SortBy {
    PID,
    Name,
    CPU,
    Memory,
}

impl std::str::FromStr for SortBy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pid" => Ok(SortBy::PID),
            "name" => Ok(SortBy::Name),
            "cpu" => Ok(SortBy::CPU),
            "mem" => Ok(SortBy::Memory),
            _ => Err(()),
        }
    }
}

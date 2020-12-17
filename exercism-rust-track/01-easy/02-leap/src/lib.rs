pub fn is_leap_year(year: u64) -> bool {
    let div_by_four = year % 4 == 0;

    let div_by_hundred = year % 100 == 0;
    
    let div_by_four_hundred = year % 400 == 0;

    div_by_four && !div_by_hundred | div_by_four_hundred
}

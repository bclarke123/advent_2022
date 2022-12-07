pub type DayFunc = (fn(), fn());

pub trait Day {
    fn get_parts() -> DayFunc;
}

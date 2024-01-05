use num::integer::Integer;
use num::ToPrimitive;

pub trait Point<T>
where
    T: Integer + ToPrimitive,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
}

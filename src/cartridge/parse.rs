pub type ParseResult<T> = Result<T, String>;

pub trait Parse<T = u8, V = Self> {
    fn parse(t: T) -> ParseResult<V>;
}

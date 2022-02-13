use crate::generic::Parser;

pub fn optional<'a, P, A>(parser: P) -> impl Parser<'a, Option<A>>
where
    P: Parser<'a, A>,
{
    move |input| match parser.parse(input) {
        Ok((next_input, item)) => Ok((next_input, Some(item))),
        Err(_) => Ok((input, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::literal;

    #[test]
    fn test_optional() -> Result<(), String> {
        let parser = optional(literal("foo"));

        assert_eq!(parser.parse("foo")?, ("", Some(())));
        assert_eq!(parser.parse("bar")?, ("bar", None));
        assert_eq!(parser.parse("")?, ("", None));
        Ok(())
    }
}

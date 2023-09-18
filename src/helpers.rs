use std::marker::PhantomData;

use combine::error::Tracked;
use combine::stream::easy::{Error, Errors, Info};
use combine::{satisfy, ParseResult, Parser, StreamOnce};

use crate::position::Pos;
use crate::tokenizer::{Kind, Token, TokenStream};

use super::common::Text;

#[derive(Debug, Clone)]
pub struct TokenMatch<'a> {
    kind: Kind,
    phantom: PhantomData<&'a u8>,
}

#[derive(Debug, Clone)]
pub struct NameMatch<'a, T>
    where
        T: Text<'a>,
{
    phantom: PhantomData<&'a T>,
}

#[derive(Debug, Clone)]
pub struct StringMatch<'a, T>
    where
        T: Text<'a>,
{
    block: bool,
    phantom: PhantomData<&'a T>,
}


#[derive(Debug, Clone)]
pub struct Value<'a> {
    kind: Kind,
    value: &'static str,
    phantom: PhantomData<&'a u8>,
}

pub fn kind<'x>(kind: Kind) -> TokenMatch<'x> {
    TokenMatch {
        kind,
        phantom: PhantomData,
    }
}

pub fn name<'a, T>() -> NameMatch<'a, T>
    where
        T: Text<'a>,
{
    NameMatch {
        phantom: PhantomData,
    }
}

pub fn _string<'a, T>() -> StringMatch<'a, T>
    where T: Text<'a>
{
    StringMatch {
        block: false,
        phantom: PhantomData,
    }
}

pub fn _blockstring<'a, T>() -> StringMatch<'a, T>
    where T: Text<'a>
{
    StringMatch {
        block: true,
        phantom: PhantomData,
    }
}


impl<'a> Parser<TokenStream<'a>> for TokenMatch<'a> {
    type Output = Token<'a>;
    type PartialState = ();

    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut TokenStream<'a>,
    ) -> ParseResult<Self::Output, <TokenStream<'a> as StreamOnce>::Error> {
        satisfy(|c: Token<'a>| c.kind == self.kind).parse_lazy(input)
    }

    fn add_error(&mut self, error: &mut Tracked<Errors<Token<'a>, Token<'a>, Pos>>) {
        error
            .error
            .add_error(Error::Expected(Info::Owned(format!("{:?}", self.kind))));
    }
}

pub fn punct<'s>(value: &'static str) -> Value<'s> {
    Value {
        kind: Kind::Punctuator,
        value,
        phantom: PhantomData,
    }
}

pub fn ident<'s>(value: &'static str) -> Value<'s> {
    Value {
        kind: Kind::Name,
        value,
        phantom: PhantomData,
    }
}

impl<'a> Parser<TokenStream<'a>> for Value<'a> {
    type Output = Token<'a>;
    type PartialState = ();

    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut TokenStream<'a>,
    ) -> ParseResult<Self::Output, <TokenStream<'a> as StreamOnce>::Error> {
        satisfy(|c: Token<'a>| c.kind == self.kind && c.value == self.value).parse_lazy(input)
    }

    fn add_error(&mut self, error: &mut Tracked<<TokenStream<'a> as StreamOnce>::Error>) {
        error
            .error
            .add_error(Error::Expected(Info::Static(self.value)));
    }
}

impl<'a, S> Parser<TokenStream<'a>> for NameMatch<'a, S>
    where
        S: Text<'a>,
{
    type Output = S;
    type PartialState = ();

    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut TokenStream<'a>,
    ) -> ParseResult<Self::Output, <TokenStream<'a> as StreamOnce>::Error> {
        satisfy(|c: Token<'a>| c.kind == Kind::Name)
            .map(|t: Token<'a>| -> S { S::from(t.value) })
            .parse_lazy(input)
    }

    fn add_error(&mut self, error: &mut Tracked<Errors<Token<'a>, Token<'a>, Pos>>) {
        error.error.add_error(Error::Expected(Info::Static("Name")));
    }
}

impl<'a, S> Parser<TokenStream<'a>> for StringMatch<'a, S>
    where
        S: Text<'a>,
{
    type Output = S;
    type PartialState = ();

    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut TokenStream<'a>,
    ) -> ParseResult<Self::Output, <TokenStream<'a> as StreamOnce>::Error> {
        satisfy(|c: Token<'a>| if self.block { c.kind == Kind::BlockString } else { c.kind == Kind::StringValue })
            .map(|t: Token<'a>| -> S {
                if self.block {
                    S::from(&t.value[3..=t.value.len() - 3])
                } else {
                    S::from(&t.value[1..=t.value.len() - 1])
                }
            })
            .parse_lazy(input)
    }

    fn add_error(&mut self, error: &mut Tracked<Errors<Token<'a>, Token<'a>, Pos>>) {
        error.error.add_error(Error::Expected(Info::Static("String")));
    }
}


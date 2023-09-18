use std::str::FromStr;

use thiserror::Error;

pub use crate::common::{Directive, Text, Type, Value};
use crate::position::Pos;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document<'a, T: Text<'a>>
    where
        T: Text<'a>,
{
    pub definitions: Vec<Definition<'a, T>>,
}

impl<'a> Document<'a, String> {
    pub fn into_static(self) -> Document<'static, String> {
        // To support both reference and owned values in the AST,
        // all string data is represented with the ::common::Str<'a, T: Text<'a>>
        // wrapper type.
        // This type must carry the liftetime of the schema string,
        // and is stored in a PhantomData value on the Str type.
        // When using owned String types, the actual lifetime of
        // the Ast nodes is 'static, since no references are kept,
        // but the nodes will still carry the input lifetime.
        // To continue working with Document<String> in a owned fasion
        // the lifetime needs to be transmuted to 'static.
        //
        // This is safe because no references are present.
        // Just the PhantomData lifetime reference is transmuted away.
        unsafe { std::mem::transmute::<_, Document<'static, String>>(self) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition<'a, T: Text<'a>> {
    SchemaDefinition(SchemaDefinition<'a, T>),
    SchemaExtension(SchemaExtension<'a, T>),
    TypeDefinition(TypeDefinition<'a, T>),
    TypeExtension(TypeExtension<'a, T>),
    DirectiveDefinition(DirectiveDefinition<'a, T>),
}

impl<'a, T: Text<'a>> Definition<'a, T> {
    pub fn into_schema_definition(self) -> Option<SchemaDefinition<'a, T>> {
        match self {
            Self::SchemaDefinition(d) => Some(d),
            _ => None,
        }
    }

    pub fn into_schema_extension(self) -> Option<SchemaExtension<'a, T>> {
        match self {
            Self::SchemaExtension(d) => Some(d),
            _ => None,
        }
    }

    pub fn into_type_definition(self) -> Option<TypeDefinition<'a, T>> {
        match self {
            Self::TypeDefinition(d) => Some(d),
            _ => None,
        }
    }

    pub fn into_type_extension(self) -> Option<TypeExtension<'a, T>> {
        match self {
            Self::TypeExtension(d) => Some(d),
            _ => None,
        }
    }

    pub fn into_directive_definition(self) -> Option<DirectiveDefinition<'a, T>> {
        match self {
            Self::DirectiveDefinition(d) => Some(d),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SchemaDefinition<'a, T: Text<'a>> {
    pub position: Pos,
    pub directives: Vec<Directive<'a, T>>,
    pub query: Option<T>,
    pub mutation: Option<T>,
    pub subscription: Option<T>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SchemaExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub directives: Vec<Directive<'a, T>>,

    pub query: Option<T>,
    pub mutation: Option<T>,
    pub subscription: Option<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefinition<'a, T: Text<'a>> {
    Scalar(ScalarType<'a, T>),
    Object(ObjectType<'a, T>),
    Interface(InterfaceType<'a, T>),
    Union(UnionType<'a, T>),
    Enum(EnumType<'a, T>),
    InputObject(InputObjectType<'a, T>),
}

impl<'a, T: Text<'a>> TypeDefinition<'a, T> {
    pub fn name(&self) -> &T {
        match self {
            TypeDefinition::Scalar(s) => &s.name,
            TypeDefinition::Object(o) => &o.name,
            TypeDefinition::Interface(i) => &i.name,
            TypeDefinition::Union(u) => &u.name,
            TypeDefinition::Enum(e) => &e.name,
            TypeDefinition::InputObject(i) => &i.name,
        }
    }
    pub fn as_scalar(&self) -> Option<&ScalarType<'a, T>> {
        match self {
            Self::Scalar(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&ObjectType<'a, T>> {
        match self {
            Self::Object(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_interface(&self) -> Option<&InterfaceType<'a, T>> {
        match self {
            Self::Interface(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_union(&self) -> Option<&UnionType<'a, T>> {
        match self {
            Self::Union(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&EnumType<'a, T>> {
        match self {
            Self::Enum(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_input(&self) -> Option<&InputObjectType<'a, T>> {
        match self {
            Self::InputObject(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_scalar(self) -> Option<ScalarType<'a, T>> {
        match self {
            Self::Scalar(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_object(self) -> Option<ObjectType<'a, T>> {
        match self {
            Self::Object(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_interface(self) -> Option<InterfaceType<'a, T>> {
        match self {
            Self::Interface(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_union(self) -> Option<UnionType<'a, T>> {
        match self {
            Self::Union(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_enum(self) -> Option<EnumType<'a, T>> {
        match self {
            Self::Enum(s) => Some(s),
            _ => None,
        }
    }
    pub fn to_input(self) -> Option<InputObjectType<'a, T>> {
        match self {
            Self::InputObject(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExtension<'a, T: Text<'a>> {
    Scalar(ScalarTypeExtension<'a, T>),
    Object(ObjectTypeExtension<'a, T>),
    Interface(InterfaceTypeExtension<'a, T>),
    Union(UnionTypeExtension<'a, T>),
    Enum(EnumTypeExtension<'a, T>),
    InputObject(InputObjectTypeExtension<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarType<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
}

impl<'a, T> ScalarType<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarTypeExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
}

impl<'a, T> ScalarTypeExtension<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub implements_interfaces: Vec<T>,
    pub directives: Vec<Directive<'a, T>>,
    pub fields: Vec<Field<'a, T>>,
}

impl<'a, T> ObjectType<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectTypeExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T,
    pub implements_interfaces: Vec<T>,
    pub directives: Vec<Directive<'a, T>>,
    pub fields: Vec<Field<'a, T>>,
}

impl<'a, T> ObjectTypeExtension<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub arguments: Vec<InputValue<'a, T>>,
    pub field_type: Type<'a, T>,
    pub directives: Vec<Directive<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputValue<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub value_type: Type<'a, T>,
    pub default_value: Option<Value<'a, T>>,
    pub directives: Vec<Directive<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceType<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub implements_interfaces: Vec<T>,
    pub directives: Vec<Directive<'a, T>>,
    pub fields: Vec<Field<'a, T>>,
}

impl<'a, T> InterfaceType<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceTypeExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T,
    pub implements_interfaces: Vec<T>,
    pub directives: Vec<Directive<'a, T>>,
    pub fields: Vec<Field<'a, T>>,
}

impl<'a, T> InterfaceTypeExtension<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionType<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
    pub types: Vec<T>,
}

impl<'a, T> UnionType<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
            types: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionTypeExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
    pub types: Vec<T>,
}

impl<'a, T> UnionTypeExtension<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
            types: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
    pub values: Vec<EnumValue<'a, T>>,
}

impl<'a, T> EnumType<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
            values: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
}

impl<'a, T> EnumValue<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumTypeExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
    pub values: Vec<EnumValue<'a, T>>,
}

impl<'a, T> EnumTypeExtension<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
            values: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectType<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
    pub fields: Vec<InputValue<'a, T>>,
}

impl<'a, T> InputObjectType<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectTypeExtension<'a, T: Text<'a>> {
    pub position: Pos,
    pub name: T,
    pub directives: Vec<Directive<'a, T>>,
    pub fields: Vec<InputValue<'a, T>>,
}

impl<'a, T> InputObjectTypeExtension<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectiveLocation {
    // executable
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,

    // type_system
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
    VariableDefinition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveDefinition<'a, T: Text<'a>> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: T,
    pub arguments: Vec<InputValue<'a, T>>,
    pub repeatable: bool,
    pub locations: Vec<DirectiveLocation>,
}

impl<'a, T> DirectiveDefinition<'a, T>
    where
        T: Text<'a>,
{
    pub fn new(name: T) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            arguments: vec![],
            repeatable: false,
            locations: vec![],
        }
    }
}

impl DirectiveLocation {
    /// Returns GraphQL syntax compatible name of the directive
    pub fn as_str(&self) -> &'static str {
        use self::DirectiveLocation::*;
        match *self {
            Query => "QUERY",
            Mutation => "MUTATION",
            Subscription => "SUBSCRIPTION",
            Field => "FIELD",
            FragmentDefinition => "FRAGMENT_DEFINITION",
            FragmentSpread => "FRAGMENT_SPREAD",
            InlineFragment => "INLINE_FRAGMENT",
            Schema => "SCHEMA",
            Scalar => "SCALAR",
            Object => "OBJECT",
            FieldDefinition => "FIELD_DEFINITION",
            ArgumentDefinition => "ARGUMENT_DEFINITION",
            Interface => "INTERFACE",
            Union => "UNION",
            Enum => "ENUM",
            EnumValue => "ENUM_VALUE",
            InputObject => "INPUT_OBJECT",
            InputFieldDefinition => "INPUT_FIELD_DEFINITION",
            VariableDefinition => "VARIABLE_DEFINITION",
        }
    }

    /// Returns `true` if this location is for queries (execution)
    pub fn is_query(&self) -> bool {
        use self::DirectiveLocation::*;
        match *self {
            Query | Mutation | Subscription | Field | FragmentDefinition | FragmentSpread
            | InlineFragment => true,

            Schema | Scalar | Object | FieldDefinition | ArgumentDefinition | Interface | Union
            | Enum | EnumValue | InputObject | InputFieldDefinition | VariableDefinition => false,
        }
    }

    /// Returns `true` if this location is for schema
    pub fn is_schema(&self) -> bool {
        !self.is_query()
    }
}

#[derive(Debug, Error)]
#[error("invalid directive location")]
pub struct InvalidDirectiveLocation;

impl FromStr for DirectiveLocation {
    type Err = InvalidDirectiveLocation;
    fn from_str(s: &str) -> Result<DirectiveLocation, InvalidDirectiveLocation> {
        use self::DirectiveLocation::*;
        let val = match s {
            "QUERY" => Query,
            "MUTATION" => Mutation,
            "SUBSCRIPTION" => Subscription,
            "FIELD" => Field,
            "FRAGMENT_DEFINITION" => FragmentDefinition,
            "FRAGMENT_SPREAD" => FragmentSpread,
            "INLINE_FRAGMENT" => InlineFragment,
            "SCHEMA" => Schema,
            "SCALAR" => Scalar,
            "OBJECT" => Object,
            "FIELD_DEFINITION" => FieldDefinition,
            "ARGUMENT_DEFINITION" => ArgumentDefinition,
            "INTERFACE" => Interface,
            "UNION" => Union,
            "ENUM" => Enum,
            "ENUM_VALUE" => EnumValue,
            "INPUT_OBJECT" => InputObject,
            "INPUT_FIELD_DEFINITION" => InputFieldDefinition,
            "VARIABLE_DEFINITION" => VariableDefinition,
            _ => return Err(InvalidDirectiveLocation),
        };

        Ok(val)
    }
}

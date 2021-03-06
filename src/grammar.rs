use std::{ str, slice, fmt };
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct OwnedSlice {
    ptr: *const u8,
    len: usize,
}

impl OwnedSlice {
    #[inline]
    pub unsafe fn from_str(source: &str) -> Self {
        OwnedSlice {
            ptr: source.as_ptr(),
            len: source.len(),
        }
    }

    #[inline]
    pub fn from_static(source: &'static str) -> Self {
        OwnedSlice {
            ptr: source.as_ptr(),
            len: source.len(),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.as_bytes())
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl From<&'static str> for OwnedSlice {
    #[inline]
    fn from(source: &'static str) -> Self {
        OwnedSlice {
            ptr: source.as_ptr(),
            len: source.len(),
        }
    }
}

impl Deref for OwnedSlice {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for OwnedSlice {
    #[inline]
    fn eq(&self, other: &OwnedSlice) -> bool {
        self.as_str() == other.as_str()
    }
}

impl fmt::Debug for OwnedSlice {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for OwnedSlice {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LiteralValue {
    LiteralUndefined,
    LiteralNull,
    LiteralTrue,
    LiteralFalse,
    LiteralInteger(u64),
    LiteralFloat(OwnedSlice),
    LiteralString(OwnedSlice),
}
pub use self::LiteralValue::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Parameter {
    pub name: OwnedSlice,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperatorType {
    FatArrow,         //   …  => …
    Accessor,         //   …  .  …
    New,              //     new …
    Increment,        //      ++ … | … ++
    Decrement,        //      -- … | … --
    LogicalNot,       //       ! …
    BitwiseNot,       //       ~ …
    Typeof,           //  typeof …
    Void,             //    void …
    Delete,           //  delete …
    Multiplication,   //   …  *  …
    Division,         //   …  /  …
    Remainder,        //   …  %  …
    Exponent,         //   …  ** …
    Addition,         //   …  +  … | + …
    Substraction,     //   …  -  … | - …
    BitShiftLeft,     //   …  << …
    BitShiftRight,    //   …  >> …
    UBitShiftRight,   //   … >>> …
    Lesser,           //   …  <  …
    LesserEquals,     //   …  <= …
    Greater,          //   …  >  …
    GreaterEquals,    //   …  >= …
    Instanceof,       //   … instanceof …
    In,               //   …  in …
    StrictEquality,   //   … === …
    StrictInequality, //   … !== …
    Equality,         //   …  == …
    Inequality,       //   …  != …
    BitwiseAnd,       //   …  &  …
    BitwiseXor,       //   …  ^  …
    BitwiseOr,        //   …  |  …
    LogicalAnd,       //   …  && …
    LogicalOr,        //   …  || …
    Conditional,      //   …  ?  …  :  …
    Assign,           //   …  =  …
    AddAssign,        //   …  += …
    SubstractAssign,  //   …  -= …
    ExponentAssign,   //   … **= …
    MultiplyAssign,   //   …  *= …
    DivideAssign,     //   …  /= …
    RemainderAssign,  //   …  %= …
    BSLAssign,        //   … <<= …
    BSRAssign,        //   … >>= …
    UBSRAssign,       //   … >>>= …
    BitAndAssign,     //   …  &= …
    BitXorAssign,     //   …  ^= …
    BitOrAssign,      //   …  |= …
    Spread,           //     ... …
}
use self::OperatorType::*;

impl OperatorType {
    /// According to the Operator Precedence Table
    /// Note: Unary opearotrs default to 15!
    pub fn binding_power(&self) -> u8 {
        match *self {
            FatArrow         |
            Accessor         => 18,

            New              => 17,

            Increment        |
            Decrement        => 16,

            LogicalNot       |
            BitwiseNot       |
            Typeof           |
            Void             |
            Delete           => 15,

            Multiplication   |
            Division         |
            Remainder        |
            Exponent         => 14,

            Addition         |
            Substraction     => 13,

            BitShiftLeft     |
            BitShiftRight    |
            UBitShiftRight   => 12,

            Lesser           |
            LesserEquals     |
            Greater          |
            GreaterEquals    |
            Instanceof       |
            In               => 11,

            StrictEquality   |
            StrictInequality |
            Equality         |
            Inequality       => 10,

            BitwiseAnd       => 9,
            BitwiseXor       => 8,
            BitwiseOr        => 7,
            LogicalAnd       => 6,
            LogicalOr        => 5,
            Conditional      => 4,

            Assign           |
            AddAssign        |
            SubstractAssign  |
            ExponentAssign   |
            MultiplyAssign   |
            DivideAssign     |
            RemainderAssign  |
            BSLAssign        |
            BSRAssign        |
            UBSRAssign       |
            BitAndAssign     |
            BitXorAssign     |
            BitOrAssign      => 3,

            Spread           => 1,
        }
    }

    pub fn prefix(&self) -> bool {
        match *self {
            LogicalNot       |
            BitwiseNot       |
            Typeof           |
            Void             |
            Delete           |
            New              |
            Spread           |
            Increment        |
            Decrement        |
            Addition         |
            Substraction     => true,

            _                => false
        }
    }

    pub fn infix(&self) -> bool {
        match *self {
            FatArrow         |
            Accessor         |
            Multiplication   |
            Division         |
            Remainder        |
            Exponent         |
            StrictEquality   |
            StrictInequality |
            Equality         |
            Inequality       |
            Lesser           |
            LesserEquals     |
            Greater          |
            GreaterEquals    |
            Instanceof       |
            In               |
            BitShiftLeft     |
            BitShiftRight    |
            UBitShiftRight   |
            BitwiseAnd       |
            BitwiseXor       |
            BitwiseOr        |
            LogicalAnd       |
            LogicalOr        |
            Conditional      |
            Addition         |
            Substraction     |
            Assign           |
            AddAssign        |
            SubstractAssign  |
            ExponentAssign   |
            MultiplyAssign   |
            DivideAssign     |
            RemainderAssign  |
            BSLAssign        |
            BSRAssign        |
            UBSRAssign       |
            BitAndAssign     |
            BitXorAssign     |
            BitOrAssign      => true,

            _                => false
        }
    }

    pub fn assignment(&self) -> bool {
        match *self {
            Assign           |
            AddAssign        |
            SubstractAssign  |
            ExponentAssign   |
            MultiplyAssign   |
            DivideAssign     |
            RemainderAssign  |
            BSLAssign        |
            BSRAssign        |
            UBSRAssign       |
            BitAndAssign     |
            BitXorAssign     |
            BitOrAssign      => true,

            _                => false
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    This,
    Identifier(OwnedSlice),
    Literal(LiteralValue),
    Array(Vec<Expression>),
    Sequence(Vec<Expression>),
    Object(Vec<ObjectMember>),
    Member {
        object: Box<Expression>,
        property: OwnedSlice,
    },
    ComputedMember {
        object: Box<Expression>,
        property: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: OperatorType,
        right: Box<Expression>,
    },
    Prefix {
        operator: OperatorType,
        operand: Box<Expression>,
    },
    Postfix {
        operator: OperatorType,
        operand: Box<Expression>,
    },
    Conditional {
        test: Box<Expression>,
        consequent: Box<Expression>,
        alternate: Box<Expression>,
    },
    ArrowFunction {
        params: Vec<Parameter>,
        body: Box<Statement>,
    },
    Function {
        name: Option<OwnedSlice>,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    }
}

impl Expression {
    pub fn binding_power(&self) -> u8 {
        match *self {
            Expression::Member {
                ..
            }
            |
            Expression::ArrowFunction {
                ..
            } => 18,

            Expression::Call {
                ..
            } => 17,

            Expression::Prefix {
                ..
            } => 15,

            Expression::Binary {
                ref operator,
                ..
            }
            |
            Expression::Postfix {
                ref operator,
                ..
            } => operator.binding_power(),

            Expression::Conditional {
                ..
            } => 4,

            _  => 100,
        }
    }

    #[inline]
    pub fn binary<E: Into<Expression>>(left: E, operator: OperatorType, right: E) -> Self {
        Expression::Binary {
            operator: operator,
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }

    #[inline]
    pub fn member<E: Into<Expression>, S: Into<OwnedSlice>>(object: E, property: S) -> Self {
        Expression::Member {
            object: Box::new(object.into()),
            property: property.into(),
        }
    }

    #[inline]
    pub fn call<E: Into<Expression>>(callee: E, arguments: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(callee.into()),
            arguments: arguments,
        }
    }
}

impl From<&'static str> for Expression {
    #[inline]
    fn from(ident: &'static str) -> Self {
        Expression::Identifier(OwnedSlice::from_static(ident))
    }
}

impl From<OwnedSlice> for Expression {
    #[inline]
    fn from(ident: OwnedSlice) -> Self {
        Expression::Identifier(ident)
    }
}

impl<'a> From<&'a OwnedSlice> for Expression {
    #[inline]
    fn from(ident: &'a OwnedSlice) -> Self {
        Expression::Identifier(*ident)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectMember {
    Shorthand {
        key: OwnedSlice,
    },
    Literal {
        key: OwnedSlice,
        value: Expression,
    },
    Computed {
        key: Expression,
        value: Expression,
    },
    Method {
        name: OwnedSlice,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    },
    ComputedMethod {
        name: Expression,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClassMember {
    Constructor {
        params: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Method {
        is_static: bool,
        name: OwnedSlice,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Property {
        is_static: bool,
        name: OwnedSlice,
        value: Expression,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub name: OwnedSlice,
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Block {
        body: Vec<Statement>,
    },
    // `Transparent` is not part of the language grammar, just a helper that
    // allows the transformer to replace a single statement with mutliple
    // statements without messing with parent array.
    Transparent {
        body: Vec<Statement>,
    },
    Labeled {
        label: OwnedSlice,
        body: Box<Statement>,
    },
    VariableDeclaration {
        kind: VariableDeclarationKind,
        declarators: Vec<VariableDeclarator>,
    },
    Expression {
        value: Expression
    },
    Return {
        value: Option<Expression>,
    },
    Break {
        label: Option<OwnedSlice>,
    },
    Function {
        name: OwnedSlice,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    },
    If {
        test: Expression,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },
    While {
        test: Expression,
        body: Box<Statement>,
    },
    For {
        init: Option<Box<Statement>>,
        test: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
    },
    ForIn {
        left: Box<Statement>,
        right: Expression,
        body: Box<Statement>,
    },
    ForOf {
        left: Box<Statement>,
        right: Expression,
        body: Box<Statement>,
    },
    Class {
        name: OwnedSlice,
        extends: Option<OwnedSlice>,
        body: Vec<ClassMember>,
    },
    Throw {
        value: Expression
    },
}

impl From<Expression> for Statement {
    #[inline]
    fn from(expression: Expression) -> Self {
        Statement::Expression {
            value: expression
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    source: String,
    pub body: Vec<Statement>,
}

impl Program {
    #[inline]
    pub fn new(source: String, body: Vec<Statement>) -> Self {
        Program {
            source: source,
            body: body,
        }
    }
}

extern crate itoa;

use std::ptr;

use grammar::*;
use grammar::OperatorType::*;

/// The `Generator` is a wrapper around an owned `String` that's used to
/// stringify the AST. There is a bunch of useful methods here to manage
/// things like indentation and automatically producing minified code.
struct Generator {
    pub minify: bool,
    code: Vec<u8>,
    dent: u16,
}

impl Generator {
    pub fn new(minify: bool) -> Self {
        Generator {
            minify: minify,
            code: Vec::with_capacity(128),
            dent: 0,
        }
    }

    #[inline]
    pub fn new_line(&mut self) {
        if !self.minify {
            self.write_byte(b'\n');
            for _ in 0..self.dent {
                self.write_bytes(b"    ");
            }
        }
    }

    #[inline]
    pub fn write<T: Code>(&mut self, item: &T) {
        item.to_code(self);
    }

    #[inline]
    pub fn write_byte(&mut self, ch: u8) {
        self.code.push(ch);
    }

    #[inline]
    pub fn write_bytes(&mut self, slice: &[u8]) {
        extend_from_slice(&mut self.code, slice);
    }

    #[inline]
    pub fn write_min(&mut self, slice: &[u8], minslice: &[u8]) {
        if self.minify {
            self.write_bytes(minslice);
        } else {
            self.write_bytes(slice);
        }
    }

    #[inline]
    pub fn write_list<T: Code>(&mut self, items: &Vec<T>) {
        let mut iter = items.iter();

        for item in iter.next() {
            self.write(item);
        }

        for item in iter {
            self.write_min(b", ", b",");
            self.write(item);
        }
    }

    #[inline]
    pub fn write_block<T: Code>(&mut self, items: &Vec<T>) {
        self.indent();
        for item in items {
            self.new_line();
            self.write(item);
        }
        self.dedent();
        self.new_line();
    }

    pub fn write_declaration_or_expression(&mut self, statement: &Statement) {
        match *statement {
            Statement::VariableDeclaration {
                ref kind,
                ref declarators,
            } => {
                self.write(kind);
                self.write_byte(b' ');
                self.write_list(declarators);
            },

            Statement::Expression {
                ref value,
            } => {
                value.to_code(self);
            },

            _ => panic!("Invalid AST structure!"),
        }
    }

    #[inline]
    pub fn indent(&mut self) {
        self.dent += 1;
    }

    #[inline]
    pub fn dedent(&mut self) {
        self.dent -= 1;
    }

    #[inline]
    pub fn consume(self) -> String {
        unsafe { String::from_utf8_unchecked(self.code) }
    }
}

/// The `Code` trait provides an interface to pieces of grammar, that allows
/// to efficiently write characters and string slices to the code `Generator`.
trait Code {
    fn to_code(&self, gen: &mut Generator);
}

impl Code for u64 {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        itoa::write(&mut gen.code, *self).expect("Can't fail on a Vec");
    }
}

impl<T: Code> Code for Box<T> {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        gen.write(self.as_ref());
    }
}

impl Code for OwnedSlice {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        extend_from_slice(&mut gen.code, self.as_bytes());
    }
}

impl<T: Code> Code for Option<T> {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        match *self {
            Some(ref value) => value.to_code(gen),
            None            => {}
        }
    }
}

impl Code for OperatorType {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        gen.write_bytes(match *self {
            FatArrow         => b"=>",
            Accessor         => b".",
            New              => b"new",
            Increment        => b"++",
            Decrement        => b"--",
            LogicalNot       => b"!",
            BitwiseNot       => b"~",
            Typeof           => b"typeof",
            Void             => b"void",
            Delete           => b"delete",
            Multiplication   => b"*",
            Division         => b"/",
            Remainder        => b"%",
            Exponent         => b"**",
            Addition         => b"+",
            Substraction     => b"-",
            BitShiftLeft     => b"<<",
            BitShiftRight    => b">>",
            UBitShiftRight   => b">>>",
            Lesser           => b"<",
            LesserEquals     => b"<=",
            Greater          => b">",
            GreaterEquals    => b">=",
            Instanceof       => b"instanceof",
            In               => b"in",
            StrictEquality   => b"===",
            StrictInequality => b"!==",
            Equality         => b"==",
            Inequality       => b"!=",
            BitwiseAnd       => b"&",
            BitwiseXor       => b"^",
            BitwiseOr        => b"|",
            LogicalAnd       => b"&&",
            LogicalOr        => b"||",
            Conditional      => b"?",
            Assign           => b"=",
            AddAssign        => b"+=",
            SubstractAssign  => b"-=",
            ExponentAssign   => b"**=",
            MultiplyAssign   => b"*=",
            DivideAssign     => b"/=",
            RemainderAssign  => b"%=",
            BSLAssign        => b"<<=",
            BSRAssign        => b">>=",
            UBSRAssign       => b">>>=",
            BitAndAssign     => b"&=",
            BitXorAssign     => b"^=",
            BitOrAssign      => b"|=",
            Spread           => b"...",
        });
    }
}

impl Code for LiteralValue {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        match *self {
            LiteralUndefined          => gen.write_min(b"undefined", b"void 0"),
            LiteralNull               => gen.write_bytes(b"null"),
            LiteralTrue               => gen.write_min(b"true", b"!0",),
            LiteralFalse              => gen.write_min(b"false", b"!1"),
            LiteralInteger(ref num)   => gen.write(num),
            LiteralFloat(ref num)     => gen.write(num),
            LiteralString(ref string) => gen.write(string),
        }
    }
}

impl Code for ObjectMember {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        match *self {
            ObjectMember::Shorthand {
                ref key
            } => gen.write(key),

            ObjectMember::Literal {
                ref key,
                ref value,
            } => {
                gen.write(key);
                gen.write_min(b": ", b":");
                gen.write(value);
            },

            ObjectMember::Computed {
                ref key,
                ref value,
            } => {
                gen.write_byte(b'[');
                gen.write(key);
                gen.write_min(b"]: ", b"]:");
                gen.write(value);
            },

            ObjectMember::Method {
                ref name,
                ref params,
                ref body,
            } => {
                gen.write(name);
                gen.write_byte(b'(');
                gen.write_list(params);
                gen.write_min(b") {", b"){");
                gen.write_block(body);
                gen.write_byte(b'}');
            },

            ObjectMember::ComputedMethod {
                ref name,
                ref params,
                ref body,
            } => {
                gen.write_byte(b'[');
                gen.write(name);
                gen.write_bytes(b"](");
                gen.write_list(params);
                gen.write_min(b") {", b"){");
                gen.write_block(body);
                gen.write_byte(b'}');
            },
        }
    }
}

impl Code for Parameter {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        gen.write_bytes(self.name.as_bytes());
    }
}

impl Code for Expression {
    fn to_code(&self, gen: &mut Generator) {
        match *self {

            Expression::This => gen.write_bytes(b"this"),

            Expression::Identifier(ref ident) => gen.write(ident),

            Expression::Literal(ref literal)  => gen.write(literal),

            Expression::Array(ref items) => {
                gen.write_byte(b'[');
                gen.write_list(items);
                gen.write_byte(b']');
            },

            Expression::Sequence(ref items) => {
                gen.write_byte(b'(');
                gen.write_list(items);
                gen.write_byte(b')');
            },

            Expression::Object(ref members) => {
                gen.write_byte(b'{');
                gen.indent();

                let mut iter = members.iter();

                for member in iter.next() {
                    gen.new_line();
                    gen.write(member);
                }

                for member in iter {
                    gen.write_byte(b',');
                    gen.new_line();
                    gen.write(member);
                }

                gen.dedent();
                gen.new_line();
                gen.write_byte(b'}');
            },

            Expression::Member {
                ref object,
                ref property,
            } => {
                gen.write(object);
                gen.write_byte(b'.');
                gen.write(property);
            },

            Expression::ComputedMember {
                ref object,
                ref property,
            } => {
                gen.write(object);
                gen.write_byte(b'[');
                gen.write(property);
                gen.write_byte(b']');
            },

            Expression::Call {
                ref callee,
                ref arguments,
            } => {
                gen.write(callee);
                gen.write_byte(b'(');
                gen.write_list(arguments);
                gen.write_byte(b')');
            },

            Expression::Binary {
                ref left,
                ref operator,
                ref right,
            } => {
                if left.binding_power() < self.binding_power() {
                    gen.write_byte(b'(');
                    gen.write(left);
                    gen.write_byte(b')');
                } else {
                    gen.write(left);
                }
                gen.write_min(b" ", b"");
                gen.write(operator);
                gen.write_min(b" ", b"");
                gen.write(right);
            },

            Expression::Prefix {
                ref operator,
                ref operand,
            } => {
                gen.write(operator);
                gen.write(operand);
            },

            Expression::Postfix {
                ref operator,
                ref operand,
            } => {
                gen.write(operand);
                gen.write(operator);
            },

            Expression::Conditional {
                ref test,
                ref consequent,
                ref alternate,
            } => {
                gen.write(test);
                gen.write_min(b" ? ", b"?");
                gen.write(consequent);
                gen.write_min(b" : ", b":");
                gen.write(alternate);
            },

            Expression::ArrowFunction {
                ref params,
                ref body,
            } => {
                if params.len() == 1 {
                    gen.write(&params[0]);
                } else {
                    gen.write_byte(b'(');
                    gen.write_list(params);
                    gen.write_byte(b')');
                }
                gen.write_min(b" => ", b"=>");
                match **body {
                    Statement::Expression {
                        ref value,
                    } => gen.write(value),
                    _ => gen.write(body),
                }
            },

            Expression::Function {
                ref name,
                ref params,
                ref body,
            } => {
                gen.write_bytes(b"function");
                if let Some(ref name) = *name {
                    gen.write_byte(b' ');
                    gen.write(name);
                } else {
                    gen.write_min(b" ", b"");
                }
                gen.write_byte(b'(');
                gen.write_list(params);
                gen.write_min(b") {", b"){");
                gen.write_block(body);
                gen.write_byte(b'}');
            },

            // _ => gen.write_byte('💀'),
        }
    }
}

impl Code for VariableDeclarationKind {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        gen.write_bytes(match *self {
            VariableDeclarationKind::Var   => b"var",
            VariableDeclarationKind::Let   => b"let",
            VariableDeclarationKind::Const => b"const",
        })
    }
}

impl Code for ClassMember {
    fn to_code(&self, gen: &mut Generator) {
        match *self {

            ClassMember::Constructor {
                ref params,
                ref body,
            } => {
                gen.write_bytes(b"constructor(");
                gen.write_list(params);
                gen.write_min(b") {", b"){");
                gen.write_block(body);
                gen.write_byte(b'}');
            },

            ClassMember::Method {
                is_static,
                ref name,
                ref params,
                ref body,
            } => {
                if is_static {
                    gen.write_bytes(b"static ");
                }
                gen.write(name);
                gen.write_byte(b'(');
                gen.write_list(params);
                gen.write_min(b") {", b"){");
                gen.write_block(body);
                gen.write_byte(b'}');
            },

            ClassMember::Property {
                is_static,
                ref name,
                ref value,
            } => {
                if is_static {
                    gen.write_bytes(b"static ");
                }
                gen.write(name);
                gen.write_min(b" = ", b"=");
                gen.write(value);
                gen.write_byte(b';');
            }
        }
    }
}

impl Code for VariableDeclarator {
    #[inline]
    fn to_code(&self, gen: &mut Generator) {
        gen.write(&self.name);
        if let Some(ref value) = self.value {
            gen.write_min(b" = ", b"=");
            gen.write(value);
        }
    }
}

impl Code for Statement {
    fn to_code(&self, gen: &mut Generator) {
        match *self {
            Statement::Labeled {
                ref label,
                ref body,
            } => {
                gen.write(label);
                gen.write_min(b": ", b":");
                gen.write(body);
            },

            Statement::Block {
                ref body,
            } => {
                gen.write_byte(b'{');
                gen.write_block(body);
                gen.write_byte(b'}');
            },

            Statement::Transparent {
                ref body,
            } => {
                let mut iter = body.iter();

                for statement in iter.next() {
                    gen.write(statement);
                }

                for statement in iter {
                    gen.new_line();
                    gen.write(statement);
                }
            },

            Statement::Expression {
                ref value,
            } => {
                gen.write(value);
                gen.write_byte(b';');
            },

            Statement::Return {
                ref value,
            } => {
                gen.write_bytes(b"return");
                if let Some(ref value) = *value {
                    gen.write_byte(b' ');
                    gen.write(value);
                }
                gen.write_byte(b';');
            },

            Statement::Break {
                ref label,
            } => {
                gen.write_bytes(b"break");
                if let Some(ref label) = *label {
                    gen.write_byte(b' ');
                    gen.write(label);
                }
                gen.write_byte(b';');
            },

            Statement::VariableDeclaration {
                ref kind,
                ref declarators,
            } => {
                gen.write(kind);
                gen.write_byte(b' ');
                gen.write_list(declarators);
                gen.write_byte(b';');
            },

            Statement::Function {
                ref name,
                ref params,
                ref body,
            } => {
                gen.new_line();
                gen.write_bytes(b"function ");
                gen.write(name);
                gen.write_byte(b'(');
                gen.write_list(params);
                gen.write_min(b") {", b"){");
                gen.write_block(body);
                gen.write_byte(b'}');
                gen.new_line();
            },

            Statement::If {
                ref test,
                ref consequent,
                ref alternate,
            } => {
                gen.write_min(b"if (", b"if(");
                gen.write(test);
                gen.write_min(b") ", b")");
                gen.write(consequent);

                if let Some(ref alternate) = *alternate {
                    gen.write_bytes(b" else ");
                    gen.write(alternate);
                };
            },

            Statement::While {
                ref test,
                ref body,
            } => {
                gen.write_min(b"while (", b"while(");
                gen.write(test);
                gen.write_min(b") ", b")");
                gen.write(body);
            },

            Statement::For {
                ref init,
                ref test,
                ref update,
                ref body,
            } => {
                gen.write_min(b"for (", b"for(");
                if let Some(ref init) = *init {
                    gen.write_declaration_or_expression(init);
                }
                gen.write_min(b"; ", b";");
                gen.write(test);
                gen.write_min(b"; ", b";");
                gen.write(update);
                gen.write_min(b") ", b")");
                gen.write(body);
            },

            Statement::ForIn {
                ref left,
                ref right,
                ref body,
            } => {
                gen.write_min(b"for (", b"for(");
                gen.write_declaration_or_expression(left);
                gen.write_bytes(b" in ");
                gen.write(right);
                gen.write_min(b") ", b")");
                gen.write(body);
            },

            Statement::ForOf {
                ref left,
                ref right,
                ref body,
            } => {
                gen.write_min(b"for (", b"for(");
                gen.write_declaration_or_expression(left);
                gen.write_bytes(b" of ");
                gen.write(right);
                gen.write_min(b") ", b")");
                gen.write(body);
            },

            Statement::Class {
                ref name,
                ref extends,
                ref body,
            } => {
                gen.new_line();
                gen.write_bytes(b"class ");
                gen.write(name);
                if let &Some(ref super_class) = extends {
                    gen.write_bytes(b" extends ");
                    gen.write(super_class);
                }
                gen.write_min(b" {", b"{");
                gen.write_block(body);
                gen.write_byte(b'}');
                gen.new_line();
            },

            Statement::Throw {
                ref value,
            } => {
                gen.write_bytes(b"throw ");
                gen.write(value);
                gen.write_byte(b';');
            }
        }
    }
}

pub fn generate_code(program: Program, minify: bool) -> String {
    let mut gen = Generator::new(minify);

    for statement in program.body {
        gen.write(&statement);
        gen.new_line();
    }

    gen.consume()
}


// From: https://github.com/dtolnay/fastwrite/blob/master/src/lib.rs#L68
//
// LLVM is not able to lower `Vec::extend_from_slice` into a memcpy, so this
// helps eke out that last bit of performance.
#[inline]
fn extend_from_slice(dst: &mut Vec<u8>, src: &[u8]) {
    let dst_len = dst.len();
    let src_len = src.len();

    dst.reserve(src_len);

    unsafe {
        // We would have failed if `reserve` overflowed
        dst.set_len(dst_len + src_len);

        ptr::copy_nonoverlapping(
            src.as_ptr(),
            dst.as_mut_ptr().offset(dst_len as isize),
            src_len);
    }
}


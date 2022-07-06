use std::{collections::BTreeMap, str::FromStr};

use rainbow_pest::{
    ast::{ASTProgram, ASTStatement, MetaStatement, RangedObject, RangedValue, SchemaStatement},
    ParserConfig,
};

use crate::{schema::Value, RainbowError, Result, Schema};

impl FromStr for Schema {
    type Err = RainbowError;

    fn from_str(s: &str) -> Result<Self> {
        let parser = ParserConfig::default();
        let ast = parser.parse(s)?;
        Schema::try_from(ast)
    }
}

impl TryFrom<ASTProgram> for Schema {
    type Error = RainbowError;

    fn try_from(program: ASTProgram) -> Result<Self> {
        let mut out = Schema::default();
        let mut ctx = SchemaContext::default();
        for i in program.statements {
            match i {
                ASTStatement::Schema(node) => {
                    out.eval_schema(node, &mut ctx)?;
                }
                ASTStatement::Meta(node) => {
                    out.eval_meta(node, &mut ctx)?;
                }
                ASTStatement::Language(node) => {
                    println!("{:#?}", node);
                    todo!()
                }
            }
        }
        Ok(out)
    }
}

impl Default for SchemaContext {
    fn default() -> Self {
        Self { first_schema: true }
    }
}

struct SchemaContext {
    first_schema: bool,
}

impl Schema {
    fn eval_schema(&mut self, ast: SchemaStatement, ctx: &mut SchemaContext) -> Result<()> {
        if ctx.first_schema {
            ctx.first_schema = false
        }
        else {
            return Err(RainbowError::duplicate_declaration("schema"));
        }
        if let Some(v) = ast.object.get_string("theme") {
            self.theme = v
        }
        if let Some(v) = ast.object.get_string("variant") {
            self.variant = v
        }
        Ok(())
    }
    fn eval_meta(&mut self, ast: MetaStatement, ctx: &mut SchemaContext) -> Result<()> {
        self.custom.insert(ast.meta, Value::eval_object(ast.object, ctx)?);
        Ok(())
    }
}

impl Value {
    fn eval_object(o: RangedObject, ctx: &mut SchemaContext) -> Result<Self> {
        let mut out = BTreeMap::new();
        for (k, ranged) in o.inner {
            let v = match ranged {
                RangedValue::Null => Value::null(),
                RangedValue::String(v) => Value::string(v),
                RangedValue::Number(v) => Value::number(v),
                RangedValue::Boolean(v) => Value::boolean(v),
                RangedValue::Color(v) => Value::color(v),
                RangedValue::Array(v) => {
                    todo!()
                }
                RangedValue::Namespace(v) => Value::reference(v),
                RangedValue::Object(v) => todo!(),
            };
            out.insert(k, v);
        }
        return todo!();
    }
}

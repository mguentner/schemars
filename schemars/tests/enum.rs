mod util;
use schemars::{schema_for, JsonSchema, Map};
use util::*;

// Ensure that schemars_derive uses the full path to std::string::String
pub struct String;

#[derive(JsonSchema)]
struct UnitStruct;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct Struct {
    foo: i32,
    bar: bool,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
enum External {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    UnitTwo,
    Tuple(i32, bool),
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_external_tag() -> TestResult {
    test_default_generated_schema::<External>("enum-external")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(tag = "typeProperty")]
enum Internal {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    UnitTwo,
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_internal_tag() -> TestResult {
    test_default_generated_schema::<Internal>("enum-internal")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(untagged)]
enum Untagged {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    Tuple(i32, bool),
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_untagged() -> TestResult {
    test_default_generated_schema::<Untagged>("enum-untagged")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(tag = "t", content = "c")]
enum Adjacent {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    Tuple(i32, bool),
    UnitTwo,
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_adjacent_tagged() -> TestResult {
    test_default_generated_schema::<Adjacent>("enum-adjacent-tagged")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(tag = "typeProperty")]
enum SimpleInternal {
    A,
    B,
    C,
}

#[test]
fn enum_simple_internal_tag() -> TestResult {
    test_default_generated_schema::<SimpleInternal>("enum-simple-internal")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum SoundOfMusic {
    /// # A deer
    ///
    /// A female deer
    Do,
    /// A drop of golden sun
    Re,
    /// A name I call myself
    Mi,
}

#[test]
fn enum_unit_with_doc_comments() -> TestResult {
    test_default_generated_schema::<SoundOfMusic>("enum-unit-doc")
}

#[derive(JsonSchema)]
enum NoVariants {}

#[test]
fn enum_no_variants() -> TestResult {
    test_default_generated_schema::<NoVariants>("no-variants")
}

#[derive(JsonSchema)]
#[serde(rename_all_fields = "PascalCase")]
pub enum RenameAllFields {
    First {
        nested_attribute: std::string::String,
    },
}

#[derive(JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RenameAll {
    First { nested_attribute: bool },
}

#[derive(JsonSchema)]
pub enum RenameAttribute {
    First {
        #[serde(rename = "RenamedAttribute")]
        nested_attribute: std::string::String,
    },
}

#[test]
fn enum_unit_rename_attribute() -> TestResult {
    test_default_generated_schema::<RenameAttribute>("enum-rename-attr")
}

#[test]
fn enum_unit_rename_all_fields() -> TestResult {
    test_default_generated_schema::<RenameAllFields>("enum-rename-all-fields")
}

#[test]
fn enum_unit_rename_all() -> TestResult {
    let schema = schema_for!(RenameAll);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    test_default_generated_schema::<RenameAll>("enum-rename-all")
}

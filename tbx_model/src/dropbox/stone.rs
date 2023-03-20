//use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dropbox/stone.pest"]
pub struct StoneParser;

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::fs;
    use pest::iterators::Pairs;
    use pest::Parser;
    use crate::dropbox::stone::StoneParser;
    use crate::dropbox::stone::Rule;

    fn print_pairs(header: &str, level: usize, pairs: Pairs<Rule>) {
        for p in pairs {
            println!("{}{} {:?}", header, ">>".repeat(level), p);
            let inner = p.into_inner();
            if 0 < inner.clone().count() {
                print_pairs(header, level + 1, inner.clone());
                println!();
            }
        }
    }

    fn assert_assume_success(source: &str, rule: Rule) {
        match StoneParser::parse(rule, source) {
            Err(pe) => {
                println!(">>>> {:?} >>>>", rule);
                println!("File Path: {:?}", pe.path());
                println!("Position: {:?}", pe.line_col);
                println!("Location: {:?}", pe.location);
                println!("Source: {:?}", pe.source());
                println!("Line: {:?}", pe.line());
                println!("Variant: {:?}", pe.variant);
                println!("{:?}", pe);
                println!("<<<< {:?} <<<<", rule);
                println!();
                assert_eq!("", source);
            }
            Ok(_p) => {
                //print_pairs(format!("{:?}", rule.type_id()).as_str(), 0, p);
            }
        }
    }

    fn assert_assume_equal(source: &str, rule: Rule) {
        match StoneParser::parse(rule, source) {
            Ok(r) => {
                assert_eq!(source, r.as_str())
            }
            Err(pe) => {
                println!(">>>> {:?} >>>>", rule);
                println!("File Path: {:?}", pe.path());
                println!("Position: {:?}", pe.line_col);
                println!("Location: {:?}", pe.location);
                println!("Source: {:?}", pe.source());
                println!("Line: {:?}", pe.line());
                println!("Variant: {:?}", pe.variant);
                println!("{:?}", pe);
                println!("<<<< {:?} <<<<", rule);
                println!();
                assert_eq!("", source);
            }
        }
    }

    #[test]
    fn test_keywords() {
        assert_assume_success("route", Rule::SYNTAX_KEYWORDS);
        assert_assume_success("String", Rule::SYNTAX_KEYWORDS);
    }

    #[test]
    fn test_identity() {
        assert_eq!("abc123", StoneParser::parse(Rule::identity, "abc123").unwrap().as_str());
        assert_eq!("common.abc123", StoneParser::parse(Rule::identity_ref, "common.abc123").unwrap().as_str());
        assert_eq!("abc123", StoneParser::parse(Rule::identity_ref, "abc123").unwrap().as_str());
        assert_assume_success("route_access_denied", Rule::identity);
        assert_assume_success("features/get_values", Rule::identity_route);

        assert_eq!("expected identity", StoneParser::parse(Rule::identity, "123abc").unwrap_err().variant.message());

        // reserved keywords
        assert_eq!("expected identity", StoneParser::parse(Rule::identity, "123abc").unwrap_err().variant.message());
    }

    #[test]
    fn test_version() {
        assert_eq!("123", StoneParser::parse(Rule::version, "123").unwrap().as_str());

        assert_eq!("expected version", StoneParser::parse(Rule::version, "0123").unwrap_err().variant.message());
    }

    #[test]
    fn test_primitive_literal() {
        assert_eq!("true", StoneParser::parse(Rule::literal_bool, "true").unwrap().as_str());
        assert_eq!("false", StoneParser::parse(Rule::literal_bool, "false").unwrap().as_str());

        assert_eq!("123", StoneParser::parse(Rule::literal_int, "123").unwrap().as_str());
        assert_eq!("0123", StoneParser::parse(Rule::literal_int, "0123").unwrap().as_str());
        assert_eq!("-123", StoneParser::parse(Rule::literal_int, "-123").unwrap().as_str());

        assert_eq!("123E10", StoneParser::parse(Rule::literal_float, "123E10").unwrap().as_str());

        assert_eq!("\"Hello\"", StoneParser::parse(Rule::literal_string, "\"Hello\"").unwrap().as_str());
        assert_assume_success(r#""[A-Za-z0-9\_]+""#, Rule::literal_string);

        assert_eq!("true", StoneParser::parse(Rule::literal, "true").unwrap().as_str());
        assert_eq!("false", StoneParser::parse(Rule::literal, "false").unwrap().as_str());
        assert_eq!("123", StoneParser::parse(Rule::literal, "123").unwrap().as_str());
        assert_eq!("-123", StoneParser::parse(Rule::literal, "-123").unwrap().as_str());
        assert_eq!("123.10", StoneParser::parse(Rule::literal, "123.10").unwrap().as_str());
        assert_eq!("\"Hello\"", StoneParser::parse(Rule::literal, "\"Hello\"").unwrap().as_str());
    }

    #[test]
    fn test_ref() {
        assert_eq!("PhotoSourceArg", StoneParser::parse(Rule::type_all, "PhotoSourceArg").unwrap().as_str());
        assert_eq!("account.PhotoSourceArg", StoneParser::parse(Rule::type_all, "account.PhotoSourceArg").unwrap().as_str());
    }

    #[test]
    fn test_type() {
        // primitive types
        assert_assume_equal("Bytes", Rule::type_all);
        assert_assume_equal("Boolean", Rule::type_all);
        assert_assume_equal("Float32", Rule::type_all);
        assert_assume_equal("Float64", Rule::type_all);
        assert_assume_equal("Int32", Rule::type_all);
        assert_assume_equal("Int64", Rule::type_all);
        assert_assume_equal("UInt32", Rule::type_all);
        assert_assume_equal("UInt64", Rule::type_all);
        assert_assume_equal("String", Rule::type_all);
        assert_assume_equal("Timestamp", Rule::type_all);
        assert_assume_equal("Timestamp?", Rule::type_all_optional);

        // complex types
        assert_assume_equal("List(Boolean)", Rule::type_all);
        assert_assume_equal("List( String)", Rule::type_all);
        assert_assume_equal("List( Photo)", Rule::type_all);
        assert_assume_equal("List( Photo)?", Rule::type_all_optional);

        // identity ref
        assert_assume_equal("Photo", Rule::type_all);
        assert_assume_equal("Photo?", Rule::type_all_optional);
        assert_assume_equal("common.Photo", Rule::type_all);
        assert_assume_equal("common.Photo?", Rule::type_all_optional);
    }

    #[test]
    fn test_union_tag() {
        assert_assume_equal("photo Photo\n  \"Photo data\"\n", Rule::spec_union_tag);
    }

    #[test]
    fn test_type_primitive() {
        assert_assume_success(r#"String(pattern="(/(.|[\\r\\n])*)?|(ns:[0-9]+(/.*)?)")"#, Rule::type_primitive)
    }

    #[test]
    fn test_struct() {
        assert_assume_success("Boolean", Rule::type_all);
        assert_assume_success("abc Boolean\n", Rule::spec_struct_field);
        assert_assume_success("abc Boolean\n", Rule::spec_struct_field);
        assert_assume_success("PhotoSourceArg", Rule::type_all);
        assert_assume_success("photo PhotoSourceArg\n", Rule::spec_struct_field);

        let s = "struct SetProfilePhotoArg\n    photo PhotoSourceArg\n        \"Image to set as the user's new profile photo.\"\n\n    example default\n        photo = default\n        ";
        assert_assume_success(s, Rule::spec_struct);
    }

    #[test]
    fn test_struct_special() {
        let r = r#"struct GetAccountArg
    account_id users_common.AccountId
        "A user's account identifier."

    example default
        account_id = "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc"
    "#;
        assert_assume_success(r, Rule::spec_struct)
    }

    #[test]
    fn test_alias_special() {
        let src = r#"alias FileRequestValidationError = String?
        "#;

        assert_assume_success(src, Rule::spec_definition);
    }

    #[test]
    fn test_union_special() {
        let src = r#"union RelocationBatchLaunch extends async.LaunchResultBase

    "Result returned by :route:`copy_batch` or :route:`move_batch` that may either launch an
    asynchronous job or complete synchronously."

    complete RelocationBatchResult

    example complete
        complete = default

    example async_job_id
        async_job_id = "34g93hh34h04y384084"
        "#;
        assert_assume_success(src, Rule::spec_union);
    }

    #[test]
    fn test_route() {
        let r = r#"route devices/revoke_device_session_batch(RevokeDeviceSessionBatchArg, RevokeDeviceSessionBatchResult, RevokeDeviceSessionBatchError)
    "Revoke a list of device sessions of team members."

    attrs
        auth = "team"
        scope = "sessions.modify"
        "#;

        match StoneParser::parse(Rule::spec_route, r) {
            Ok(p) => {
                print_pairs("route", 0, p);
            }
            _ => {}
        }
    }

    #[test]
    fn test_stone() {
        let entries = fs::read_dir("resources/dropbox/api_spec").unwrap();
        for entry in entries {
            let e = entry.unwrap();
            if e.file_name().clone().as_os_str().to_str().unwrap().ends_with(".stone") {
                println!("Parsing file: {}", e.path().display());
                let source = fs::read_to_string(e.path()).unwrap();
                assert_assume_success(source.as_str(), Rule::spec);
            }
        }
    }
}

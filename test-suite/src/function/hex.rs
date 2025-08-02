use {
    crate::*,
    gluesql_core::{
        error::{EvaluateError, TranslateError},
        prelude::Value::*,
    },
};

test_case!(hex, {
    let g = get_tester!();

    let test_cases = [
        (
            "SELECT HEX('GlueSQL') AS hex",
            Ok(select!(
                hex
                Str;
                "476C756553514C".to_string()
            )),
        ),
        (
            "SELECT HEX(10) AS hex",
            Ok(select!(
                hex
                Str;
                "A".to_string()
            )),
        ),
        (
            "SELECT HEX(10.5) AS hex",
            Ok(select!(
                hex
                Str;
                "A".to_string()
            )),
        ),
        (
            "SELECT HEX(255.0) AS hex",
            Ok(select!(
                hex
                Str;
                "FF".to_string()
            )),
        ),
        (
            "SELECT HEX(123.456) AS hex",
            Ok(select!(
                hex
                Str;
                "7B".to_string()
            )),
        ),
        ("SELECT HEX(NULL) AS hex", Ok(select_with_null!(hex; Null))),
        (
            "SELECT HEX(0) AS hex",
            Ok(select!(
                hex
                Str;
                "0".to_string()
            )),
        ),
        (
            "SELECT HEX(-1) AS hex",
            Ok(select!(
                hex
                Str;
                "FFFFFFFFFFFFFFFF".to_string()
            )),
        ),
        (
            "SELECT HEX(65535) AS hex",
            Ok(select!(
                hex
                Str;
                "FFFF".to_string()
            )),
        ),
        (
            "SELECT HEX() AS hex",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "HEX".to_owned(),
                expected: 1,
                found: 0,
            }
            .into()),
        ),
        (
            "SELECT HEX([1, 2, 3]) AS hex",
            Err(EvaluateError::FunctionRequiresStringValue("HEX".to_owned()).into()),
        ),
        (
            "SELECT HEX(TRUE) AS hex",
            Err(EvaluateError::FunctionRequiresStringValue("HEX".to_owned()).into()),
        ),
        (
            "SELECT HEX(FALSE) AS hex",
            Err(EvaluateError::FunctionRequiresStringValue("HEX".to_owned()).into()),
        ),
    ];

    for (sql, expected) in test_cases {
        g.test(sql, expected).await;
    }
});

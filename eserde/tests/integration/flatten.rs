use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::flatten::*;
use eserde_test_helper::test;

#[test]
fn flattened_struct() {
    let test = test!(serialized; Deep1,
        r#"{"f":0.10925001,"b":false,"s":"oJ654XjRD","v":[293213948,-698189839,-1145449804,100811548,2092713542,-162305236,1550235965]}"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        Deep1 {
            f: 0.10925001,
            deep2: Deep2 {
                b: false,
                deep3: Some(
                    Deep3 {
                        s: "oJ654XjRD",
                    },
                ),
            },
            v: [
                293213948,
                -698189839,
                -1145449804,
                100811548,
                2092713542,
                -162305236,
                1550235965,
            ],
        },
    )
    "#);
}

#[test]
fn flattened_value() {
    let test = test!(serialized; FlattenValue,
        r#"{"flag":true,"rh6rGx64R5z8bXM53JB":null,"tKCHK":0.45199126013011237,"zRNzOrvBGBkR":-942429839.0,"zvPAta":-1473060449.0}"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        FlattenValue {
            flag: true,
            value: JsonValue(
                Object {
                    "rh6rGx64R5z8bXM53JB": Null,
                    "tKCHK": Number(0.4519912601301124),
                    "zRNzOrvBGBkR": Number(-942429839.0),
                    "zvPAta": Number(-1473060449.0),
                },
            ),
        },
    )
    "#);
}

#[test]
fn flattened_map() {
    let test = test!(serialized; FlattenMap,
        r#"{"flag":false,"0Zdvgwzntg1":null,"2xmy0O":-201987135.0,"815o0wQYoQXKwQ4sk35":{"c5OSxU6Sg1RkLzA3LI":null,"cGlIGCxZqgKd":true,"m0BahDL9nqdE":null},"CWli7oV":false,"RPpRcK3uDfcPMcMMSl":-1046786077.0,"TtDt8QISh":null,"UeOlw3LZgVq01vyUfeJ":"SYS4VXt0E9Baiqqx0","VnVAc8m2IzWxHqUyE":"yIyXTZG"}"#
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        FlattenMap {
            flag: false,
            value: {
                "0Zdvgwzntg1": JsonValue(
                    Null,
                ),
                "2xmy0O": JsonValue(
                    Number(-201987135.0),
                ),
                "815o0wQYoQXKwQ4sk35": JsonValue(
                    Object {
                        "c5OSxU6Sg1RkLzA3LI": Null,
                        "cGlIGCxZqgKd": Bool(true),
                        "m0BahDL9nqdE": Null,
                    },
                ),
                "CWli7oV": JsonValue(
                    Bool(false),
                ),
                "RPpRcK3uDfcPMcMMSl": JsonValue(
                    Number(-1046786077.0),
                ),
                "TtDt8QISh": JsonValue(
                    Null,
                ),
                "UeOlw3LZgVq01vyUfeJ": JsonValue(
                    String("SYS4VXt0E9Baiqqx0"),
                ),
                "VnVAc8m2IzWxHqUyE": JsonValue(
                    String("yIyXTZG"),
                ),
            },
        },
    )
    "#);
}

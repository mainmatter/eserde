use eserde_test_helper::assert_from_json_inline;
use eserde_test_helper::enums_flattened::*;
use eserde_test_helper::test_helper::TestHelper;

#[test]
fn enums_flattened() {
    let test = TestHelper::<Container>::new_serialized(
        r#"{"f":0.6193613,"S":"iq4m5jByT","U":2413626873,"S2":"5B6FXFBEm","U2":2852593204,"S3":"HzE1mKd6Gv6L6DX"}"#,
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        Container {
            f: 0.6193613,
            e1: S(
                "iq4m5jByT",
            ),
            e2: U(
                2413626873,
            ),
            e3: S2(
                "5B6FXFBEm",
            ),
            e4: U2(
                2852593204,
            ),
            e5: S3(
                "HzE1mKd6Gv6L6DX",
            ),
        },
    )
    "#);
}

#[test]
fn enums_flattened_deny_unknown_fields() {
    let test = TestHelper::<ContainerDenyUnknownFields>::new_serialized(
        r#"{"f":0.6193613,"S":"iq4m5jByT","U":2413626873,"S2":"5B6FXFBEm","U2":2852593204,"S3":"HzE1mKd6Gv6L6DX"}"#,
    );
    assert_from_json_inline!(test, @r#"
    Ok(
        ContainerDenyUnknownFields {
            f: 0.6193613,
            e1: S(
                "iq4m5jByT",
            ),
            e2: U(
                2413626873,
            ),
            e3: S2(
                "5B6FXFBEm",
            ),
            e4: U2(
                2852593204,
            ),
            e5: S3(
                "HzE1mKd6Gv6L6DX",
            ),
        },
    )
    "#);
}

Structures and Enumerations that implement the [Real Estate Standards
Organization (RESO) Data Dictionary][reso].

The structures defined here can be serialized and deserialized using
[serde][serde].

```
use reso_dd;
use serde_json;

let reso = r#"{
  "StandardStatus": "Active"
}"#;

let property: reso_dd::Property = serde_json::from_str(reso)?;
assert_eq!(property.standard_status, Some(reso_dd::StandardStatus::Active));

println!("{}", serde_json::to_string(&property)?);
```

[reso]: https://ddwiki.reso.org/display/DDW17/RESO+Data+Dictionary+Wiki+v1.7
[serde]: https://serde.rs/

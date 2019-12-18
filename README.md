Structures and Enumerations that implement the [Real Estate Standards
Organization (RESO) Data Dictionary][reso].

[reso]: https://ddwiki.reso.org/display/DDW17/RESO+Data+Dictionary+Wiki+v1.7


## Generation

This project includes both the `reso_dd` project, and a project that can
generate `reso_dd` from an XML file, called `reso_dd_generator`.

To regenerated `reso_dd` (perhaps after changing `reso_dd_generator`), run

```
cargo run -p reso_dd_generator
```

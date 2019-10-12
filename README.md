# UUID Generator library

## Examples

```rust
use rs_uuid::{uuid8, uuid16, uuid32};

let id_8byte = uuid8();
let id_16byte = uuid16();
let id_32byte = uuid32();

// a94b-d2-9f-83-020290
println!("{}", id_8byte);

// 22f4b97a-94b4-37bf-0993-462840d1c3e3
println!("{}", id_16byte);

// e8893dbded63452b-9a764159-5d9128eb-e04ffb33-f1be0b8a79a15d119fc511f9
println!("{}", id_32byte);
```

ISO Standard UUID generators

```rust
use rs_uuid::iso::uuid_v4;

let iso_v4_uuid = uuid_v4();

// 22f4b97a-94b4-47bf-0993-462840d1c3e3
println!("{}", iso_v4_uuid);
```

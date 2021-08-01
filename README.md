# wallet-pass

[![Latest version](https://img.shields.io/crates/v/wallet-pass.svg)](https://crates.io/crates/wallet-pass)

Build and sign passes for apple wallet

## Sign an exisiting pass

```bash
cargo build --release --bin signpass

./target/release/signpass --help
```

## Create a custom pass

```rust
use std::path::Path;
use wallet_pass::{
    template::{Details, Field, Barcode, BarcodeFormat},
    Pass,
};

fn main() {
    // Load pass template
    let mut pass = Pass::from_path(Path::new("./StoreCard.pass")).unwrap();

    // Set general attributes
    pass.pass_type_identifier("pass.com.store.generic");
    pass.team_identifier("ASDF1234ASDF");

    // Set user specific attributes
    pass.serial_number("1234567890");
    pass.authentication_token("sda8f6ffDFS798SFDfsfSdf");

    pass.barcode(Barcode::new(BarcodeFormat::PkBarcodeFormatQr, "QR Code", "iso-8859-1"));

    let mut store_card = Details::new();

    let mut field = Field::new_f64("balance", 13.37);
    field.label("balance");
    field.currency_code("EUR");
    store_card.add_primary_field(field);

    let mut field = Field::new_string("account_name", "Max Mustermann");
    field.label("account_name");
    store_card.add_secondary_field(field);

    pass.store_card(store_card);

    // Sign, comprass and save pass 
    pass.export_to_file(
        Path::new("./StoreCard.pkpass"),
        Path::new("Certificates.p12"),
        "Certificates Password",
        Path::new("Apple Worldwide Developer Relations Certification Authority.pem"),
    )
    .unwrap();
}
```
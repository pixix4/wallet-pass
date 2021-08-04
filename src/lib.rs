#![warn(missing_docs)]

//! # wallet-pass
//! ```no_run
//! use std::path::Path;
//! use wallet_pass::{
//!     template::{Details, Field, Barcode, BarcodeFormat},
//!     Pass,
//! };
//!
//! // Load pass template
//! let mut pass = Pass::from_path(Path::new("./StoreCard.pass")).unwrap();
//!
//! // Set general attributes
//! pass.pass_type_identifier("pass.com.store.generic");
//! pass.team_identifier("ASDF1234ASDF");
//!
//! // Set user specific attributes
//! pass.serial_number("1234567890");
//! pass.authentication_token("sda8f6ffDFS798SFDfsfSdf");
//!
//! pass.barcode(Barcode::new(BarcodeFormat::PkBarcodeFormatQr, "QR Code", "iso-8859-1"));
//!
//! let mut store_card = Details::new();
//!
//! let mut field = Field::new_f64("balance", 13.37);
//! field.label("balance");
//! field.currency_code("EUR");
//! store_card.add_primary_field(field);
//!
//! let mut field = Field::new_string("account_name", "Max Mustermann");
//! field.label("account_name");
//! store_card.add_secondary_field(field);
//!
//! pass.store_card(store_card);
//!
//! // Sign, comprass and save pass
//! pass.export_to_file(
//!     Path::new("Certificates.p12"),
//!     "Certificates Password",
//!     Path::new("Apple Worldwide Developer Relations Certification Authority.pem"),
//!     Path::new("./StoreCard.pkpass"),
//! )
//! .unwrap();  
//! ```

mod pass;
pub use pass::Pass;

/// Sign an package of passes
pub mod sign;

/// Json template of passes
pub mod template;

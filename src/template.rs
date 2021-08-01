use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Apple Wallet pass with localizations, NFC and web service push updates support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Template {
    /// A URL to be passed to the associated app when launching it. The app receives this URL in
    /// the application:didFinishLaunchingWithOptions: and application:openURL:options: methods
    /// of its app delegate.
    #[serde(rename = "appLaunchURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_launch_url: Option<String>,

    /// A list of iTunes Store item identifiers for the associated apps.
    /// Only one item in the list is used—the first item identifier for an app compatible with
    /// the current device. If the app is not installed, the link opens the App Store and shows
    /// the app. If the app is already installed, the link launches the app.
    #[serde(rename = "associatedStoreIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_store_identifiers: Option<Vec<f64>>,

    /// The authentication token to use with the web service.
    #[serde(rename = "authenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,

    /// Background color of the pass, specified as an CSS-style RGB triple.
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

    /// Information specific to the pass’s barcode.
    /// Deprecated in iOS 9.0 and later; use barcodes instead.
    #[serde(rename = "barcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<Barcode>,

    /// Information specific to the pass’s barcode. The system uses the first valid barcode
    /// dictionary in the array. Additional dictionaries can be added as fallbacks.
    /// Available only in iOS 9.0 and later.
    #[serde(rename = "barcodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcodes: Option<Vec<Barcode>>,

    /// Beacons marking locations where the pass is relevant.
    /// Available in iOS 7.0.
    #[serde(rename = "beacons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacons: Option<Vec<Beacon>>,

    /// Information specific to a boarding pass.
    #[serde(rename = "boardingPass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_pass: Option<BoardingPass>,

    /// Information specific to a coupon.
    #[serde(rename = "coupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Details>,

    /// Brief description of the pass, used by the iOS accessibility technologies.
    /// Don’t try to include all of the data on the pass in its description, just include enough
    /// detail to distinguish passes of the same type.
    /// Localizable.
    #[serde(rename = "description")]
    pub description: String,

    /// Information specific to an event ticket.
    #[serde(rename = "eventTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticket: Option<Details>,

    /// Date and time when the pass expires.
    /// Available in iOS 7.0.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,

    /// Foreground color of the pass, specified as a CSS-style RGB triple
    #[serde(rename = "foregroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,

    /// Version of the file format.
    #[serde(rename = "formatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<serde_json::Value>,

    /// Information specific to a generic pass.
    #[serde(rename = "generic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic: Option<Details>,

    /// Identifier used to group related passes. If a grouping identifier is specified, passes
    /// with the same style, pass type identifier, and grouping identifier are displayed as a
    /// group. Otherwise, passes are grouped automatically.
    /// Use this to group passes that are tightly related, such as the boarding passes for
    /// different connections of the same trip.
    /// Available in iOS 7.0.
    #[serde(rename = "groupingIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_identifier: Option<String>,

    /// olor of the label text, specified as a CSS-style RGB triple.
    /// If omitted, the label color is determined automatically.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,

    /// Locations where the pass is relevant. For example, the location of your store.
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,

    /// Text displayed next to the logo on the pass.
    /// Localizable.
    #[serde(rename = "logoText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_text: Option<String>,

    /// Maximum distance in meters from a relevant latitude and longitude that the pass is
    /// relevant. This number is compared to the pass’s default distance and the smaller value is
    /// used.
    /// Available in iOS 7.0.
    #[serde(rename = "maxDistance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,

    /// Information used for Value Added Service Protocol transactions.
    /// Available in iOS 9.0.
    #[serde(rename = "nfc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc: Option<Nfc>,

    /// Display name of the organization that originated and signed the pass.
    /// Localizable.
    #[serde(rename = "organizationName")]
    pub organization_name: String,

    /// Pass type identifier, as issued by Apple. The value must correspond with your signing
    /// certificate.
    #[serde(rename = "passTypeIdentifier")]
    pub pass_type_identifier: String,

    /// Date and time when the pass becomes relevant. For example, the start time of a movie.
    /// Recommended for event tickets and boarding passes.
    #[serde(rename = "relevantDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_date: Option<String>,

    /// Serial number that uniquely identifies the pass. No two passes with the same pass type
    /// identifier may have the same serial number.
    #[serde(rename = "serialNumber")]
    pub serial_number: String,

    /// Information specific to a store card.
    #[serde(rename = "storeCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_card: Option<Details>,

    /// If true, the strip image is displayed without a shine effect. The default value prior to
    /// iOS 7.0 is false. In iOS 7.0, a shine effect is never applied, and this key is deprecated.
    #[serde(rename = "suppressStripShine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_strip_shine: Option<bool>,

    /// Team identifier of the organization that originated and signed the pass, as issued by
    /// Apple.
    #[serde(rename = "teamIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_identifier: Option<String>,

    /// Custom information for companion apps. This data is not displayed to the user.
    /// For example, a pass for a cafe could include information about the user’s favorite drink
    /// and sandwich in a machine-readable form for the companion app to read, making it easy to
    /// place an order for “the usual” from the app.
    /// Available in iOS 7.0.
    #[serde(rename = "userInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<HashMap<String, Option<serde_json::Value>>>,

    /// Indicates that the pass is void—for example, a one time use coupon that has been
    /// redeemed.
    /// Available in iOS 7.0.
    #[serde(rename = "voided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided: Option<bool>,

    /// The URL of a web service that conforms to the API described in PassKit Web Service
    /// Reference. The web service must use the HTTPS protocol; the leading https:// is included
    /// in the value of this key. On devices configured for development, there is UI in Settings
    /// to allow HTTP web services.
    #[serde(rename = "webServiceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_service_url: Option<String>,
}

impl Template {
    /// Create a new Instance
    pub fn new(
        description: &str,
        organization_name: &str,
        pass_type_identifier: &str,
        serial_number: &str,
    ) -> Self {
        Self {
            app_launch_url: None,
            associated_store_identifiers: None,
            authentication_token: None,
            background_color: None,
            barcode: None,
            barcodes: None,
            beacons: None,
            boarding_pass: None,
            coupon: None,
            description: description.into(),
            event_ticket: None,
            expiration_date: None,
            foreground_color: None,
            format_version: None,
            generic: None,
            grouping_identifier: None,
            label_color: None,
            locations: None,
            logo_text: None,
            max_distance: None,
            nfc: None,
            organization_name: organization_name.into(),
            pass_type_identifier: pass_type_identifier.into(),
            relevant_date: None,
            serial_number: serial_number.into(),
            store_card: None,
            suppress_strip_shine: None,
            team_identifier: None,
            user_info: None,
            voided: None,
            web_service_url: None,
        }
    }

    /// A URL to be passed to the associated app when launching it. The app receives this URL in
    /// the application:didFinishLaunchingWithOptions: and application:openURL:options: methods
    /// of its app delegate.
    pub fn app_launch_url(&mut self, app_launch_url: &str) {
        self.app_launch_url = Some(app_launch_url.into());
    }

    /// A list of iTunes Store item identifiers for the associated apps.
    /// Only one item in the list is used—the first item identifier for an app compatible with
    /// the current device. If the app is not installed, the link opens the App Store and shows
    /// the app. If the app is already installed, the link launches the app.
    pub fn add_associated_store_identifiers(&mut self, associated_store_identifier: f64) {
        let mut vec = match &self.associated_store_identifiers {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };

        vec.push(associated_store_identifier);
        self.associated_store_identifiers = Some(vec);
    }

    /// A list of iTunes Store item identifiers for the associated apps.
    /// Only one item in the list is used—the first item identifier for an app compatible with
    /// the current device. If the app is not installed, the link opens the App Store and shows
    /// the app. If the app is already installed, the link launches the app.
    pub fn clear_associated_store_identifiers(&mut self) {
        self.associated_store_identifiers = None;
    }

    /// The authentication token to use with the web service.
    pub fn authentication_token(&mut self, authentication_token: &str) {
        self.authentication_token = Some(authentication_token.into());
    }

    /// Background color of the pass, specified as an CSS-style RGB triple.
    pub fn background_color(&mut self, background_color: &str) {
        self.background_color = Some(background_color.into());
    }

    /// Information specific to the pass’s barcode.
    /// Deprecated in iOS 9.0 and later; use barcodes instead.
    pub fn barcode(&mut self, barcode: Barcode) {
        self.barcode = Some(barcode);
    }

    /// Information specific to the pass’s barcode. The system uses the first valid barcode
    /// dictionary in the array. Additional dictionaries can be added as fallbacks.
    /// Available only in iOS 9.0 and later.
    pub fn add_barcodes(&mut self, barcode: Barcode) {
        let mut vec = match &self.barcodes {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };

        vec.push(barcode);
        self.barcodes = Some(vec);
    }

    /// Information specific to the pass’s barcode. The system uses the first valid barcode
    /// dictionary in the array. Additional dictionaries can be added as fallbacks.
    /// Available only in iOS 9.0 and later.
    pub fn clear_barcodes(&mut self) {
        self.barcodes = None;
    }

    /// Beacons marking locations where the pass is relevant.
    /// Available in iOS 7.0.
    pub fn add_beacon(&mut self, beacon: Beacon) {
        let mut vec = match &self.beacons {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };

        vec.push(beacon);
        self.beacons = Some(vec);
    }

    /// Beacons marking locations where the pass is relevant.
    /// Available in iOS 7.0.
    pub fn clear_beacons(&mut self) {
        self.beacons = None;
    }

    /// Information specific to a boarding pass.
    pub fn boarding_pass(&mut self, boarding_pass: BoardingPass) {
        self.boarding_pass = Some(boarding_pass);
    }

    /// Information specific to a coupon.
    pub fn coupon(&mut self, coupon: Details) {
        self.coupon = Some(coupon);
    }

    /// Brief description of the pass, used by the iOS accessibility technologies.
    /// Don’t try to include all of the data on the pass in its description, just include enough
    /// detail to distinguish passes of the same type.
    /// Localizable.
    pub fn description(&mut self, description: &str) {
        self.description = description.into();
    }

    /// Information specific to an event ticket.
    pub fn event_ticket(&mut self, event_ticket: Details) {
        self.event_ticket = Some(event_ticket);
    }

    /// Date and time when the pass expires.
    /// Available in iOS 7.0.
    pub fn expiration_date(&mut self, expiration_date: &str) {
        self.expiration_date = Some(expiration_date.into());
    }

    /// Foreground color of the pass, specified as a CSS-style RGB triple
    pub fn foreground_color(&mut self, foreground_color: &str) {
        self.foreground_color = Some(foreground_color.into());
    }

    /// Version of the file format.
    pub fn format_version(&mut self, format_version: serde_json::Value) {
        self.format_version = Some(format_version);
    }

    /// Information specific to a generic pass.
    pub fn generic(&mut self, generic: Details) {
        self.generic = Some(generic);
    }

    /// Identifier used to group related passes. If a grouping identifier is specified, passes
    /// with the same style, pass type identifier, and grouping identifier are displayed as a
    /// group. Otherwise, passes are grouped automatically.
    /// Use this to group passes that are tightly related, such as the boarding passes for
    /// different connections of the same trip.
    /// Available in iOS 7.0.
    pub fn grouping_identifier(&mut self, grouping_identifier: &str) {
        self.grouping_identifier = Some(grouping_identifier.into());
    }

    /// olor of the label text, specified as a CSS-style RGB triple.
    /// If omitted, the label color is determined automatically.
    pub fn label_color(&mut self, label_color: &str) {
        self.label_color = Some(label_color.into());
    }

    /// Locations where the pass is relevant. For example, the location of your store.
    pub fn add_location(&mut self, location: Location) {
        let mut vec = match &self.locations {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };

        vec.push(location);
        self.locations = Some(vec);
    }

    /// Locations where the pass is relevant. For example, the location of your store.
    pub fn clear_locations(&mut self) {
        self.locations = None;
    }

    /// Text displayed next to the logo on the pass.
    /// Localizable.
    pub fn logo_text(&mut self, logo_text: &str) {
        self.logo_text = Some(logo_text.into());
    }

    /// Maximum distance in meters from a relevant latitude and longitude that the pass is
    /// relevant. This number is compared to the pass’s default distance and the smaller value is
    /// used.
    /// Available in iOS 7.0.
    pub fn max_distance(&mut self, max_distance: f64) {
        self.max_distance = Some(max_distance);
    }

    /// Information used for Value Added Service Protocol transactions.
    /// Available in iOS 9.0.
    pub fn nfc(&mut self, nfc: Nfc) {
        self.nfc = Some(nfc);
    }

    /// Display name of the organization that originated and signed the pass.
    /// Localizable.
    pub fn organization_name(&mut self, organization_name: &str) {
        self.organization_name = organization_name.into();
    }

    /// Pass type identifier, as issued by Apple. The value must correspond with your signing
    /// certificate.
    pub fn pass_type_identifier(&mut self, pass_type_identifier: &str) {
        self.pass_type_identifier = pass_type_identifier.into();
    }

    /// Date and time when the pass becomes relevant. For example, the start time of a movie.
    /// Recommended for event tickets and boarding passes.
    pub fn relevant_date(&mut self, relevant_date: &str) {
        self.relevant_date = Some(relevant_date.into());
    }

    /// Serial number that uniquely identifies the pass. No two passes with the same pass type
    /// identifier may have the same serial number.
    pub fn serial_number(&mut self, serial_number: &str) {
        self.serial_number = serial_number.into();
    }

    /// Information specific to a store card.
    pub fn store_card(&mut self, store_card: Details) {
        self.store_card = Some(store_card);
    }

    /// If true, the strip image is displayed without a shine effect. The default value prior to
    /// iOS 7.0 is false. In iOS 7.0, a shine effect is never applied, and this key is deprecated.
    pub fn suppress_strip_shine(&mut self, suppress_strip_shine: bool) {
        self.suppress_strip_shine = Some(suppress_strip_shine);
    }

    /// Team identifier of the organization that originated and signed the pass, as issued by
    /// Apple.
    pub fn team_identifier(&mut self, team_identifier: &str) {
        self.team_identifier = Some(team_identifier.into());
    }

    /// Custom information for companion apps. This data is not displayed to the user.
    /// For example, a pass for a cafe could include information about the user’s favorite drink
    /// and sandwich in a machine-readable form for the companion app to read, making it easy to
    /// place an order for “the usual” from the app.
    /// Available in iOS 7.0.
    pub fn add_user_info(&mut self, key: &str, value: serde_json::Value) {
        let mut map = match &self.user_info {
            Some(map) => map.clone(),
            None => HashMap::new(),
        };

        map.insert(key.into(), Some(value));
        self.user_info = Some(map);
    }

    /// Custom information for companion apps. This data is not displayed to the user.
    /// For example, a pass for a cafe could include information about the user’s favorite drink
    /// and sandwich in a machine-readable form for the companion app to read, making it easy to
    /// place an order for “the usual” from the app.
    /// Available in iOS 7.0.
    pub fn clear_user_info(&mut self) {
        self.user_info = None;
    }

    /// Indicates that the pass is void—for example, a one time use coupon that has been
    /// redeemed.
    /// Available in iOS 7.0.
    pub fn voided(&mut self, voided: bool) {
        self.voided = Some(voided);
    }

    /// The URL of a web service that conforms to the API described in PassKit Web Service
    /// Reference. The web service must use the HTTPS protocol; the leading https:// is included
    /// in the value of this key. On devices configured for development, there is UI in Settings
    /// to allow HTTP web services.
    pub fn web_service_url(&mut self, web_service_url: &str) {
        self.web_service_url = Some(web_service_url.into());
    }
}

/// Information specific to the pass’s barcode.
/// Deprecated in iOS 9.0 and later; use barcodes instead.
///
/// Information about a pass’s barcode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barcode {
    /// Text displayed near the barcode. For example, a human-readable version of the barcode
    /// data in case the barcode doesn’t scan.
    #[serde(rename = "altText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,

    /// Barcode format. PKBarcodeFormatCode128 may only be used for dictionaries in the barcodes
    /// array.
    #[serde(rename = "format")]
    pub format: BarcodeFormat,

    /// Message or payload to be displayed as a barcode.
    #[serde(rename = "message")]
    pub message: String,

    /// Text encoding that is used to convert the message from the string representation to a
    /// data representation to render the barcode. The value is typically iso-8859-1, but you may
    /// use another encoding that is supported by your barcode scanning infrastructure.
    #[serde(rename = "messageEncoding")]
    pub message_encoding: String,
}

impl Barcode {
    /// Create a new Instance
    pub fn new(format: BarcodeFormat, message: &str, message_encoding: &str) -> Self {
        Self {
            alt_text: None,
            format,
            message: message.into(),
            message_encoding: message_encoding.into(),
        }
    }
}

/// Information about a location beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beacon {
    /// Major identifier of a Bluetooth Low Energy location beacon.
    #[serde(rename = "major")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,

    /// Minor identifier of a Bluetooth Low Energy location beacon.
    #[serde(rename = "minor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,

    /// Unique identifier of a Bluetooth Low Energy location beacon.
    #[serde(rename = "proximityUUID")]
    pub proximity_uuid: String,

    /// Text displayed on the lock screen when the pass is currently relevant.
    #[serde(rename = "relevantText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_text: Option<String>,
}

impl Beacon {
    /// Create a new Instance
    pub fn new(proximity_uuid: &str) -> Self {
        Self {
            major: None,
            minor: None,
            proximity_uuid: proximity_uuid.into(),
            relevant_text: None,
        }
    }
}

/// Information specific to a boarding pass.
///
/// Keys that define the structure of the pass.
/// These keys are used for all pass styles and partition the fields into the various parts
/// of the pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardingPass {
    /// Type of transit.
    #[serde(rename = "transitType")]
    pub transit_type: TransitType,

    /// Additional fields to be displayed on the front of the pass.
    #[serde(rename = "auxiliaryFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_fields: Option<Vec<Field>>,

    /// Fields to be on the back of the pass.
    #[serde(rename = "backFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_fields: Option<Vec<Field>>,

    /// Fields to be displayed in the header on the front of the pass.Use header fields
    /// sparingly; unlike all other fields, they remain visible when a stack of passes are
    /// displayed.
    #[serde(rename = "headerFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_fields: Option<Vec<Field>>,

    /// Fields to be displayed prominently on the front of the pass.
    #[serde(rename = "primaryFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_fields: Option<Vec<Field>>,

    /// Fields to be displayed on the front of the pass.
    #[serde(rename = "secondaryFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_fields: Option<Vec<Field>>,
}

impl BoardingPass {
    /// Create a new Instance
    pub fn new(transit_type: TransitType) -> Self {
        Self {
            transit_type,
            auxiliary_fields: None,
            back_fields: None,
            header_fields: None,
            primary_fields: None,
            secondary_fields: None,
        }
    }
}

/// Keys that define an individual field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Attributed value of the field.
    /// The value may contain HTML markup for links. Only the <a> tag and its href attribute are
    /// supported. This key’s value overrides the text specified by the value key.
    /// Available in iOS 7.0.
    #[serde(rename = "attributedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_value: Option<ValueUnion>,

    /// Format string for the alert text that is displayed when the pass is updated. The format
    /// string must contain the escape %@, which is replaced with the field’s new value. If you
    /// don’t specify a change message, the user isn’t notified when the field changes.
    /// Localizable.
    #[serde(rename = "changeMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,

    /// Code of currency
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,

    /// Data detectors that are applied to the field’s value. Provide an empty array to use no
    /// data detectors. Data detectors are applied only to back fields.
    #[serde(rename = "dataDetectorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_detector_types: Option<Vec<serde_json::Value>>,

    /// Style of date to display.
    #[serde(rename = "dateStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_style: Option<EStyle>,

    /// Always display the time and date in the given time zone, not in the user’s current time
    /// zone.
    /// The format for a date and time always requires a time zone, even if it will be ignored.
    /// For backward compatibility with iOS 6, provide an appropriate time zone, so that the
    /// information is displayed meaningfully even without ignoring time zones.
    /// This key does not affect how relevance is calculated.
    /// Available in iOS 7.0.
    #[serde(rename = "ignoresTimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_time_zone: Option<bool>,

    /// If true, the label’s value is displayed as a relative date; otherwise, it is displayed as
    /// an absolute date.
    /// This key does not affect how relevance is calculated.
    #[serde(rename = "isRelative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relative: Option<bool>,

    /// The key must be unique within the scope of the entire pass.
    #[serde(rename = "key")]
    pub key: String,

    /// Label text for the field.
    /// Localizable.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Style of number to display. Number styles have the same meaning as the Cocoa number
    /// formatter styles with corresponding names. See
    /// https://developer.apple.com/documentation/foundation/nsnumberformatterstyle
    #[serde(rename = "numberStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_style: Option<NumberStyle>,

    /// Machine-readable metadata to allow the system to offer Wallet passes to users
    /// intelligently.
    #[serde(rename = "semantics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantics: Option<Semantics>,

    /// Alignment for the field’s contents.
    /// This key is not allowed for primary fields or back fields.
    #[serde(rename = "textAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<TextAlignment>,

    /// Style of time to display.
    #[serde(rename = "timeStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_style: Option<EStyle>,

    /// Value of the field
    #[serde(rename = "value")]
    pub value: ValueUnion,
}

impl Field {
    /// Create a new Instance with a String value
    pub fn new_string(key: &str, value: &str) -> Self {
        Self {
            attributed_value: None,
            change_message: None,
            currency_code: None,
            data_detector_types: None,
            date_style: None,
            ignores_time_zone: None,
            is_relative: None,
            key: key.into(),
            label: None,
            number_style: None,
            semantics: None,
            text_alignment: None,
            time_style: None,
            value: ValueUnion::String(value.into()),
        }
    }

    /// Create a new Instance with a Double value
    pub fn new_f64(key: &str, value: f64) -> Self {
        Self {
            attributed_value: None,
            change_message: None,
            currency_code: None,
            data_detector_types: None,
            date_style: None,
            ignores_time_zone: None,
            is_relative: None,
            key: key.into(),
            label: None,
            number_style: None,
            semantics: None,
            text_alignment: None,
            time_style: None,
            value: ValueUnion::Double(value),
        }
    }

    /// Attributed value of the field.
    /// The value may contain HTML markup for links. Only the <a> tag and its href attribute are
    /// supported. This key’s value overrides the text specified by the value key.
    /// Available in iOS 7.0.
    pub fn attributed_value(&mut self, attributed_value: ValueUnion) {
        self.attributed_value = Some(attributed_value);
    }

    /// Format string for the alert text that is displayed when the pass is updated. The format
    /// string must contain the escape %@, which is replaced with the field’s new value. If you
    /// don’t specify a change message, the user isn’t notified when the field changes.
    /// Localizable.
    pub fn change_message(&mut self, change_message: &str) {
        self.change_message = Some(change_message.into());
    }

    /// Code of currency
    pub fn currency_code(&mut self, currency_code: &str) {
        self.currency_code = Some(currency_code.into());
    }

    /// Data detectors that are applied to the field’s value. Provide an empty array to use no
    /// data detectors. Data detectors are applied only to back fields.
    pub fn clear_data_detector_types(&mut self) {
        self.data_detector_types = None;
    }

    /// Data detectors that are applied to the field’s value. Provide an empty array to use no
    /// data detectors. Data detectors are applied only to back fields.
    pub fn add_data_detector_type(&mut self, data_detector_type: serde_json::Value) {
        let mut vec = match &self.data_detector_types {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(data_detector_type);

        self.data_detector_types = Some(vec);
    }

    /// Style of date to display.
    pub fn date_style(&mut self, date_style: EStyle) {
        self.date_style = Some(date_style);
    }

    /// Always display the time and date in the given time zone, not in the user’s current time
    /// zone.
    /// The format for a date and time always requires a time zone, even if it will be ignored.
    /// For backward compatibility with iOS 6, provide an appropriate time zone, so that the
    /// information is displayed meaningfully even without ignoring time zones.
    /// This key does not affect how relevance is calculated.
    /// Available in iOS 7.0.
    pub fn ignores_time_zone(&mut self, ignores_time_zone: bool) {
        self.ignores_time_zone = Some(ignores_time_zone);
    }

    /// If true, the label’s value is displayed as a relative date; otherwise, it is displayed as
    /// an absolute date.
    /// This key does not affect how relevance is calculated.
    pub fn is_relative(&mut self, is_relative: bool) {
        self.is_relative = Some(is_relative);
    }

    /// Label text for the field.
    /// Localizable.
    pub fn label(&mut self, label: &str) {
        self.label = Some(label.into());
    }

    /// Style of number to display. Number styles have the same meaning as the Cocoa number
    /// formatter styles with corresponding names. See
    /// https://developer.apple.com/documentation/foundation/nsnumberformatterstyle
    pub fn number_style(&mut self, number_style: NumberStyle) {
        self.number_style = Some(number_style);
    }

    /// Machine-readable metadata to allow the system to offer Wallet passes to users
    /// intelligently.
    pub fn semantics(&mut self, semantics: Semantics) {
        self.semantics = Some(semantics);
    }

    /// Alignment for the field’s contents.
    /// This key is not allowed for primary fields or back fields.
    pub fn text_alignment(&mut self, text_alignment: TextAlignment) {
        self.text_alignment = Some(text_alignment);
    }

    /// Style of time to display.
    pub fn time_style(&mut self, time_style: EStyle) {
        self.time_style = Some(time_style);
    }
}

/// Machine-readable metadata to allow the system to offer Wallet passes to users
/// intelligently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Semantics {
    /// The IATA airline code, such as 'EX' for flightCode 'EX123'.
    #[serde(rename = "airlineCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airline_code: Option<String>,

    /// The Adam IDs for the artists performing, in decreasing order of significance.
    #[serde(rename = "artistIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_i_ds: Option<Vec<String>>,

    /// The unique abbreviation of the away team's name.
    #[serde(rename = "awayTeamAbbreviation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team_abbreviation: Option<String>,

    /// The home location of the away team.
    #[serde(rename = "awayTeamLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team_location: Option<String>,

    /// The name of the away team.
    #[serde(rename = "awayTeamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team_name: Option<String>,

    /// The balance redeemable with the pass.
    #[serde(rename = "balance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<CurrencyAmount>,

    /// A group number for boarding.
    #[serde(rename = "boardingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_group: Option<String>,

    /// A sequence number for boarding.
    #[serde(rename = "boardingSequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_sequence_number: Option<String>,

    /// The car number.
    #[serde(rename = "carNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_number: Option<String>,

    /// A booking or reservation confirmation number.
    #[serde(rename = "confirmationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,

    /// The updated date and time of arrival, if different than the original scheduled date.
    #[serde(rename = "currentArrivalDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_arrival_date: Option<String>,

    /// The updated date and time of boarding, if different than the original scheduled date.
    #[serde(rename = "currentBoardingDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_boarding_date: Option<String>,

    /// The updated date and time of departure, if different than the original scheduled date.
    #[serde(rename = "currentDepartureDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_departure_date: Option<String>,

    /// The IATA airport code for the departure airport.
    #[serde(rename = "departureAirportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,

    /// The full name of the departure airport
    #[serde(rename = "departureAirportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_name: Option<String>,

    /// The gate number or letters of the departure gate, such as '1A'. Do not include the word
    /// 'Gate'.
    #[serde(rename = "departureGate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_gate: Option<String>,

    /// The geographic coordinates of the transit departure, suitable to be shown on a map. If
    /// possible, precise locations are more useful to travelers, such as the specific location
    /// of the gate at an airport.
    #[serde(rename = "departureLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_location: Option<Location>,

    /// A brief description of the departure location.
    #[serde(rename = "departureLocationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_location_description: Option<String>,

    /// The name of the departure platform, such as 'A'. Do not include the word 'Platform'.
    #[serde(rename = "departurePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_platform: Option<String>,

    /// The name of the departure station.
    #[serde(rename = "departureStationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_station_name: Option<String>,

    /// The terminal name or letter of the departure terminal, such as 'A'. Do not include the
    /// word 'Terminal'
    #[serde(rename = "departureTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_terminal: Option<String>,

    /// The IATA airport code for the destination airport.
    #[serde(rename = "destinationAirportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_airport_code: Option<String>,

    /// The full name of the destination airport
    #[serde(rename = "destinationAirportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_airport_name: Option<String>,

    /// The gate number or letters of the destination gate, such as '1A'. Do not include the word
    /// 'Gate'.
    #[serde(rename = "destinationGate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_gate: Option<String>,

    /// The geographic coordinates of the transit destination, suitable to be shown on a map.
    #[serde(rename = "destinationLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_location: Option<Location>,

    /// A brief description of the destination location.
    #[serde(rename = "destinationLocationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_location_description: Option<Location>,

    /// The name of the destination platform, such as 'A'. Do not include the word 'Platform'.
    #[serde(rename = "destinationPlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_platform: Option<String>,

    /// The name of the destination station.
    #[serde(rename = "destinationStationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_station_name: Option<String>,

    /// The terminal name or letter of the destination terminal, such as 'A'. Do not include the
    /// word 'Terminal'
    #[serde(rename = "destinationTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_terminal: Option<String>,

    /// The duration of the event or transit journey, in seconds.
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// The date and time the event ends.
    #[serde(rename = "eventEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_end_date: Option<String>,

    /// The full name for the event, such as the title of a movie.
    #[serde(rename = "eventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    /// The date and time the event starts.
    #[serde(rename = "eventStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_date: Option<String>,

    /// The event type.
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,

    /// The IATA flight code
    #[serde(rename = "flightCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_code: Option<String>,

    /// The numeric portion of the IATA flightCode, such as 123 for flightCode 'EX123'
    #[serde(rename = "flightNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<f64>,

    /// The genre of the performance.
    #[serde(rename = "genre")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// The unique abbreviation of the home team's name.
    #[serde(rename = "homeTeamAbbreviation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_abbreviation: Option<String>,

    /// The home location of the home team.
    #[serde(rename = "homeTeamLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_location: Option<String>,

    /// The name of the home team.
    #[serde(rename = "homeTeamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_name: Option<String>,

    /// The abbreviated league name for a sporting event.
    #[serde(rename = "leagueAbbreviation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league_abbreviation: Option<String>,

    /// he unabbreviated league name for a sporting event.
    #[serde(rename = "leagueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league_name: Option<String>,

    /// The name of a frequent flyer or loyalty program.
    #[serde(rename = "membershipProgramName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_program_name: Option<String>,

    /// The ticketed passenger's frequent flyer or loyalty number.
    #[serde(rename = "membershipProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_program_number: Option<String>,

    /// The original scheduled date and time of arrival.
    #[serde(rename = "originalArrivalDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_arrival_date: Option<String>,

    /// The original scheduled date and time of boarding.
    #[serde(rename = "originalBoardingDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_boarding_date: Option<String>,

    /// The original scheduled date and time of departure.
    #[serde(rename = "originalDepartureDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_departure_date: Option<String>,

    /// The passenger's name.
    #[serde(rename = "passengerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<PersonNameComponents>,

    /// The full names of the performers and opening acts, in decreasing order of significance.
    #[serde(rename = "performerNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer_names: Option<Vec<String>>,

    /// he priority status held by the ticketed passenger
    #[serde(rename = "priorityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_status: Option<String>,

    /// Seating details for all seats at the event or transit journey.
    #[serde(rename = "seats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seats: Option<Vec<Seat>>,

    /// The type of security screening that the ticketed passenger will be subject to, such as
    /// 'Priority'.
    #[serde(rename = "securityScreening")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_screening: Option<String>,

    /// Request the user's device to remain silent during a the event or transit journey. This
    /// key may not be honored and the system will determine the length of the silence period.
    #[serde(rename = "silenceRequested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_requested: Option<bool>,

    /// The commonly used local name of the sport.
    #[serde(rename = "sportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sport_name: Option<String>,

    /// The total price for the pass.
    #[serde(rename = "totalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<CurrencyAmount>,

    /// The name of the transit company.
    #[serde(rename = "transitProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_provider: Option<String>,

    /// A brief description of the current status of the vessel being boarded. For delayed
    /// statuses, provide currentBoardingDate, currentDepartureDate, and currentArrivalDate where
    /// available.
    #[serde(rename = "transitStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_status: Option<String>,

    /// A brief description explaining the reason for the current transitStatus
    #[serde(rename = "transitStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_status_reason: Option<String>,

    /// The name of the vehicle being boarded, such as the name of a boat.
    #[serde(rename = "vehicleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_name: Option<String>,

    /// The identifier of the vehicle being boarded, such as the aircraft registration number or
    /// train number.
    #[serde(rename = "vehicleNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,

    /// A brief description of the type of vehicle being boarded, such as the model and
    /// manufacturer of a plane or the class of a boat.
    #[serde(rename = "vehicleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_type: Option<String>,

    /// The full name of the entrance to use to gain access to the ticketed event.
    #[serde(rename = "venueEntrance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_entrance: Option<String>,

    /// The geographic coordinates of the venue.
    #[serde(rename = "venueLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_location: Option<Location>,

    /// The full name of the venue.
    #[serde(rename = "venueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_name: Option<String>,

    /// The phone number for enquiries about the venue's ticketed event.
    #[serde(rename = "venuePhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_phone_number: Option<String>,

    /// The full name of the room where the ticketed event is taking place.
    #[serde(rename = "venueRoom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue_room: Option<String>,
}

impl Semantics {
    /// Create a new Instance
    pub fn new() -> Self {
        Self {
            airline_code: None,
            artist_i_ds: None,
            away_team_abbreviation: None,
            away_team_location: None,
            away_team_name: None,
            balance: None,
            boarding_group: None,
            boarding_sequence_number: None,
            car_number: None,
            confirmation_number: None,
            current_arrival_date: None,
            current_boarding_date: None,
            current_departure_date: None,
            departure_airport_code: None,
            departure_airport_name: None,
            departure_gate: None,
            departure_location: None,
            departure_location_description: None,
            departure_platform: None,
            departure_station_name: None,
            departure_terminal: None,
            destination_airport_code: None,
            destination_airport_name: None,
            destination_gate: None,
            destination_location: None,
            destination_location_description: None,
            destination_platform: None,
            destination_station_name: None,
            destination_terminal: None,
            duration: None,
            event_end_date: None,
            event_name: None,
            event_start_date: None,
            event_type: None,
            flight_code: None,
            flight_number: None,
            genre: None,
            home_team_abbreviation: None,
            home_team_location: None,
            home_team_name: None,
            league_abbreviation: None,
            league_name: None,
            membership_program_name: None,
            membership_program_number: None,
            original_arrival_date: None,
            original_boarding_date: None,
            original_departure_date: None,
            passenger_name: None,
            performer_names: None,
            priority_status: None,
            seats: None,
            security_screening: None,
            silence_requested: None,
            sport_name: None,
            total_price: None,
            transit_provider: None,
            transit_status: None,
            transit_status_reason: None,
            vehicle_name: None,
            vehicle_number: None,
            vehicle_type: None,
            venue_entrance: None,
            venue_location: None,
            venue_name: None,
            venue_phone_number: None,
            venue_room: None,
        }
    }
}

impl Default for Semantics {
    fn default() -> Self {
        Self::new()
    }
}

/// The balance redeemable with the pass.
///
/// An ISO 4217 currency code and an amount.
///
/// The total price for the pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyAmount {
    /// Amount of currency
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// Code of currency
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl CurrencyAmount {
    /// Create a new Instance
    pub fn new() -> Self {
        Self {
            amount: None,
            currency_code: None,
        }
    }
}

impl Default for CurrencyAmount {
    fn default() -> Self {
        Self::new()
    }
}

/// The geographic coordinates of the transit departure, suitable to be shown on a map. If
/// possible, precise locations are more useful to travelers, such as the specific location
/// of the gate at an airport.
///
/// Information about a location.
///
/// The geographic coordinates of the transit destination, suitable to be shown on a map.
///
/// A brief description of the destination location.
///
/// The geographic coordinates of the venue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Altitude, in meters, of the location.
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,

    /// Latitude, in degrees, of the location.
    #[serde(rename = "latitude")]
    pub latitude: f64,

    /// Longitude, in degrees, of the location.
    #[serde(rename = "longitude")]
    pub longitude: f64,

    /// Text displayed on the lock screen when the pass is currently relevant.
    #[serde(rename = "relevantText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_text: Option<String>,
}

impl Location {
    /// Create a new Instance
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            altitude: None,
            latitude,
            longitude,
            relevant_text: None,
        }
    }
}

/// The passenger's name.
///
/// An object that manages the separate parts of a person's name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonNameComponents {
    /// Name bestowed upon an individual to denote membership in a group or family.
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,

    /// Name bestowed upon an individual to differentiate them from other members of a group that
    /// share a family name.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// Secondary name bestowed upon an individual to differentiate them from others that have
    /// the same given name.
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// The portion of a name’s full form of address that precedes the name itself.
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,

    /// The portion of a name’s full form of address that follows the name itself.
    #[serde(rename = "nameSuffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// Name substituted for the purposes of familiarity.
    #[serde(rename = "nickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The phonetic representation name components of the receiver.
    #[serde(rename = "phoneticRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_representation: Box<Option<PhoneticRepresentation>>,
}

impl PersonNameComponents {
    /// Create a new Instance
    pub fn new() -> Self {
        Self {
            family_name: None,
            given_name: None,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            nickname: None,
            phonetic_representation: Box::new(None),
        }
    }
}

impl Default for PersonNameComponents {
    fn default() -> Self {
        Self::new()
    }
}

/// The phonetic representation name components of the receiver.
///
/// The passenger's name.
///
/// An object that manages the separate parts of a person's name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneticRepresentation {
    /// Name bestowed upon an individual to denote membership in a group or family.
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,

    /// Name bestowed upon an individual to differentiate them from other members of a group that
    /// share a family name.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// Secondary name bestowed upon an individual to differentiate them from others that have
    /// the same given name.
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// The portion of a name’s full form of address that precedes the name itself.
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,

    /// The portion of a name’s full form of address that follows the name itself.
    #[serde(rename = "nameSuffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// Name substituted for the purposes of familiarity.
    #[serde(rename = "nickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The phonetic representation name components of the receiver.
    #[serde(rename = "phoneticRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic_representation: Box<Option<PhoneticRepresentation>>,
}

impl PhoneticRepresentation {
    /// Create a new Instance
    pub fn new() -> Self {
        Self {
            family_name: None,
            given_name: None,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            nickname: None,
            phonetic_representation: Box::new(None),
        }
    }
}

impl Default for PhoneticRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

/// A dictionary with seat information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seat {
    /// Seat description
    #[serde(rename = "seatDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_description: Option<String>,

    /// Seat identifier
    #[serde(rename = "seatIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_identifier: Option<String>,

    /// Seat number
    #[serde(rename = "seatNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_number: Option<String>,

    /// Seat row
    #[serde(rename = "seatRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_row: Option<String>,

    /// Seat section
    #[serde(rename = "seatSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_section: Option<String>,

    /// Seat type
    #[serde(rename = "seatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<String>,
}

impl Seat {
    /// Create a new Instance
    pub fn new() -> Self {
        Self {
            seat_description: None,
            seat_identifier: None,
            seat_number: None,
            seat_row: None,
            seat_section: None,
            seat_type: None,
        }
    }
}

impl Default for Seat {
    fn default() -> Self {
        Self::new()
    }
}

/// Information specific to a coupon.
///
/// Information specific to an event ticket.
///
/// Information specific to a generic pass.
///
/// Information specific to a store card.
///
/// Information specific to a boarding pass.
///
/// Keys that define the structure of the pass.
/// These keys are used for all pass styles and partition the fields into the various parts
/// of the pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    /// Additional fields to be displayed on the front of the pass.
    #[serde(rename = "auxiliaryFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_fields: Option<Vec<Field>>,

    /// Fields to be on the back of the pass.
    #[serde(rename = "backFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_fields: Option<Vec<Field>>,

    /// Fields to be displayed in the header on the front of the pass. Use header fields
    /// sparingly; unlike all other fields, they remain visible when a stack of passes are
    /// displayed.
    #[serde(rename = "headerFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_fields: Option<Vec<Field>>,

    /// Fields to be displayed prominently on the front of the pass.
    #[serde(rename = "primaryFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_fields: Option<Vec<Field>>,

    /// Fields to be displayed on the front of the pass.
    #[serde(rename = "secondaryFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_fields: Option<Vec<Field>>,

    /// Type of transit.
    #[serde(rename = "transitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_type: Option<TransitType>,
}

impl Details {
    /// Create a new Instance
    pub fn new() -> Self {
        Self {
            auxiliary_fields: None,
            back_fields: None,
            header_fields: None,
            primary_fields: None,
            secondary_fields: None,
            transit_type: None,
        }
    }

    /// Type of transit.
    pub fn transit_type(&mut self, transit_type: TransitType) {
        self.transit_type = Some(transit_type);
    }

    /// Add additional field to be displayed on the front of the pass.
    pub fn add_auxiliary_field(&mut self, field: Field) {
        let mut vec = match &self.auxiliary_fields {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(field);

        self.auxiliary_fields = Some(vec);
    }

    /// Remove additional fields to be displayed on the front of the pass.
    pub fn clear_auxiliary_fields(&mut self) {
        self.auxiliary_fields = None;
    }

    /// Add field to be on the back of the pass.
    pub fn add_back_field(&mut self, field: Field) {
        let mut vec = match &self.back_fields {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(field);

        self.back_fields = Some(vec);
    }

    /// Remove fields to be on the back of the pass.
    pub fn clear_back_fields(&mut self) {
        self.back_fields = None;
    }

    /// Add field to be displayed in the header on the front of the pass. Use header fields
    /// sparingly; unlike all other fields, they remain visible when a stack of passes are
    /// displayed.
    pub fn add_header_field(&mut self, field: Field) {
        let mut vec = match &self.header_fields {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(field);

        self.header_fields = Some(vec);
    }

    /// Remove fields to be displayed in the header on the front of the pass. Use header fields
    /// sparingly; unlike all other fields, they remain visible when a stack of passes are
    /// displayed.
    pub fn clear_header_fields(&mut self) {
        self.header_fields = None;
    }

    /// Add field to be displayed prominently on the front of the pass.    
    pub fn add_primary_field(&mut self, field: Field) {
        let mut vec = match &self.primary_fields {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(field);

        self.primary_fields = Some(vec);
    }

    /// Remove fields to be displayed prominently on the front of the pass.
    pub fn clear_primary_fields(&mut self) {
        self.primary_fields = None;
    }

    /// Add field to be displayed on the front of the pass.
    pub fn add_secondary_field(&mut self, field: Field) {
        let mut vec = match &self.secondary_fields {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(field);

        self.secondary_fields = Some(vec);
    }

    /// Remove fields to be displayed on the front of the pass.
    pub fn clear_secondary_fields(&mut self) {
        self.secondary_fields = None;
    }
}

impl Default for Details {
    fn default() -> Self {
        Self::new()
    }
}

/// Information used for Value Added Service Protocol transactions.
/// Available in iOS 9.0.
///
/// Information about the NFC payload passed to an Apple Pay terminal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nfc {
    /// The public encryption key used by the Value Added Services protocol. Use a Base64 encoded
    /// X.509 SubjectPublicKeyInfo structure containing a ECDH public key for group P256.
    #[serde(rename = "encryptionPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_public_key: Option<String>,

    /// The payload to be transmitted to the Apple Pay terminal. Must be 64 bytes or less.
    /// Messages longer than 64 bytes are truncated by the system.
    #[serde(rename = "message")]
    pub message: String,
}

impl Nfc {
    /// Create a new Instance
    pub fn new(message: &str) -> Self {
        Self {
            encryption_public_key: None,
            message: message.into(),
        }
    }
}

/// Represents a Double or String value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueUnion {
    /// Represents a Double value
    Double(f64),

    /// Represents a String value
    String(String),
}

/// Barcode format. PKBarcodeFormatCode128 may only be used for dictionaries in the barcodes
/// array.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarcodeFormat {
    /// Barcode fromat `PKBarcodeFormatAztec`
    #[serde(rename = "PKBarcodeFormatAztec")]
    PkBarcodeFormatAztec,

    /// Barcode fromat `PKBarcodeFormatCode128`
    #[serde(rename = "PKBarcodeFormatCode128")]
    PkBarcodeFormatCode128,

    /// Barcode fromat `PKBarcodeFormatPDF417`
    #[serde(rename = "PKBarcodeFormatPDF417")]
    PkBarcodeFormatPdf417,

    /// Barcode fromat `PKBarcodeFormatQR`
    #[serde(rename = "PKBarcodeFormatQR")]
    PkBarcodeFormatQr,
}

/// Style of date to display.
///
/// Style of time to display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EStyle {
    /// date/time style `PKDateStyleFull`
    #[serde(rename = "PKDateStyleFull")]
    PkDateStyleFull,

    /// date/time style `PKDateStyleLong`
    #[serde(rename = "PKDateStyleLong")]
    PkDateStyleLong,

    /// date/time style `PKDateStyleMedium`
    #[serde(rename = "PKDateStyleMedium")]
    PkDateStyleMedium,

    /// date/time style `PKDateStyleNone`
    #[serde(rename = "PKDateStyleNone")]
    PkDateStyleNone,

    /// date/time style `PKDateStyleShort`
    #[serde(rename = "PKDateStyleShort")]
    PkDateStyleShort,
}

/// Style of number to display. Number styles have the same meaning as the Cocoa number
/// formatter styles with corresponding names. See
/// https://developer.apple.com/documentation/foundation/nsnumberformatterstyle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumberStyle {
    /// Number style `PKNumberStyleDecimal`
    #[serde(rename = "PKNumberStyleDecimal")]
    PkNumberStyleDecimal,

    /// Number style `PKNumberStylePercent`
    #[serde(rename = "PKNumberStylePercent")]
    PkNumberStylePercent,

    /// Number style `PKNumberStyleScientific`
    #[serde(rename = "PKNumberStyleScientific")]
    PkNumberStyleScientific,

    /// Number style `PKNumberStyleSpellOut`
    #[serde(rename = "PKNumberStyleSpellOut")]
    PkNumberStyleSpellOut,
}

/// The event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// Event type `PKEventTypeConference`
    #[serde(rename = "PKEventTypeConference")]
    PkEventTypeConference,

    /// Event type `PKEventTypeConvention`
    #[serde(rename = "PKEventTypeConvention")]
    PkEventTypeConvention,

    /// Event type `PKEventTypeGeneric`
    #[serde(rename = "PKEventTypeGeneric")]
    PkEventTypeGeneric,

    /// Event type `PKEventTypeLivePerformance`
    #[serde(rename = "PKEventTypeLivePerformance")]
    PkEventTypeLivePerformance,

    /// Event type `PKEventTypeMovie`
    #[serde(rename = "PKEventTypeMovie")]
    PkEventTypeMovie,

    /// Event type `PKEventTypeSocialGathering`
    #[serde(rename = "PKEventTypeSocialGathering")]
    PkEventTypeSocialGathering,

    /// Event type `PKEventTypeSports`
    #[serde(rename = "PKEventTypeSports")]
    PkEventTypeSports,

    /// Event type `PKEventTypeWorkshop`
    #[serde(rename = "PKEventTypeWorkshop")]
    PkEventTypeWorkshop,
}

/// Alignment for the field’s contents.
/// This key is not allowed for primary fields or back fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlignment {
    /// Alignment `PKTextAlignmentCenter`
    #[serde(rename = "PKTextAlignmentCenter")]
    PkTextAlignmentCenter,

    /// Alignment `PKTextAlignmentLeft`
    #[serde(rename = "PKTextAlignmentLeft")]
    PkTextAlignmentLeft,

    /// Alignment `PKTextAlignmentNatural`
    #[serde(rename = "PKTextAlignmentNatural")]
    PkTextAlignmentNatural,

    /// Alignment `PKTextAlignmentRight`
    #[serde(rename = "PKTextAlignmentRight")]
    PkTextAlignmentRight,
}

/// Type of transit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitType {
    /// Transit type `PKTransitTypeAir`
    #[serde(rename = "PKTransitTypeAir")]
    PkTransitTypeAir,

    /// Transit type `PKTransitTypeBoat`
    #[serde(rename = "PKTransitTypeBoat")]
    PkTransitTypeBoat,

    /// Transit type `PKTransitTypeBus`
    #[serde(rename = "PKTransitTypeBus")]
    PkTransitTypeBus,

    /// Transit type `PKTransitTypeGeneric`
    #[serde(rename = "PKTransitTypeGeneric")]
    PkTransitTypeGeneric,

    /// Transit type `PKTransitTypeTrain`
    #[serde(rename = "PKTransitTypeTrain")]
    PkTransitTypeTrain,
}

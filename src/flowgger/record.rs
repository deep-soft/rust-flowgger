use std::fmt;

#[derive(Debug, Clone)]
pub enum SDValue {
    String(String),
    Bool(bool),
    F64(f64),
    I64(i64),
    U64(u64),
    Null,
}

#[cfg(feature = "ltsv")]
#[derive(Debug, Clone)]
pub enum SDValueType {
    String,
    Bool,
    F64,
    I64,
    U64,
}

#[derive(Debug)]
pub struct StructuredData {
    pub sd_id: Option<String>,
    pub pairs: Vec<(String, SDValue)>,
}

impl StructuredData {
    pub fn new(sd_id: Option<&str>) -> StructuredData {
        StructuredData {
            sd_id: match sd_id {
                Some(sd_id) => Some(sd_id.to_owned()),
                None => None,
            },
            pairs: Vec::new(),
        }
    }
}

/// Implement the structured data display also provides to_string() for free
impl fmt::Display for StructuredData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;
        if let Some(sd_id) = &self.sd_id {
            f.write_str(&sd_id)?;
        }
        for &(ref name, ref value) in &self.pairs {
            // Remove trailing '_' if exists
            let name = if (*name).starts_with('_') {
                &name[1..] as &str
            } else {
                name as &str
            };

            match *value {
                SDValue::String(ref value) => write!(f, " {}=\"{}\"", name, value)?,
                SDValue::Bool(ref value) => write!(f, " {}=\"{}\"", name, value)?,
                SDValue::F64(ref value) => write!(f, " {}=\"{}\"", name, value)?,
                SDValue::I64(ref value) => write!(f, " {}=\"{}\"", name, value)?,
                SDValue::U64(ref value) => write!(f, " {}=\"{}\"", name, value)?,
                SDValue::Null => write!(f, " {}", name)?,
            }
        }
        f.write_str("]")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Record {
    pub ts: f64,
    pub hostname: String,
    pub facility: Option<u8>,
    pub severity: Option<u8>,
    pub appname: Option<String>,
    pub procid: Option<String>,
    pub msgid: Option<String>,
    pub msg: Option<String>,
    pub full_msg: Option<String>,
    pub sd: Option<StructuredData>,
}

#[cfg(feature = "capnp-recompile")]
pub const FACILITY_MAX: u8 = 0xff >> 3;
#[cfg(feature = "capnp-recompile")]
pub const FACILITY_MISSING: u8 = 0xff;
#[cfg(any(feature = "capnp-recompile", feature = "gelf"))]
pub const SEVERITY_MAX: u8 = (1 << 3) - 1;
#[cfg(feature = "capnp-recompile")]
pub const SEVERITY_MISSING: u8 = 0xff;


#[test]
fn test_structured_data_display() {
    let expected_string = r#"[someid a="b" c="123456"]"#;
    let data = StructuredData {
        sd_id: Some("someid".to_string()),
        pairs: vec![
            ("a".to_string(), SDValue::String("b".to_string())),
            ("c".to_string(), SDValue::U64(123456))
        ]
    };

    let result = data.to_string();
    assert_eq!(result, expected_string);
}

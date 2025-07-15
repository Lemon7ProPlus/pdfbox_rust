use std::collections::BTreeMap;
use lopdf::{decode_text_string, Dictionary, Document, Object};

pub mod prelude {
    pub use super::read_metadata;
    pub use super::modify_metada;
}

pub fn read_metadata(
    documents: &mut Vec<Document>,
    key: &str
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // possible metadata:
    // Title, Author, Subject, Keywords, Creator, Producer, CreationDate, ModDate
    let mut titles: Vec<String> = Vec::new();
    for document in documents {
        match get_metadata(document, key) {
            Ok(title) => {
                titles.push(title.clone());
            }
            Err(e) => {
                println!("Failed to get title: {e}");
                titles.push(String::new());
            }
        }
    }
    Ok(titles)
}

fn get_metadata(document: &Document, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut metadata = BTreeMap::new();

    if let Ok(Object::Reference(info_id)) = document.trailer.get(b"Info") {
        if let Some(Object::Dictionary(info_dict)) = document.objects.get(info_id) {
            for (key, value) in info_dict.into_iter() {
                let key_str = String::from_utf8_lossy(key);
                let value_str = match value {
                    Object::String(_, _) => {
                        match decode_text_string(value) {
                            Ok(decoded) => decoded,
                            Err(e) => {
                                eprintln!("Failed to decode string: {:?}", e);
                                if let Object::String(bytes, _) = value {
                                    String::from_utf8_lossy(bytes).to_string()
                                } else {
                                    format!("{:?}", value)
                                }
                            }
                        }
                    },
                    Object::Name(name) => String::from_utf8_lossy(name).to_string(),
                    _ => format!("{:?}", value)
                };
                metadata.insert(key_str.to_string(), value_str);
            }
        }
    }
    metadata
        .get(key)
        .ok_or_else(|| format!("No key in metadata: {}", key).into())
        .map(|s| s.to_string())
}

pub fn modify_metada(document: &mut Document, key: &str, title: String) -> Result<Document, Box<dyn std::error::Error>> {
    set_metadata(document, |info_dict| {
        info_dict.set(key, Object::String(title.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    })?;

    Ok(document.clone())
}

fn set_metadata<F> (
    document: &mut Document,
    metadata_updater: F 
) -> Result<Document, Box<dyn std::error::Error>> 
where 
    F: FnOnce(&mut Dictionary),
    {
        let info_id = match document.trailer.get(b"Info") {
            Ok(Object::Reference(id)) => *id,
            _ => {
                let info_dict = Dictionary::new();
                let new_id = (document.max_id + 1, 0);
                document.objects.insert(new_id, Object::Dictionary(info_dict));
                document.max_id += 1;
                let info_id = new_id;
                document.trailer.set("Info", Object::Reference(info_id));
                info_id
            }
        };

        if let Some(Object::Dictionary(info_dict)) = document.objects.get_mut(&info_id) {
            metadata_updater(info_dict)
        }

        Ok(document.clone())
    }
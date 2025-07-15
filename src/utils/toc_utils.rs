use lopdf::{Bookmark, Document, Object, ObjectId};

pub mod prelude {
    pub use super::add_toc;
}

pub fn add_toc(
    document: &mut Document,
    titles: Vec<String>,
    pagenums: Vec<usize>,
) -> Result<&Document, Box<dyn std::error::Error>> {
    for (title, pagenum) in titles.iter().zip(pagenums.iter()) {
        let page_id = get_page_id_by_number(&document, *pagenum)?;
        let bookmark = Bookmark::new(
            title.clone(), 
            [0.0, 0.0, 1.0], 
            0, 
            page_id
        );
        document.add_bookmark(bookmark, None);
    }

    document.adjust_zero_pages();

    if let Some(outline_id) = document.build_outline() {
        if let Ok(catalog_id) = document.trailer.get(b"Root") {
            if let Ok(catalog_id) = catalog_id.as_reference() {
                if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_id) {
                    dict.set("Outlines", Object::Reference(outline_id));
                }
            }
        }
    }
    // document.build_outline();

    Ok(document)
}

fn get_page_id_by_number(
    doc: &Document,
    page_number: usize,
) -> Result<ObjectId, Box<dyn std::error::Error>> {
    let pages = doc.get_pages();
    let page_index = page_number - 1;

    let page_keys: Vec<u32> = pages.keys().cloned().collect();
    if page_index < page_keys.len() {
        let page_key = page_keys[page_index];
        if let Some(page_obj) = pages.get(&page_key) {
            return Ok((page_obj.0, page_obj.1));
        }
    }

    Err(format!("Page {} doesn't exist", page_number).into())
}


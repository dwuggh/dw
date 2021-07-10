mod web;

pub use web::*;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::server::Query;

    use super::google_translate::GTrans;

    fn query_en(text: &str) -> Arc<Query> {
        let query = Query::new(text, "en", "zh", false);
        Arc::new(query)
    }

    fn query_fuck() -> Arc<Query> {
        query_en("fuck")
    }

    #[test]
    fn google_translate_can_translate_fuck() {
        crate::server::config::init().unwrap();
        let g = GTrans::new();
        let query = query_fuck();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let resp = rt.block_on(g.query(Arc::clone(&query))).unwrap();
        assert_eq!(resp.basic_desc, "他妈的")
    }
}

use std::collections::HashMap;

pub fn search(query: &Vec<String>, hashed_docs: &HashMap<String, HashMap<String, u32>>) -> Option<Vec<(String, f64)>> {

    let mut document_scores = Vec::new();
    for (path, doc) in hashed_docs {
        let mut tf_idf = 0.0;
        for token in query {
            tf_idf += get_idf(&token, &hashed_docs) * get_tf(&token, &doc);
        }
        document_scores.push((path.to_string(), tf_idf));
    }


    document_scores
    .sort_by(|a, b| match b.1.partial_cmp(&a.1) {
        Some(document_scores) => document_scores,
        None => std::cmp::Ordering::Less,
    });
    return Some(document_scores.to_vec());
}

fn get_idf(query: &str, hashed_docs: &HashMap<String, HashMap<String, u32>>) -> f64 {
    let m = hashed_docs.values().filter(|tf| tf.contains_key(query)).count().max(1) as f64;
    let size = hashed_docs.len()as f64;
    return (size/m).log10();
}

fn get_tf(query: &str, hashed_doc: &HashMap<String, u32>) -> f64 {
    let total_frequency = hashed_doc.get(query).cloned().unwrap_or(0) as f64;
    let total_words = hashed_doc.iter().map(|(_, t)| *t).sum::<u32>() as f64;
    return total_frequency/total_words;
}

pub fn print_tfidf_vecs(doc: &Vec<(String, f64)>){
    for token in doc.iter().take(5) {
        println!("{}: TFIDF - {}", token.0, token.1);
    }
}


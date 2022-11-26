pub fn levenshtein(a: &str, b: &str) -> usize {
    let mut result = 0;
    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result;
    }
    let _len_a = a.chars().count();
    let _len_b = b.chars().count();
    
    if _len_a == 0 {
        return _len_b;
    }
    if _len_b == 0 {
        return _len_a;
    }

    /* Initialize the vector. Normally a matrix is used. 
     * This is fast single vector implementation. 
    */
     let mut cache: Vec<usize> = (1..).take(_len_a).collect();
     let mut distance_a;
     let mut distance_b;
 
     /* Loop. */
     for (index_b, code_b) in b.chars().enumerate() {
         result = index_b;
         distance_a = index_b;
 
         for (index_a, code_a) in a.chars().enumerate() {
             distance_b = if code_a == code_b {
                 distance_a
             } else {
                 distance_a + 1
             };
 
             distance_a = cache[index_a]; 
             result = if distance_a > result {
                 if distance_b > result {
                     result + 1
                 } else {
                     distance_b
                 }
             } else if distance_b > distance_a {
                 distance_a + 1
             } else {
                 distance_b
             };
             cache[index_a] = result;
         }
    }

    return result;
}
// https://github.com/wooorm/levenshtein-rs

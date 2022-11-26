<template>
    <div>
      <h1>Levenshtein Test</h1>
      <p>Levenshtein distance (or edit distance) between two strings is the number of deletions, insertions, or substitutions required to transform source string into target string. For example, if the source string is "book" and the target string is "back," to transform "book" to "back," you will need to change first "o" to "a," second "o" to "c," without additional deletions and insertions. Thus, the Levenshtein distance between "book" and "back" will be 2.</p>
      <input v-model="input1" placeholder="input1" />
      <input v-model="input2" placeholder="input2" />      

      <button @click="doLvs"> Do Levenshtein </button>
      <p>{{resultLev}}</p>

      <hr>
      <p>The Levenshtein distance between two strings a, b (of length |a| and |b| respectively) is given by lev(a,b) where:</p>
      <img alt="Levenshtein algorithm" src="../assets/lvs.png" />

      <p>The Levenshtein distance is named after the Russian scientist Vladimir Levenshtein, who devised the metric in 1965. 
        There are several algorithms to compute the Levenshtein distance:</p>
      <ul>
        <li>Recursive; the straightforward algorithm, which follows the definition</li>
        <li>Iterative with full matrix; the one used in the calculator above</li>
        <li>Iterative with two matrix rows</li>
      </ul>

<h2>Levenshtein in Rust</h2>

<pre><code>
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
     let mut cache: Vec &lt;usize&gt; = (1..).take(_len_a).collect();
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
</code></pre>

  </div>
</template>

<script>
// https://planetcalc.com/1721/
export default {
  name: "Levenshtein",
  components: {
  
  },
  data() {
    return {
      resultLev: 0,
      wasm : null,
      input1:"book",
      input2:"back"
    }
  },
  methods: {
    async doLvs() {
      this.resultLev = await this.wasm.levenshtein(this.input1,this.input2);      
    }    
  },
  async mounted() {
    this.wasm = await import("../../wasm/pkg");
  }
};
</script>

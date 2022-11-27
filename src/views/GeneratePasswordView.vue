<template>
    <div>
      <h1>Generate Password</h1>
      <form>      
        <label>Num Chars:&nbsp;<span><input v-model="numChars" type="range" min="6" max="16">&nbsp;{{numChars}}</span></label><br>
        <label>ABC..:&nbsp;<input v-model="useUpperCase" type="checkbox" checked="useUpperCase"/></label><br>
        <label>abc..:&nbsp;<input v-model="useLowerCase" type="checkbox" checked="useLowerCase"/></label><br>
        <label>123..:&nbsp;<input v-model="useNumbers" type="checkbox" checked="useNumbers"/></label><br>
        <label>$#@..:&nbsp;<input v-model="useSpecialChars" type="checkbox" checked="useSpecialChars"/></label><br>
      </form>

      <hr>
      <button @click="doGeneratePwd"> Generate Password </button>
      <p>{{results}}  ({{results.length}})</p>
    </div>
</template>
<script>
export default {
  name: "GeneratePassword",
  components: { },
  data() {
    return {
      results: 0,
      numChars: 8,
      useSpecialChars:true,
      useUpperCase:true,
      useLowerCase:true,
      useNumbers:true,
      wasm : null,
    }
  },
  methods: {
    async doGeneratePwd() {
      this.results = await this.wasm.generatepwd(
        parseInt(this.numChars, 10),
        this.useSpecialChars,
        this.useUpperCase,
        this.useLowerCase,
        this.useNumbers
      );      
    }    
  },

  async mounted() {
    this.wasm = await import("../../wasm/pkg");
  }
};
</script>

<style>
</style>
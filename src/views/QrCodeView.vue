<template>
    <div>
      <h1>QRCode Generator</h1>
      <input v-model="input1" size="50" placeholder="input1" />
      <br>
      <br>
      <button @click="createQRCode">Create QR Code</button>
      <br>
      <br>
      <img v-if="qrcode" :src="qrcode" width="180"/>
    </div>
</template>
  
<script>
export default {
  name: "AboutView",
  components: { },

  data() {
    return {
      input1:"Give Rust and Webassembly a try!",
      wasm : null,
      qrcode: null,
    }
  },

  methods: {
    async createQRCode() {
      this.qrcode = await this.wasm.qrcode(this.input1,100,100);
    },
  },
  async mounted() {
    this.wasm = await import("../../wasm/pkg");
  }
}
</script>

<style>
</style>
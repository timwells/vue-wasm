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

      <br>
      <br>
      
      <h3>Rust Code</h3>      
      <pre>
        <code>
extern crate qrcode;
use qrcode::render::svg;
use qrcode::QrCode;
use qrcode::types::QrError;

/// Generate a QR code from the respective data. Returns a string containing the SVG string
fn qrcode &lt;'T&gt;(data: T, width: u32, height: u32) -> Result&lt;String, QrError&gt;
where T: AsRef&lt;[u8]&gt; {
    QrCode::with_error_correction_level(data.as_ref(), qrcode::EcLevel::Q)
        .map(|code| code.render::&lt;svg::Color&gt;()
            .max_dimensions(width, height)
            .min_dimensions(width, height)
            .build()
        )
}

/// Returns a new pointer to a new location in memory where the SVG code for the qrcode 
/// as a base64 is located.
pub fn qrcode_ffi(arg: &str, width: u32, height: u32) -> String {
    match qrcode(arg, width, height) {
        Ok(v) => format!("{}{}","data:image/svg+xml;base64,", base64::encode(v)),
        // Since we're on an FFI boundary we can't return strongly typed errors. Instead if we get
        // an error from the qrcode generation we return the error string.
        Err(e) => format!("{}", e),
    }
}        
        </code>
    </pre>


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
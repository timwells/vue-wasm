<template>
    <div>
      <h1>Image Dectection</h1>
      <img id="img" src="../assets/piano.jpg" width="224px" />
      <br>
      <br>
      <button @click="doDectect">Dectect</button>

</div>
</template>
  
<script>
import "@tensorflow/tfjs-backend-wasm";
import * as tf from "@tensorflow/tfjs";

export default {
  name: "ImageDectection",
  components: { },

  data() {
    return {
      wasm : null,
      model : null,
      url: 'https://storage.googleapis.com/tfjs-models/savedmodel/mobilenet_v2_1.0_224/model.json'

    }
  },

  /*
import * as tf from "@tensorflow/tfjs"
  
// Defining tensor input elements
const model_Url =

'https://storage.googleapis.com/tfjs-models/savedmodel/mobilenet_v2_1.0_224/model.json';
  
// Calling the loadGraphModel() method
const mymodel = await tf.loadGraphModel(model_Url);
  
// Defining inputs
const inputs = tf.zeros([1, 224, 224, 3]);
  
// Calling predict() method and 
// Printing output
mymodel.predict(inputs).print();
  */


  methods: {
      async doDectect() {
        console.log("doDectect");
        const imgElement = document.getElementById("img");
        let image = tf.browser
          .fromPixels(imgElement)
          .resizeBilinear([224, 224])
          .expandDims(0)
          .toFloat();
      
        let model = await tf.loadGraphModel(
          this.url,
        // 'https://tfhub.dev/google/imagenet/mobilenet_v2_100_224/classification/2',
          {fromTFHub: true});
        const y = model.predict(image);
        // console.log(y.dataSync()[4])
        console.log(y.dataSync()[5])
        //console.log(y[1])
        //console.log(y[2])
      }
  },
  async mounted() {
    // this.wasm = await import("../../wasm/pkg");

    await tf.setBackend("wasm");

    tf.ready().then(async () => {
      this.model = await tf.loadGraphModel(
        "https://tfhub.dev/google/tfjs-model/imagenet/mobilenet_v2_100_224/classification/3/default/1",
          { fromTFHub: true })
      console.log("ready")
    });
  }
}
</script>

<style>
</style>
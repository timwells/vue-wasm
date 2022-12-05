<template>
    <div>
        <h3>Javascript vs. WebAssembly - Performance</h3>
        <input type="radio" id="jsbox" name="algorithm" />
            <label for="jsbox">Javascript</label>
        <input type="radio" id="wasmbox" name="algorithm" />
            <label for="wasmbox">WebAssembly</label>
        
        <div class="buttons">
            <button id="noiseBtn">Noise</button>
            &nbsp;&nbsp;
            <button id="clearBtn">Clear</button>
            &nbsp;&nbsp;
            <span>Model Frame Rate:</span><span id="fpsId"></span>
        </div>

        <!--p class="description">
            This is a simulation of the <a href="https://en.wikipedia.org/wiki/Wave_equation">wave equation</a>
            <span class="eq">&#8706;<sup>2</sup>u/&#8706;t<sup>2</sup> = c<sup>2</sup>(&#8706;<sup>2</sup>u/&#8706;x<sup>2</sup>+&#8706;<sup>2</sup>u/&#8706;y<sup>2</sup>)</span>
            across a circular 2D manifold. Use the mouse to create waves.
        </p-->
        <!--p class="description">
            This is becoming a demonstration of how extensively Codepen messes with JavaScript; the performance of the JavaScript version has been deteriorating over time. It's not normally this much slower than WebAssembly.
        </p-->
        <!--p class="description">
            It contains two implementations of the same C code: one transpiled by hand to JavaScript, and one compiled by Enscripten to WebAssembly.
            This particular algorithm processes large arrays and performs a lot of integer math (but does no floating point calculations).
            Memory is shared between JS and WebAssembly with no copying, and almost all CPU time is spent in the algorithm itself.
            (The canvas API introduces a minor overhead of about 10%.)
            There is a modest speedup when the same code is run in a WebAssembly context as opposed to a JavaScript context.
        </p-->
 
        <canvas id="canvas" :width="width" :height="height"></canvas> 
    </div>
</template>
  
<script>
// https://codepen.io/jtiscione/pen/yxybjX
export default {
    name: "WaveEquation",
    components: { },

    data() {
        return {
            model:"jsModel",

            //imageArray : null,
            //forceArray : null,
            //statusArray : null,
            //applyBrakes : false,
            //u0_offset: 0,
            //u1_offset: 0,
            
            //timestamps: [],
            //lastFpsJitter: 0,
            //animationCount: 0,
            //lastMouseX : null,
            //lastMouseY : null,
            //wh : 0,
            
            //animationMax : 100,
            //unsignedHeap : null,
            //signedHeap: null,

            //status_offset : 0, 
            //vel_offset : 0,
            //force_offset: 0,
            //heapSize: 0,
            //heap : null,

            width: 550, 
            height: 550,
            
            //algorithm: null,
            //wasmAlgorithm: null,
            //jsAlgorithm : null,

            //canvas: null,
            //context: null,
            
            //fps: 0,

            //cbrushMatrix: [],
            //brushMatrixRadius: 28,
        }
    },

    methods: {
        jsWaveAlgorithm3(width, height) {
            const ALPHA = 0xFF000000;
            const STATUS_DEFAULT = 0;
            const STATUS_WALL = 1;
            const STATUS_POS_TRANSMITTER = 2;
            const STATUS_NEG_TRANSMITTER = 3;
            const FORCE_DAMPING_BIT_SHIFT = 4;

            const wh = width * height;
            let u0_offset = wh;
            let u1_offset = 2 * wh;
            const vel_offset = 3 * wh;
            const force_offset = 4 * wh;
            const status_offset = 5 * wh;

            // Need room for six Int32 arrays, each with imageWidth * imageHeight elements.
            const heapSize = 6 * 4 * wh;
            const heap = new ArrayBuffer(heapSize);
            const unsignedHeap = new Uint32Array(heap);
            const signedHeap = new Int32Array(heap);

            // To avoid falling off edges, mark the pixels along the edge as being wall pixels.
            // Walls implement a Dirichlet boundary condition by setting u=0.
            for (let i = 0; i < height; i++) {
                unsignedHeap[status_offset + i * width] = STATUS_WALL; // left edge
                unsignedHeap[status_offset + (i * width) + (width - 1)] = STATUS_WALL; // right edge
            }
            for (let j = 0; j < width; j++) {
                unsignedHeap[status_offset + j] = STATUS_WALL; // top edge
                unsignedHeap[status_offset + (width * (height - 1)) + j] = STATUS_WALL; // bottom edge
            }

            // Full int32 range is -0x80000000 to 0x7FFFFFFF. Use half.
            function applyCap(x) {
                return x < -0x40000000 ? -0x40000000 : (x > 0x3FFFFFFF ? 0x3FFFFFFF : x);
            }

            function toRGB(signed32bitValue) {
                // Map negative values to red, positive to blue-green, zero to black
                let val = (signed32bitValue >> 22);
                let rgba = ALPHA;
                if (val > 0) {
                    rgba = (val << 8) | (val << 16) | ALPHA; // blue-green
                } else if (val < 0) {
                    val = val + 1; // OR: val = Math.max(val, -255);
                    rgba = -val | ALPHA; // red
                }
                return rgba;
            }

            
            // Applies the wave equation d2u/dt2 = c*c*(d2u/dx2+d2u/dy2)
            //where all derivatives on the right are partial 2nd derivatives
            function singleFrame(signalAmplitude, dampingBitShift = 0) {
                console.log("singleFrame...")
                let uCen = 0,
                uNorth = 0,
                uSouth = 0,
                uEast = 0,
                uWest = 0;

                // Do two iterations- first writing into swap and then writing back into u0 region
                for (let cycle = 0; cycle < 2; cycle++) {
                    // First loop: look for noise generator pixels and set their values in u
                    for (let i = 0; i < wh; i++) {
                        const status = unsignedHeap[status_offset + i];
                        if (status === STATUS_POS_TRANSMITTER) {
                            signedHeap[u1_offset + i] = signalAmplitude;
                            signedHeap[vel_offset + i] = 0;
                            signedHeap[force_offset + i] = 0;
                        }
                        if (status === STATUS_NEG_TRANSMITTER) {
                            signedHeap[u1_offset + i] = -signalAmplitude;
                            signedHeap[vel_offset + i] = 0;
                            signedHeap[force_offset + i] = 0;
                        }
                    }
                    // Second loop: apply wave equation at all pixels
                    for (let i = 0; i < wh; i++) {
                        if (unsignedHeap[status_offset + i] === STATUS_DEFAULT) {
                            uCen = signedHeap[u0_offset + i];
                            uNorth = signedHeap[u0_offset + i - width];
                            uSouth = signedHeap[u0_offset + i + width];
                            uWest = signedHeap[u0_offset + i - 1];
                            uEast = signedHeap[u0_offset + i + 1];

                            const uxx = (((uWest + uEast) >> 1) - uCen);
                            const uyy = (((uNorth + uSouth) >> 1) - uCen);

                            let vel = signedHeap[vel_offset + i];
                            vel = vel + (uxx >> 1);
                            vel = vel + (uyy >> 1);
                            vel = applyCap(vel);

                            let force = signedHeap[force_offset + i];
                            signedHeap[u1_offset + i] = applyCap(force + applyCap(uCen + vel));
                            force -= (force >> FORCE_DAMPING_BIT_SHIFT);
                            signedHeap[force_offset + i] = force;

                            if (dampingBitShift) {
                                vel -= (vel >> dampingBitShift);
                            }
                            signedHeap[vel_offset + i] = vel;
                        }
                    }
                    const swap = u0_offset;
                    u0_offset = u1_offset;
                    u1_offset = swap;
                }

                // Final pass: calculate color values
                for (let i = 0; i < wh; i++) {
                    if (signedHeap[status_offset + i] === STATUS_WALL) {
                        unsignedHeap[i] = 0x00000000;
                    } else {
                        unsignedHeap[i] = toRGB(signedHeap[u0_offset + i]);
                    }
                }
            }

            return {
                getImageArray: function () {
                    return new Uint8ClampedArray(heap, 0, 4 * wh);
                },
                getForceArray: function () {
                    return new Int32Array(heap, 4 * force_offset, wh);
                },
                getStatusArray: function () {
                    return new Int32Array(heap, 4 * status_offset, wh);
                },
                getEntireArray: function () {
                    return unsignedHeap;
                },
                singleFrame
            };
        },

        wasmWaveAlgorithm3(wasm, width, height) {
            const instance = wasm.instance;
            instance.exports.init(width, height);
            const startByteOffset = instance.exports.getStartByteOffset();

            // These are int32 offsets- multiply by 4 to get byte offsets.
            const wh = width * height;
            const force_offset = 4 * wh;
            const status_offset = 5 * wh;
            const heap = instance.exports.memory.buffer;

            return {
                // The "output" from WASM
                getImageArray: function () {
                    return new Uint8ClampedArray(heap, startByteOffset, 4 * wh);
                },
                // Input to WASM: mouse movements cause writes to this array
                getForceArray: function () {
                    return new Int32Array(heap, startByteOffset + (4 * force_offset), wh);
                },
                // Input to WASM: wall and transmitter statuses can be set programmatically
                getStatusArray: function () {
                    return new Int32Array(heap, startByteOffset + (4 * status_offset), wh);
                },
                // For bulk copying, etc.
                getEntireArray: function () {
                    return new Uint32Array(heap, startByteOffset, 6 * wh);
                },
                // The main hot spot function that needs to run in WebAssembly:
                singleFrame: function (signalAmplitude, drag = false) {
                    instance.exports.singleFrame(signalAmplitude, drag);
                },
            };
        },

        wave3(wasm) {
            const canvas = document.getElementById('canvas');            
            const jsBox = document.getElementById('jsbox');
            const wasmBox = document.getElementById('wasmbox');
            const noiseBtn = document.getElementById('noiseBtn');
            const clearBtn = document.getElementById('clearBtn');

            let width = canvas.width;
            let height = canvas.height;
            const context = canvas.getContext('2d');

            let imageArray = null;
            let forceArray = null;
            let statusArray = null;
            let applyBrakes = false;

            const jsAlgorithm = this.jsWaveAlgorithm3(width, height);
            let algorithm = jsAlgorithm;
            let wasmAlgorithm = null;

            const swap = function (replacement) {
                    replacement.getEntireArray().set(algorithm.getEntireArray());
                    algorithm = replacement;
                    forceArray = null;
                    statusArray = null;
                    imageArray = null;
            };
            if (wasm) {
                wasmAlgorithm = this.wasmWaveAlgorithm3(wasm, width, height);
                algorithm = wasmAlgorithm;
                wasmBox.checked = true;

                jsBox.addEventListener('click', function (event) {
                    console.log(event.type)
                    console.log("jsBox")
                    swap(jsAlgorithm);
                });
                wasmBox.addEventListener('click', function (event) {
                    console.log(event.type)
                    console.log("wasmBox")
                    swap(wasmAlgorithm);
                });
            } else {
                //jsBox.disabled = true;
                //wasmBox.disabled = true;
                //document.getElementById('radio').style.display = 'none';
                //document.getElementById('sorry').style.display = 'block';
            }

            let timestamps = [];
            let lastFpsJitter = 0;
            let animationCount = 0;

            let lastMouseX = null, lastMouseY = null;

            function animate() {
                setTimeout(animate, 0);
                if (animationCount === 0) {
                    // First frame - Sprinkle noise generator pixels and set applyBrakes flag
                    const threshold = 0.001;
                    if (statusArray === null) {
                        statusArray = algorithm.getStatusArray();
                    }
                    for (let i = 0; i < statusArray.length; i++) {
                        if (Math.random() < threshold) {
                            statusArray[i] = (i % 2 === 0) ? 2 : 3;
                        }
                    }
                    // Draw a circular wall
                    const centerX = width / 2;
                    const centerY = height / 2;
                    const radius = Math.min((width / 2), (height / 2));
                    for (let i = 0; i < height; i++) {
                        for (let j = 0; j < width; j++) {
                            let dist = Math.sqrt(((i - centerY) * (i - centerY)) + ((j - centerX) * (j - centerX)));
                            if (dist > radius) {
                                const targetIndex = i * width + j;
                                statusArray[targetIndex] = 1;
                            }
                        }
                    }
                    applyBrakes = true;
                }

                if (animationCount === 200) { //100
                    // Hundredth frame- clear noise generator pixels and clear applyBrakes flag
                    for (let i = 0; i < statusArray.length; i++) {
                        if (statusArray[i] === 2 || statusArray[i] === 3) {
                            statusArray[i] = 0;
                        }
                    }
                    applyBrakes = false;
                }

                if (lastMouseX !== null && lastMouseY !== null) {
                    applyBrush(lastMouseX, lastMouseY);
                }

                let amplitude = Math.floor(0x3FFFFFFF * Math.sin(6.283 * animationCount / 100));
                algorithm.singleFrame(amplitude, applyBrakes ? 5 : 0);

                if (imageArray === null) {
                    imageArray = algorithm.getImageArray();
                }
                const imgData = context.createImageData(width, height);
                imgData.data.set(imageArray);
                context.putImageData(imgData, 0, 0);
                const now = Date.now();
                timestamps.push(now);
                timestamps = timestamps.filter(function (e) {
                    return ((now - e) < 1000);
                });
                
                const count = timestamps.length;
                if (now - lastFpsJitter > 400) {
                    lastFpsJitter = now;

                    document.getElementById('fpsId').innerHTML = count;
                }

                animationCount++;
            }

            function windowToCanvas(canvas, x, y) {
                const bbox = canvas.getBoundingClientRect();
                return {
                    x: Math.round(x - bbox.left * (canvas.width / bbox.width)),
                    y: Math.round(y - bbox.top * (canvas.height / bbox.height))
                };
            }

            const brushMatrix = [];
            const brushMatrixRadius = 28;
            for (let p = -brushMatrixRadius; p <= brushMatrixRadius; p++) {
                const row = [];
                brushMatrix.push(row);
                for (let q = -brushMatrixRadius; q <= brushMatrixRadius; q++) {
                    const element = Math.floor(0x3FFFFFFF * Math.exp(-0.05 * ((p * p) + (q * q))));
                    row.push(element);
                }
            }
            function applyCap(x) {
                return x < -0x40000000 ? -0x40000000 : (x > 0x3FFFFFFF ? 0x3FFFFFFF : x);
            }
            function applyBrush(x, y) {
                if (forceArray === null) {
                    forceArray = algorithm.getForceArray();
                }
                for (let p = -brushMatrixRadius; p <= brushMatrixRadius; p++) {
                    const targetY = y + p;
                    if (targetY <= 0 || targetY >= height - 1) {
                        continue;
                    }
                    for (let q = -brushMatrixRadius; q <= brushMatrixRadius; q++) {
                        const targetX = x + q;
                        if (targetX <= 0 || targetX >= width - 1) {
                            continue;
                        }
                        const brushValue = brushMatrix[p + brushMatrixRadius][q + brushMatrixRadius];
                        const targetIndex = targetY * width + targetX;
                        forceArray[targetIndex] += brushValue;
                        forceArray[targetIndex] = applyCap(forceArray[targetIndex]);
                    }
                }
            }

            canvas.onmousedown = function (e) {
                e.preventDefault();
                applyBrakes = false;
                const loc = windowToCanvas(canvas, e.clientX, e.clientY);
                lastMouseX = loc.x;
                lastMouseY = loc.y;
                applyBrush(loc.x, loc.y);
            };

            canvas.ontouchstart = function (e) {
                e.preventDefault();
                applyBrakes = false;
                for (let i = 0; i < e.targetTouches.length; i++) {
                    const touch = e.targetTouches[i];
                    const loc = windowToCanvas(canvas, touch.clientX, touch.clientY);
                    lastMouseX = loc.x;
                    lastMouseY = loc.y;
                    applyBrush(loc.x, loc.y);
                }
            };

            canvas.onmousemove = function (e) {
                e.preventDefault();
                const loc = windowToCanvas(canvas, e.clientX, e.clientY);
                const targetX = loc.x,
                targetY = loc.y;
                if (lastMouseX !== null && lastMouseY !== null) {
                    // draw a line from the last place we were to the current place
                    const r = Math.sqrt((loc.x - lastMouseX) * (loc.x - lastMouseX) + (loc.y - lastMouseY) * (loc.y - lastMouseY));
                    for (let t = 0; t < r; t++) {
                        const currX = Math.round(lastMouseX + (targetX - lastMouseX) * (t / r));
                        const currY = Math.round(lastMouseY + (targetY - lastMouseY) * (t / r));
                        applyBrush(currX, currY);
                    }
                    applyBrush(loc.x, loc.y);
                    lastMouseX = loc.x;
                    lastMouseY = loc.y;
                }
            };

            canvas.ontouchmove = function (e) {
                e.preventDefault();
                if (lastMouseX !== null && lastMouseY !== null) {
                    for (let i = 0; i < e.targetTouches.length; i++) {
                        const touch = e.targetTouches[i];
                        const loc = windowToCanvas(canvas, touch.clientX, touch.clientY);
                        const targetX = loc.x,
                        targetY = loc.y;
                        // draw a line from the last place we were to the current place
                        const r = Math.sqrt((loc.x - lastMouseX) * (loc.x - lastMouseX) + (loc.y - lastMouseY) * (loc.y - lastMouseY));
                        for (let t = 0; t < r; t++) {
                            const currX = Math.round(lastMouseX + (targetX - lastMouseX) * (t / r));
                            const currY = Math.round(lastMouseY + (targetY - lastMouseY) * (t / r));
                            applyBrush(currX, currY);
                        }
                        applyBrush(loc.x, loc.y);
                        lastMouseX = loc.x;
                        lastMouseY = loc.y;
                    }
                }
            };

            canvas.onmouseover = canvas.onmouseout = canvas.onmouseup = canvas.ontouchend = function (e) {
                e.preventDefault();
                lastMouseX = null;
                lastMouseY = null;
            };

            if (noiseBtn) {
                noiseBtn.addEventListener('click', function (e) {
                    e.preventDefault();
                    animationCount = 0;
                });
            }
            if (clearBtn) {
                clearBtn.addEventListener('click', function (e) {
                    e.preventDefault();
                    applyBrakes = true;
                });
            }

            animate();
        },

        webAssemblySupported() {
            try {
                if (typeof WebAssembly === "object" && typeof WebAssembly.instantiate === "function") {
                    const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
                    if (module instanceof WebAssembly.Module)
                        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
                }
            } catch (e) {
                console.log(e)
            }
            return false;
        }
    },
    async mounted() {        
        if (!this.webAssemblySupported()) {
            console.log("mounted: IS NOT webAssemblySupported")
            this.wave3(null);
        } else {
            console.log("mounted: IS webAssemblySupported")
            // Smuggle WASM code into vue as an inlined base-64 string literal. (https://rot47.net/base64encoder.html)
            const b64 = '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';
            const binaryString = window.atob(b64);
            const len = binaryString.length;
            const unsigned = new Uint8Array(len);
            for (let i = 0; i < len; i++) {
                unsigned[i] = binaryString.charCodeAt(i);
            }

            WebAssembly.instantiate(unsigned.buffer, { env: {} }).then((instance) => { this.wave3(instance) });
        }
    }
}
</script>

<style>
.container {
    padding: 10px;
    margin: auto;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: flex-start;
    user-select: none;
}

.huge {
    font-size: 20pt;
}

.big {
    font-size: 1.5em;
    padding: 0px;
    margin: 0px;
}

.bigger {
    font-size: 2em;
    padding: 0px;
    margin: 0px;
}

.buttons {
    margin-top: 10px;
}

.btn {
    box-shadow: 0px 1px 0px 0px #141412;
    background:linear-gradient(to bottom, #eae0c2 5%, #ccc2a6 100%);
    background-color:#eae0c2;
    border-radius:15px;
    border:2px solid #8a7648;
    display:inline-block;
    cursor:pointer;
    color:#505739;
    font-family:Arial;
    font-size:14px;
    font-weight:bold;
    padding:5px 10px;
    text-decoration:none;
    text-shadow:0px 1px 0px #ffffff;
}
.btn:hover {
    background:linear-gradient(to bottom, #ccc2a6 5%, #eae0c2 100%);
}
.btn:active {
    position:relative;
    top:1px;
}

.description {
    text-align: left;
}

.boxes {
    margin: 5px;
}

.boxes .radio {
    overflow: hidden;
    white-space: nowrap;
}

.boxes .lame {
    margin: 20px;
    color: darkred;
}

.boxes input {
    margin: 20px auto;
    background: #fcfff4;
    background: linear-gradient(to bottom, #fcfff4 0%, #dfe5d7 40%, #b3bead 100%);
    border-radius: 10px;
    box-shadow: inset 0px 1px 1px white, 0px 1px 3px rgba(0, 0, 0, 0.5);
    color: green;
}

.boxes label {
    margin-right: 10px;
}

</style>
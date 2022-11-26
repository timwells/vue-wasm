<template>
    <div>        
        <h2>What is WebAssembly</h2>
        <p>WebAssembly is a new, low level binary code, for the web</p>
        <ul>
            <li>It’s a new language. It became the offical “fourth language for web” 
                    on 5th December 2019, i.e. after HTML, CSS & JavaScript</li>
            <li>Its a standard <a href="https://webassembly.org/">webassembly.org</a></li>
            <li>It’s binary, so it is not really human readable (it’s an “assembly” language for the web)</li>
            <li>It’s low level, i.e. it’s much closer to actual machine code (than JavaScript and other higher
                 level languages)</li>
            <li>And it’s for the web, that means (modern) web browsers can read and execute them</li>
        </ul>

        <p>WebAssembly is a binary language. Though there is a text representation, 
            you as a developer won’t generally “write” code in WebAssembly! It’s actually an IR or 
            Intermediate Representation code, comparable to ByteCode in Java or MSIL/CIL in .NET. 
            It’s meant to be a compilation target for higher level languages.</p>

        <p>WebAssembly is set of low level binary instructions that works across common 
            standard hardware. Unlike higher level languages like C#, Go or Java, it is more 
            closer to machine language, and can be converted into actual machine code very fast & 
            efficiently. So rather than writing code in it, you still write code in a language that you 
            already know, like C/C++ or Rust, and a special compiler compiles your code into WebAssembly, 
            and that runs inside a special Virtual Machine. This VM can turn the IR into actual, platform 
            specific Machine Code and execute them. This VM can run on any type of platform (hardware + OS).</p>  
        <img src="../assets/WebAssembly_compile.png"/>
    
        <h2>Why WebAssembly?</h2>

        <p>JavaScript works well and is fine tonnes for developers. 
            There are great and productive framworsk like Angular, React, Vue etc. 
            that make complex web development much easier. 
            In fact JS is so accessible, that developers even write back-end or server-side code in JS, 
            thanks to NodeJS</p>
        
        <p>
            However, there are issues with JavaScript, mainly with performance. 
            It works fine for a standard web applications where small pieces of processing is 
            done on the web, based on user interaction. Generally, the heavy processing is done 
            on servers, and web interacts with them through http services. But it does not work well 
            when lot of real-time processing is required, think of - image & video processing, 
            3D gaming, AR/VR etc.            
        </p>
        <div style="background-color:#F5F5F5;">
        <p ><small>JavaScript is an interpreted language i.e. at the run-time, each line of code is 
            turned into machine code as it is encountered. This happens everytime, even the same line 
            of code is executed multiple times (e.g. loops). This is inefficient. And there is no way to 
            look at the whole code at once & ompimize. Also, the garbage collector cycles takes time to 
            run and free up memory, stalling the applications. Well, this is how 
            JavaScript used to run traditionally. But the modern JavaScript engines use JIT compiler 
            which partially compiles the code & has some performance improvements. But still it is not the 
            best we could get, partly because of the dynamic nature of JavaScript</small></p>
        </div> 
        <p>Web has been an important medium of user interaction for software programs, 
            and with modern SaaS & cloud applications, users are now enabled to do more & more stuffs on web. 
            Users & developers, both want to be able to do more stuffs on web, and faster. 
            And with frameworks like Electron, web apps can run like native apps on desktop, 
            cross-platform. Now, apart from JavaScript being not-so-fast, always going over the 
            network for any heavy processing & bringing back results, makes fast processing on web 
            applications almost impossible.</p>

        <p>
        Web has been an important medium of user interaction for software programs, and with modern 
        SaaS & cloud applications, users are now enabled to do more & more stuffs on web. Users & developers,
        both want to be able to do more stuffs on web, and faster. And with frameworks like Electron, web 
        apps can run like native apps on desktop, cross-platform. Now, apart from JavaScript being 
        not-so-fast, always going over the network for any heavy processing & bringing back results, 
        makes fast processing on web applications almost impossible.</p>

        <p>WebAssembly aims to solve this problem. WebAssembly is a new type of assembly language 
            (Intermediate Code) that:</p>
        <ul>
        <li>Any language compiler can target</li>
        <li>Can be executed on web, stand-alone</li>
        </ul>
        <p>With that, developers can write their code on high-performance languages like C/C++ or 
        Rust and compile them into WebAssembly. This IR code matches very closely with machine languages 
        across hardwares. Now the browser (with WebAssembly VMs) can directly translate that into machine 
        code & execute. This can make the code run real fast in near native (directly running compiled 
        machine code) speed. 
        </p>


        </div>
  </template>
  
  <script>
  
  export default {
    name: 'HomeView',
    components: {
    }
  }
  </script>
  
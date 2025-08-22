<div align="center">
    <h1>Hanriman Website</h1>
</div>

## About
This repository is the source code of https://hanriman.com website. the code is written in Rust using Yew and compiled to Webassembly.

## Getting Started
You will need a couple of tools to compile, build this website. to get started, I recommend using the official [Yew](https://yew.rs/docs/getting-started/introduction) documentation to install Rust, Webassembly target, and Trunk.

### Run the Website Locally
To run this application locally, in your terminal go to the repository directory and run this command: 
```shell
trunk serve
```
for release mode:
```shell
trunk serve --release
```

### Run the Website on Github Pages
To run this website on Github Pages, you can push this repository to your own github account. Make sure file .github/workflows/gh-pages-deploy.yml directory is included. gh-pages-deploy.yml will trigger github action to compiled the source code to Webassembly artifacts and deploy it to github page.

## Errata Policy
I am trying my best to make this website as error-free as possible. If you find any errors, please let me know by open an [issue](https://github.com/hanriman/hanriman.github.io/issues)

## Licensed Under MIT
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

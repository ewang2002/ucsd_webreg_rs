# webreg_scraper
A program designed to scrape WebReg for data.

## Programming Language
The main project (API wrapper) uses the latest version of [Rust](https://www.rust-lang.org/).

<details>
<summary>More Information</summary>
<br> 

The reason why I chose Rust instead of, say, Python or C#, is because I wanted to learn more about Rust. Plus, I've been meaning to work on a project with Rust.

There is additionally another project, creatively named `webregautoin`, which uses Node's [HTTP](https://nodejs.org/api/http.html) library to create a local API server which the wrapper can use. In particular, this local API has one sole purpose: when new cookies are needed to log into WebReg, the wrapper can make a request to the local API. The local API will then use [a headless Chrome browser](https://github.com/puppeteer/puppeteer) to log into WebReg and get the new cookies. Note that you'll need to log into WebReg beforehand, so you can select the `Remember me for 7 days` checkbox for the Duo 2FA (this will automatically be done when an initial request is made).

</details>


## Features
This program's main feature is that it can track enrollment counts and save this information to a CSV file.

Some other features that are supported include:
- Create conflict-free schedules (not exactly efficiently, but it works).
- Exports all available sections into a CSV file.
- Host a small web server (using [Rocket](https://rocket.rs/)) to make it easy for other applications to use the data.

## Webweg
Note that this program makes use of the [webweg](https://github.com/ewang2002/webweg) crate, which I developed.

## License
Everything in this repository is licensed under the MIT license.
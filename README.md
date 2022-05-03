# run-cli


<div id="top"></div>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
  </a>

  <h3 align="center">Run-cli</h3>

  <p align="center">
A run-codes cli front end with some extra features
    <br />
    <br />
    <a href="https://github.com/othneildrew/Best-README-Template/issues">Report Bug</a>
    ·
    <a href="https://github.com/othneildrew/Best-README-Template/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Run-cli is a tool to use [run.codes](https://run.codes/) from the command line.

What it can do:

- [x] Login in run codes site
- [x] Save your credentials
- [x] Select a course an a exercise
- [ ] Create a initial template from your submission type selection
  - [ ] C++
  - [ ] C
  - [ ] Haskell
  - [ ] Java
  - [x] Zip/MakeFile
  - [ ] Python
- [x] Run and compile your code
- [x] Download all test cases
- [x] Run your test cases locally
- [ ] Send your submission to run.codes
- [ ] Subscribe in new courses
- [ ] Get the input of the closed test cases
- [ ] Import projects from git/run.codes


See the [open issues](https://github.com/Math-42/run-cli/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With [Rust](https://www.rust-lang.org/)

Used libraries:

* [clap](https://crates.io/crates/clap)
* [reqwest](https://docs.rs/reqwest/latest/reqwest/)
* [rpassword](https://crates.io/crates/rpassword)
* [scraper](https://crates.io/crates/scraper)
* [html2md](https://crates.io/crates/html2md)
* [git2](https://crates.io/crates/git2)
* [zip](https://crates.io/crates/zip)
* [dialoguer](https://crates.io/crates/dialoguer)
* [chrono](https://crates.io/crates/chrono)
* [serde](https://crates.io/crates/serde)
* [toml](https://crates.io/crates/toml)
* [home](https://crates.io/crates/home)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

How to get started with the project

### Prerequisites

First you will need to install rust.

  ```sh
    curl https://sh.rustup.rs -sSf | sh npm install npm@latest -g
  ```

### Installation

1. Clone the repo

   ```sh
    git clone https://github.com/Math-42/run-cli && cd run-cli
   ```

2. Install cargo packages and build the project

   ```sh
   cargo install --path .
   ```

3. Create a project

   ```sh
   run-cli init
   ```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

Run:

   ```sh
   run-cli -h
   ```

To get a full list of possible commands.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## How to Contribute

1. Fork the Project
2. Create your Feature Branch (`git checkout -b features/feature-name`)
3. Commit your Changes (`git commit -m 'feature: Add some code'`)
4. Push to the Branch (`git push origin features/feature-name`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Your Name - [@NedLandy](https://twitter.com/NedLandy) - matheus.vieira.g@usp.br

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/othneildrew/Best-README-Template/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/othneildrew/Best-README-Template.svg?style=for-the-badge
[forks-url]: https://github.com/othneildrew/Best-README-Template/network/members
[stars-shield]: https://img.shields.io/github/stars/othneildrew/Best-README-Template.svg?style=for-the-badge
[stars-url]: https://github.com/othneildrew/Best-README-Template/stargazers
[issues-shield]: https://img.shields.io/github/issues/othneildrew/Best-README-Template.svg?style=for-the-badge
[issues-url]: https://github.com/othneildrew/Best-README-Template/issues
[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/othneildrew/Best-README-Template/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew

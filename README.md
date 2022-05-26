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
  </a>

  <h3 align="center">Run-cli</h3>

  <p align="center">
A run-codes cli front end with some extra features
    <br />
    <br />
    <a href="https://github.com/math-42/run-cli/issues">Report Bug</a>
    ·
    <a href="https://github.com/math-42/run-cli/issues">Request Feature</a>
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
- [x] Select a course and a exercise
- [x] Show a list of upcoming exercises
- [ ] Create an initial template from your submission type selection
  - [x] C++
  - [x] C
  - [ ] Haskell
  - [ ] Java 8
  - [x] Zip MakeFile
  - [x] Python 3
- [x] Download exercise descriptions
- [x] Download exercise related files
- [x] Run and compile your code
- [x] Download all test cases
- [x] Run your test cases locally
  - [ ] Run a specific test case
- [x] Generate a report for the test cases output
- [x] Send your submission to run.codes
- [ ] Show your run.codes results
- [ ] Subscribe in new courses
- [ ] Get the input of the closed test cases
- [ ] Import projects from git/run.codes

See the [open issues](https://github.com/Math-42/run-cli/issues) for a full list of proposed features (and known issues).

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
* [colored](https://crates.io/crates/colored)
* [rust-embed](https://crates.io/crates/rust-embed)

<!-- GETTING STARTED -->
## Getting Started

How to get started with the project

### Prerequisites

First you will need to install [rust](https://www.rust-lang.org/tools/install).

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

<!-- USAGE EXAMPLES -->
## Usage

Run:

   ```sh
   run-cli -h
   ```

To get a full list of possible commands.

```sh
run-cli 0.9.1
A run.codes cli front end with some extra features.

USAGE:
    run-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    build          Compile the project
    credentials    Set the user credentials
    fetch          Show the next exercises
    help           Print this message or the help of the given subcommand(s)
    import         Import a project from a git repo or from run codes
    init           Start a new project
    run            Runs the project
    secret         Get the closed test cases
    send           Submit the current project to run.codes
    subscribe      Enroll to a new class
    test           Run all test cases
    update         Update the data of the exercise
USAGE:
    run-cli <SUBCOMMAND>

```

<!-- CONTRIBUTING -->
## How to Contribute

1. Fork the Project
2. Create your Feature Branch (`git checkout -b features/feature-name`)
3. Commit your Changes (`git commit -m 'feature: Add some code'`)
4. Push to the Branch (`git push origin features/feature-name`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right"><a href="#top">back to top</a></p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/math-42/run-cli.svg?style=for-the-badge
[contributors-url]: https://github.com/math-42/run-cli/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/math-42/run-cli.svg?style=for-the-badge
[forks-url]: https://github.com/math-42/run-cli/network/members

[stars-shield]: https://img.shields.io/github/stars/math-42/run-cli.svg?style=for-the-badge
[stars-url]: https://github.com/math-42/run-cli/stargazers

[issues-shield]: https://img.shields.io/github/issues/math-42/run-cli.svg?style=for-the-badge
[issues-url]: https://github.com/math-42/run-cli/issues

[license-shield]: https://img.shields.io/github/license/math-42/run-cli.svg?style=for-the-badge
[license-url]: https://github.com/math-42/run-cli/blob/master/LICENSE.txt

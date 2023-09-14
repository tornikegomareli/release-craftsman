# ReleaseCraftsman 🛠

ReleaseCraftsman is a Rust CLI tool designed to fetch and format git commit logs between specified tags or up to the latest commit. 
This tool aims to make the release process easier and more standardized by automating the extraction of relevant git log information.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
  - [From Source](#from-source)
  - [With Cargo](#with-cargo)
  - [With Makefile](#with-makefile)
  - [With Homebrew](#with-homebrew)
  - [From Compiled Binary](#from-compiled-binary)
- [Usage](#usage)
  - [Options](#options)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)
- [TODO](#todo)

## Features

* Fetch git commit logs in either compact or full format
* Specify custom git tag as boundaries for log extraction
* Specify commit logs between two tags
* Specify commit logs from last tag to current date
* Easily integratable into CI/CD pipelines

## TODO

- [ ] ChatGPT integration for generating release notes in different file formats
- [ ] Predefined prompts
- [ ] Custom prompts
- [ ] Better error handling
- [ ] CLI prompts, interactivity

## Installation

### From Source

Clone the repository:

```bash
git clone https://github.com/tornikegomareli/releasecraftsman.git
```

Navigate into the project directory and compile.

#### With Cargo

To compile and install using Cargo, ensure you have [Cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html) on your machine. 

Then run the following commands:
```bash
cd releasecraftsman
cargo build --release
```

After building, you'll find the executable in the `target/release` directory. You'll need to include this in your PATH in your `.zshrc` or `.bashrc` file:

```bash
export PATH=$PATH:/path/to/target/release
```
Reload your shell or run 

```bash
source ~/.zshrc
```
or

```bash
source ~/.bashrc
```
for bash users, to update your PATH.

#### With Makefile

Alternatively, you can compile using the provided Makefile, it still uses cargo, so you still need to have cargo on your machine.
So you need to follow above steps.

```bash
cd releasecraftsman
make
```

This will move the `releasecraftsman` executable to `/usr/local/bin`. You can modify the Makefile to place it in a different location if you prefer.

### With Homebrew

If you're on macOS, you can also install using Homebrew:

```bash
brew tap tornikegomareli/homebrew-releasecraftsman
brew install releasecraftsman
```

### From Compiled Binary

Compiled binaries are also available for download from the [Releases](https://github.com/tornikegomareli/release-craftsman/releases) section on GitHub.

## Usage

Once installed, you can run `releasecraftsman` from the terminal to start fetching and formatting git logs. 
Below are the available options and commands.

#### 1. Default Compact Format

This will run the program using the default 'compact' format for git logs

```bash
releasecraftsman
```

#### 2. Specify a Format
Choose between 'compact' and 'hard' formats for the git logs.
```bash
releasecraftsman -f hard
releasecraftsman -f compact
# OR
releasecraftsman --format=hard
releasecraftsman --format compact
```
#### 3. Specify Start and End Tags
Fetch git logs between specified start and end tags.

```bash
releasecraftsman -s v1.0 -e v1.2
# OR
releasecraftsman --start=v1.0 --end=v1.2
```
This will defaultly formats in compact mode, if you want hard format

```bash
releasecraftsman -f hard -s v1.0 -e v1.2
# OR
releasecraftsman --format hard --start=v1.0 --end=v1.2
```

#### 4. Specify Start Tag Only
Fetch git logs from the specified start tag to the latest commit.

```bash
releasecraftsman -s v1.0
# OR
releasecraftsman --start=v1.0
```

## Contributing

### Early Development Stage

Please note that `releasecraftsman` is currently in a very early stage of development. 

### Types of Contributions

We welcome contributions of all types. Here are some ways you can contribute:

- Bug Fixes: If you find a bug, feel free to open an issue or create a pull request.
- New Features: New ideas and features are always welcome. Please open an issue for discussion before submitting a pull request.
- Code Quality: Improvements to the codebase, like optimizations or even simple clean-ups, are always appreciated.
- Documentation: Enhancements to the README or in-code documentation are fantastic and extremely helpful.

### How to Contribute

1. **Fork the Repository**: Start by forking the [ReleaseCraftsman repository](https://github.com/tornikegomareli/releasecraftsman).

2. **Clone the Fork**: Clone your forked repository onto your local machine.

    ```bash
    git clone https://github.com/your-username/releasecraftsman.git
    ```

3. **Create a New Branch**: Create a new branch on your local repository to implement your changes.

    ```bash
    git checkout -b new-feature-or-fix
    ```

4. **Implement Changes**: Make your changes to the codebase, ensuring that they are well-documented.

5. **Commit and Push**: Once you're happy with your changes, commit them and push the changes to your GitHub repository.

    ```bash
    git add .
    git commit -m "Describe your changes here"
    git push origin new-feature-or-fix
    ```

6. **Create a Pull Request**: Navigate to your repository on GitHub and click on "New Pull Request" to submit your changes for review.

### Feedback and Questions

For any questions or feedback, please open an issue on GitHub. 

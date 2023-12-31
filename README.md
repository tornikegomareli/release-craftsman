# `$ releasecraftsman ` 👷🏻‍♂️🛠

![render1695156361444](https://github.com/tornikegomareli/release-craftsman/assets/24585160/e8d8772f-abb4-4f20-a209-0216fa2ca58c)

Automate Your Release Process with Precision and Ease.

## 🎁 Features

- Generate well-crafted release notes using GPT-3.5 and GPT-4 models
- Fetch git commit logs in various formats (`compact`, `full`)
- Filter commit logs between two specified tags
- Filter commit logs from the last tag to the current date
- Seamlessly integrates with CI/CD pipelines

🦄 Prompts can be improved much more

## 🏗️ TODO

- [x] ChatGPT integration for generating release notes
- [x] Predefined prompts
- [x] Better error handling
- [X] Create different file formats, with generated content for example: Markdown
- [X] Spinner, activity indicator or some funny activity in terminal, while GPT is generating relese notes
- [X] Interactivity, step by step asking user to input data. 
- [ ] Custom prompts
- [ ] Github and Gitlab integration to create tags + releases.

## ⛑️ Usage

Once installed, you can run `releasecraftsman` from the terminal to start generating release notes.

`releasecraftsman` will ask you couple of questions based on your need, then it will generate and create markdown file for you.

If you want to run it with single command without prompts, below are available options and commands.

This current command crafts release notes, with specific version from last tag to recent commit, using GPT4 model.
```bash
releasecraftsman -f compact -s v1.0.0 -k API_KEY -m Gpt_4 -v1.0.1 
```

You can customize it and use *GPT_3_5Turbo*, you need to use your own, or companies API_KEY.

#### Options
#### 1. Default Compact Format

This will run the program using the default 'compact' format for git logs for latest 5 commit logs

```bash
releasecraftsman
```

#### 2. Specify a Format
Choose between 'compact' and 'hard' formats for the git logs.
```bash
releasecraftsman -f hard
```
#### 3. Specify Start and End Tags
Fetch git logs between specified start and end tags.

```bash
releasecraftsman -s v1.0 -e v1.2
```

This will defaultly uses compact mode, if you want hard format

```bash
releasecraftsman -f hard -s v1.0 -e v1.2
```

#### 4. Specify Start Tag Only
From Specified start tag to the latest commit.

```bash
releasecraftsman -s v1.0
```

## ☁️ Installation

### From Source

Clone the repository:

```bash
git clone https://github.com/tornikegomareli/releasecraftsman.git
cd releasecraftsman
```

#### With Cargo

To compile and install using Cargo, ensure you have [Cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html) on your machine. 

Then run the following commands:
```bash
cargo build --release
# Add to PATH
echo 'export PATH=$PATH:/path/to/target/release' >> ~/.bashrc
source ~/.bashrc
```

After building, you'll find the executable in the `target/release` directory. You'll need to include this in your PATH in your `.zshrc` or `.bashrc` file

#### With Makefile

Alternatively, you can compile using the provided Makefile, it still uses cargo, so you still need to have cargo on your machine.
So you need to follow above steps.

```bash
cd releasecraftsman
make build
```
It will compile and build

```bash
cd releasecraftsman
make install
```

It will move executable to /user/local/bin

```bash
cd releasecraftsman
make all
```

It will do both above commands in order
You can change release/debug variable inside makefile

### With Homebrew

If you're on macOS, you can also install using Homebrew:

```bash
brew tap tornikegomareli/homebrew-releasecraftsman
brew install releasecraftsman
```

Currently releasecraftsman is not on official homebrew, thats why it usess my own tap, but soon will be there.
Also it will be soon on popular package managers for different OS's, its not only designed for mac.

### From Compiled Binary

Compiled binaries are also available for download from the [Releases](https://github.com/tornikegomareli/release-craftsman/releases) section on GitHub.

## Contributing

### Early Development Stage

Please note that `releasecraftsman` is currently in a very early stage of development. 

### Types of Contributions

We welcome contributions of all types. Here are some ways you can contribute:

- Bug Fixes: If you find a bug, feel free to open an issue or create a pull request.
- New Features: New ideas and features are always welcome. Please open an issue for discussion before submitting a pull request.
- Code Quality: Improvements to the codebase, like optimizations or even simple clean-ups, are always appreciated.
- Documentation: Enhancements to the README or in-code documentation are fantastic and extremely helpful.

### 🧑‍🤝‍🧑 How to Contribute

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

### ❓ Feedback and Questions

For any questions or feedback, please open an issue on GitHub. 

# Table of Contents

1.  [Take Breath](#org1ebe878)
2.  [Algorithm](#orgb5a9440)
3.  [Roadmap](#orgd93afe6)
4.  [Building](#orgf01c00a)
5.  [Usage](#org9dff6d1)
6.  [License](#org208238b)
7.  [Contribution](#orgc1ebf01)


<a id="org1ebe878"></a>

# Take Breath

Take Breath is a program that reminds you when it is time to take a breath
from your computer.


<a id="orgb5a9440"></a>

# Algorithm

When the program starts, it starts a work time counter. When the work time
counter is greater than 45 minutes, program notifies you to take a breath and
starts a computer idle time counter. When the computer idle time counter is
less than 15 minutes, program notifies you that your rest is too short and you
should rest more. Otherwise it starts a work time counter again.


<a id="orgd93afe6"></a>

# Roadmap

-   [X] Basic algorithm implementation
-   [X] Handle idle while work
-   [ ] Split app into library and binary parts
-   [ ] CLI implementation
-   [ ] Better error handling
-   [ ] Customization features
-   [ ] Documentation
-   [-] Support for various operating systems
	-   [X] Linux (X only)
	-   [ ] Mac
	-   [ ] Windows


<a id="orgf01c00a"></a>

# Building

In order to compile Take Breath program you should have [Rust toolchain](https://www.rust-lang.org/tools/install)
installed. If you have, just run the following shell commands:

	git clone https://github.com/markmelix/take-breath.git
	cd take-breath
	cargo build --release

It puts compiled `take-breath` file into the ./target/release directory.

You can also get compiled take-breath program without cloning the repository
using the following command: "cargo install take-breath". It puts compiled
take-breath file into the ~/.cargo/bin directory.


<a id="org9dff6d1"></a>

# Usage

**Note that for now only Linux is supported by the program**

Just run the following command in a terminal: "`take-breath
  &`". It executes `take-breath` program as a background process.


<a id="org208238b"></a>

# License

Take Breath is provided under [MIT License](./LICENSE).


<a id="orgc1ebf01"></a>

# Contribution

Contributions are welcome.

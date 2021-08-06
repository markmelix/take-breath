# Table of Contents

1.  [Take Breath](#org9d2feaa)
2.  [Algorithm](#orgca37786)
3.  [Roadmap](#orgb27651a)
4.  [Building](#org884326c)
5.  [Usage](#orgb42dbdc)
6.  [License](#org5ffc384)
7.  [Contribution](#org1a25e23)


<a id="org9d2feaa"></a>

# Take Breath

Take Breath is a program that reminds you when it is time to take a breath
from your computer.


<a id="orgca37786"></a>

# Algorithm

When the program starts, it starts a work time counter. When the work time
counter is greater than 45 minutes, program notifies you to take a breath and
starts a computer idle time counter. When the computer idle time counter is
less than 15 minutes, program notifies you that your rest is too short and you
should rest more. Otherwise it starts a work time counter again.


<a id="orgb27651a"></a>

# Roadmap

-   [X] Basic algorithm implementation
-   [X] Handle idle while work
-   [X] Split app into library and binary parts
-   [ ] Customization features
-   [ ] CLI implementation
-   [ ] Better error handling
-   [ ] Documentation
-   [-] Support for various operating systems
	-   [X] Linux (X only)
	-   [ ] Mac
	-   [ ] Windows


<a id="org884326c"></a>

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


<a id="orgb42dbdc"></a>

# Usage

**Note that for now only Linux is supported by the program**

Just run the following command in a terminal: "`take-breath
  &`". It executes `take-breath` program as a background process.


<a id="org5ffc384"></a>

# License

Take Breath is provided under [MIT License](./LICENSE).


<a id="org1a25e23"></a>

# Contribution

Contributions are welcome.

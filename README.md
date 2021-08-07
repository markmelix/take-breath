# Table of Contents

1.  [Take Breath](#introdution)
2.  [Algorithm](#algorithm)
3.  [Roadmap](#roadmap)
4.  [Building](#building)
5.  [Usage](#usage)
6.  [Customization](#customization)
7.  [License](#license)
8.  [Contribution](#contribution)


<a id="introdution"></a>

# Take Breath

Take Breath is a program that reminds you when it is time to take a breath
from your computer.


<a id="algorithm"></a>

# Algorithm

When the program starts, it starts a work time counter. When the work time
counter is greater than 45 minutes, program notifies you to take a breath and
starts a computer idle time counter. When the computer idle time counter is
less than 15 minutes, program notifies you that your rest is too short and you
should rest more. Otherwise it starts a work time counter again.


<a id="roadmap"></a>

# Roadmap

-   [X] Basic algorithm implementation
-   [X] Handle idle while work
-   [X] Split app into library and binary parts
-   [X] Customization features
-   [ ] CLI implementation
-   [ ] Better error handling
-   [ ] Documentation
-   [-] Support for various operating systems
	-   [X] Linux (X only)
	-   [ ] Mac
	-   [ ] Windows


<a id="building"></a>

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


<a id="usage"></a>

# Usage

**Note that for now only Linux is supported by the program**

Just run the following command in a terminal: "`take-breath
  &`". It executes `take-breath` program as a background process.


<a id="customization"></a>

# Customization

When you first start a program, it will automatically create `take-breath`
configuration directory with config.toml file in the one of the following
directories depending on the system:

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Platform</th>
<th scope="col" class="org-left">Value</th>
</tr>
</thead>

<tbody>
<tr>
<td class="org-left">Linux</td>
<td class="org-left">$XDG<sub>CONFIG</sub><sub>HOME</sub> or $HOME/.config</td>
</tr>


<tr>
<td class="org-left">macOS</td>
<td class="org-left">$HOME/Library/Application Support</td>
</tr>


<tr>
<td class="org-left">Windows</td>
<td class="org-left">%USERPROFILE%\AppData\Roaming</td>
</tr>
</tbody>
</table>

After creation config file has the following data:

	[work_time]
	duration = '45m'
	idle_to_pause = '2m'

	[rest_time]
	duration = '15m'

This file is written in the [TOML](https://toml.io) format.
Description of each field:

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Description</th>
<th scope="col" class="org-left">Default Value</th>
</tr>
</thead>

<tbody>
<tr>
<td class="org-left">work<sub>time.duration</sub></td>
<td class="org-left">Work duration</td>
<td class="org-left">45m</td>
</tr>


<tr>
<td class="org-left">work<sub>time.idle</sub><sub>to</sub><sub>pause</sub></td>
<td class="org-left">How much time computer have to be idle to pause work time counter</td>
<td class="org-left">2m</td>
</tr>


<tr>
<td class="org-left">rest<sub>time.duration</sub></td>
<td class="org-left">Rest duration</td>
<td class="org-left">15m</td>
</tr>
</tbody>
</table>


<a id="license"></a>

# License

Take Breath is provided under [MIT License](./LICENSE).


<a id="contribution"></a>

# Contribution

Contributions are welcome.

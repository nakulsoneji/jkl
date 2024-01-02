# jkl - a simple package manager
this program is similar to portage, gentoo linux's package maanger, but with far less features and power.
## Prerequisites
1. git
2. cargo
3. rust
4. sh (does not have to be your primary shell, just make sure that `sh` is an executable on the PATH)<br>
<b>note: this software is untested on Mac or windows, but should theoretically work if you have the "HOME" environment variable set</b>
## To install:
1. Clone the repo
```sh
git clone https://github.com/nakulsoneji/jkl.git
```
2. Build the repo in release mode with cargo
```sh
cd jkl && cargo build --release
```
3. Add the `jkl` exe in target/release/ to PATH (your browser is your friend if you dont understand the steps below)</br>
<br>
Linux: <br>
simply move the exe to /usr/bin or .local/bin <br>
.local/bin is usually not on PATH by default, so make sure to add it to PATH 
```sh
sudo mv target/release/jkl /usr/bin
```
note: sudo is not needed if you are moving to .local/bin<br>
<br>
Mac:<br>
I believe that something similar to the above can be done on Mac, but I do not have a Mac machine to test this  
  
<br>Windows:<br>
Add it to whatever directory you please and add the exe to PATH
4. run `jkl init` to generate the needed directories

## To use:
### Installing:
jkl is a source-based package manager, so you will need to write your own build scripts. The rest of this section will use the installation of `bun`, a popular js runtime, package manager, and much more as an example.
To start off, create a folder in the ~/.jkl/repo directory with the folowing format: <BIN_NAME>-<BIN_VERSION>. BIN_NAME MUST be the name of the binary you install. <br>
Inside of that folder, create a file called `build.sh`

ex.
```sh
cd ~/.jkl/repo
mkdir bun-1.0.20
cd bun-1.0.20
touch build.sh
```

Next, modify build.sh to install your package. The following env variables are exported for you:<br>

BUILD_DIR - this is the directory your script will be executed in (~/.jkl/build) <br>
BIN_DIR - move your installed executable to this directory at the end (THIS IS REQUIRED, ~/.jkl/bin) <br>
SCRIPTS_DIR - this is exported so that you use any custom scripts stored in ~/.jkl/scripts <br>
V - the version (taken from folder name) <br>
P - the binary name (taken from folder name) <br>
PV - the binary and version formatted like <binary>-version> (this is the same as the folder name) <br>

ex.
```sh
# ~/.jkl/repo/bun-1.0.20/build.sh
MY_PV="bun-v${V}"
DOWNLOAD_URL="https://github.com/oven-sh/bun/releases/download/${MY_PV}/bun-linux-x64.zip"
PKG_ARCHIVE="bun-linux-x64"

# build
wget -q --show-progress $DOWNLOAD_URL 2>&1
unzip "${PKG_ARCHIVE}.zip"
mv "${PKG_ARCHIVE}/${P}" $BIN_DIR
rm -rf ${BUILD_DIR}/*
```
note: this technically just installs a binary and does not build it from source, but building it from source would still work with the package manager<br>
Notice how I am using the environment variables for the github source code link. This will make updates much easier later.<br>
Now, install with `jkl install <BIN_NAME>`. The script will automatically terminate if there is an error. <br>
note: if you use `wget`, make sure you run it with 2>&1 or -q, as `wget` logs all output to stderr. This will trigger `jkl` to stop running and output an error.

### Updating:
This package manager does not rely on community maintained repos like `apt`, `pacman`, or `choco`. To update, change the folder version and run `jkl update <BIN_NAME>`<br>
note: changing the folder verison will only work if you used the env vars for the version, and if build instructions haven't changed since the last release of the package. If so, then you willhave to make those changes manually.<br>

ex. 
If bun updated from 1.0.20 to 1.0.21, I would run the following commands:
```
mv ~/.jkl/repo/bun-1.0.20 ~/.jkl/repo/bun-1.0.21
jkl update bun
```

### Deleting:
Run `jkl delete <BIN_NAME>`. This will delete it from the database and delete the binary, but will not delete the repo folder. This is for easier reinstall, and you can simply just run `jkl install <BIN_NAME>`. Also, you can just manually delete the repo folder if you want with `rm -r`. Support for the `jkl` to do this is coming soon!

### List:
Run `jkl list` to list all installed binaries and version with a count of installed packages.<br>
sample output:<br>
![jkl list example](https://github.com/nakulsoneji/jkl/assets/98666847/86280aa8-1e39-443d-9068-48b727fb2391)


### Other:
This isn't meant to be a complete solution or replacement for anything, I just made it for fun. Expect some bugs and inconveniences that I may or may not fix in the future :)

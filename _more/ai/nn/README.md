# Neural Network -- Rust

* https://srenevey.github.io/neuro/
    * https://crates.io/crates/neuro
    * 這個套件使用了 ArrayFire 套件，必須先安裝


安裝

* https://arrayfire.com/download/

To use the rust bindings for ArrayFire from crates.io, the following requirements are to be met first.

Download and install ArrayFire binaries based on your operating system.
Set the evironment variable AF_PATH to point to ArrayFire installation root folder.
Make sure to add the path to lib files to your path environment variables.

* On Linux: do export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib64
* On OSX: do export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$AF_PATH/lib64
* On Windows: Add %AF_PATH%\lib to your PATH environment variable.

Add arrayfire = "3.7" to the dependencies section of your project's Cargo.toml file. Make sure to change the version to latest available.

## Installation through Source Code

To be able to build and run the project, you need to have Rust installed:
https://www.rust-lang.org/tools/install

```
git clone https://github.com/paultimke/SoftcoreCPU.git

cd Assembler
cargo build

cd ../Emulator
cargo build
```

The binaries of the emulator and assembler are found in the target/release directory.

## Running the example programs
There is one example assembly file called `file1.s` on the `Assembler/test/` directory. The
purpose of this program is to sum the elements of an array. You can specify
the length of the array by giving it as a parameter in line 10 on register `r1`.

To assemble the program, go to the Assembler directory and run the program with 
the file as arguement.
```
cd Assembler
./target/release/assembler test/file1.s
```
This will generate the binary file `out.bin`. You can now run this program with the emulator as such:
```
mv out.bin ../Emulator
cd ../Emulator
./target/release/emulator -DEBUG out.bin
```
By passing the `-DEBUG` flag, you can step through the code line by line and see registers change.
If you want to just run the program and see the output, run it without this flag.
On the image below, the example program `test/file1.s` (left image) was run with the emulator. On the right,
we can see the output of the program after running.
<img width="957" alt="example" src="https://github.com/paultimke/SoftcoreCPU/assets/87957114/78e38da3-f4b8-490f-9a6e-49097f5c7534">


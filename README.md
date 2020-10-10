
For crosscompile to windows (x64) from ubuntu you have to install:
`sudo apt install mingw-w64`
and add linker:
`rustup target add x86_64-pc-windows-gnu`

You must check installed linkers:
`rustup target list`

Then just run:
`cargo build --target x86_64-pc-windows-gnu --release`

Game creating try on rust

For crosscompile to windows (x64) from ubuntu you have to install:
`sudo apt install mingw-w64`
Then you must add linker:
`rustup target add x86_64-pc-windows-gnu`
You can check installed linkers:
`rustup target list`

Then just run:
`cargo build --target x86_64-pc-windows-gnu --release`

(developed by tutorial: https://sokoban.iolivia.me/)
## Run 
`cargo run -- <path> <filename> <extension>`

## Run Example
`cargo run -- screen homeScreen tsx`

`cargo run -- atom homeAtom ts`

`cargo run -- component homeComponent tsx`

## Roadmap

This is early development version. I am currently considering:
- [x] Parse 3 arguments `action` and `filename` and `extension`.
- [x] implement `cargo run -- screen homeScreen tsx` command.
- [x] break code into functions.
- [x] create any file.
- [x] verify if path exist before creation.
- [x] create any file but check if file exist.
- [x] write exemple string to the file.
- [ ] verify if path exist before creation (look for other solutions and compare if there is a better one).
- [ ] implement template for `tsx` and `ts`.
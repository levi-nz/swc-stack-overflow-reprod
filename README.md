# swc-stack-overflow-reprod
A stack overflow error re-produced when attempting to parse JS with SWC.

Note that this error only seems to appear on Windows when running in debug mode.
When compiling the application in release mode, the error does not occur.
The error is also not produced on macOS, and presumably other Unix-based systems as well.

## Usage
Simply run:\
`cargo run`

This will read the JavaScript from `input.js` and attempt to save the processed output
to `output.js`.
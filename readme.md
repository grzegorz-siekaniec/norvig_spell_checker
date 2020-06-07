Crate contains implementation of simple spell checker proposed by Norvig. See https://norvig.com/spell-correct.html.

The idea woudl be to play with Rust and see how it can be implemented with the focus on:
1. ~~Handling files~~
2. Data manipulation code.
3. ~~Adding CLI interface.~~
4. Using crate in Python - Foreign Function Interfaces (FFI) and see how easy/hard it is.
5. Convert counter into generic
6. Microbenchmarking
7. Parallelization - TBB pipeline style.
8. ~~Multiple asserts in unit tests~~
9. ~~Reading form file - nice, Use a pipe there to read line and pass to counter~~
10. Make it API.
11. Add subcommand:
    a. to pass corrections and output in cmd
    b. to start server and process requests.
    
12. Build docker image
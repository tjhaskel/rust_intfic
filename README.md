# intfic

intfic is a framework that allows you to write a branching story with minimal code.
It uses story files with a custom markup syntax that allows for the following:

* Write text or specific quotes of text with different colors
* Display options that trigger different Story Blocks or Story Files
* Set flags or add to counters in the GameState
* Check flags or counters in the GameState and conditionally display text or options

Additionally, I've included some basic functions for asking yes-no questions and traveling in the cardinal directions, should you prefer to take a more "text adventure" approach with code.

## Getting Started

1. Run the example with "cargo run"
2. Examine the example story files and read up on the [intfic Story File Markup Specification](https://docs.rs/intfic/0.3.6/intfic/parse_file/index.html#story-file-markup-specification)
3. Write you own story, and update main.rs to start it!

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

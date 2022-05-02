# MDBook Preprocessor for Vlang

A preprocessor parsing MDBook compatible with Vlang to enable preproccessing in Vlang

The goal of this project is to allow preprocessing with Vlang by formatting deserialized mdbook, and sending encoded object via rmb. The processed book is then received via rmb and formatted back to MDBook conventions ready for compilation.

## Work in progress

Currently one can format MDBooks into Vlang compatible structs.

The purpose of this project is to overcome the incompatibilities between rust and vlang so that once can process MDBook objects in v.

Right now, formatted book objects can be accessed via v over redis for processing. However, Reliable Message Bus will be used for messaging between rust and v which is currently under development.

In future implementations this project will be shipped as a cargo package to enable easy mdbook preprocessing.

## Running the preprocessor

Simply create a folder with md files in this project, and add the following to book.toml

```

[preprocessor.vpreprocessor]
command = "cargo run"

```

Then you can run mdbook build, and the book will be outputted to the docs directory
See /example for implementation.

Robert Masen's [presentation preprocessor](https://github.com/FreeMasen/mdbook-presentation-preprocessor) has been used to start this project.

# App Functionality

1. Can work with git, clone, commit, push, pull, etc.
2. Can process the kdl templates (replace the variables and place holders with the real values...) and write them in the correct form (other formats like toml, yml, json...).
3. Can detect events and perform actions like change the theme, install the themes, generate files...

# App Design

### Modules

| Module        | Description                                    |
| ---           | ---                                            |
| git           | with with git, clone, commit, push, pull, etc. |
| parser        | parse the kdl templates preparing them         |
| formatter     | convert files from format a to format b        |
| linker        | link (simlinks) files                          |
| event         | detect events                                  |
| script-engine | run scripts                                    |

### Details

#### event

events kinds:
    - script - user run a pre-defined specific script via cli for e.g: `cli script change-theme`.
    - git - after/before performing a git action.
    - hook - after/before performing a command.

#### scripts

it's just a executable files that can be written in any language, if the engine is localy available or the file is in binary form.

#### formatter

should contain a from/to toml, yml, yaml, json, xml, plian text (just write it), kdl converters.

#### linker

use the unix simlinks.

#### parser

use the `??var` placeholder.

### App architecture

description: the app should act with a directory called `.dotfiles` on the local file system, this dir contains two main thing:

- resources - which is 3 categories, each category has a sub-dir:
    - templates - contains the kdl templates that may has a placeholders.
    - data - kdl files contains the data that will be used to replace the placeholders.
    - scripts - scripts that will be run on events.
- the `Dotfile.kdl` - inspired by docker and Dockerfile, with is the entry point and the most important file out there which the file contains the recipe of all the linking, installing, generating, converting..., it's provide a set of commands that use can use to do what he want to do.

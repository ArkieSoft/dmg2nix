# dmg2nix
## A script that converts dmg archives and makes a nix derivation

A simple script built in Rust that takes the download URL for a dmg archive, mounts it, analyzes it, then spits out a .nix derivative.
It isn't perfect, and this is an initial release with a work in progress. I am making no roadmap as this is being developed only until I am satisfied, or lose interest.

There are several problems as I am not all that experienced in Rust and will work more on it as I gain proficiency. This is a learning project with a practical use.

## Issues
*Only spits out a target.nix file
*Doesn't produce a correct hash. So I intend on fixing this, in the meantime, add the hash after the error like everyone else does  

## Install
It is currently on crates.io which can be installed with this command:  
```
cargo install dmg2nix
```
There is also a flake in this repo, simply (callPackage /Path/To/dmg2nix.nix {}) to install as a package.

## Contribute?
I am a dum-dum in coding and would be happy to accept any contributions.

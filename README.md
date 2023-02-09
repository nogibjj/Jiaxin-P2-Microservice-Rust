# Jiaxin-P2-Microservice-Rust

## Key Objectives of Project
In project 2, the purpose is to build a functional Web Microservice in Rust based on Kubernetes or other similar platforms (Kubernetes Deployments provide a higher-level abstraction for managing the deployment and scaling of applications in a cluster, making it easier to automate and manage the process).
I create a simple actix Microservice for movie buffs which can be used for randomly choosing a good classic movie at leisure based on the list of the world top 10 best movies. 

This actix Microservice has multiple routes:
A. type: "/" that returns a message : "Hello, random best movies around the world!"
B. type: "/movie" that returns a random best movie in the list of the world top 10 best movies
C. type: "/version" that returns the version of the service 

## Structure Diagram


## Demo Video Link


## Preparation
### 1. Containerization: Setup virtual environment
A virtual environment is a tool that helps to keep dependencies required by different projects separate by creating isolated python virtual environments for them. 
* Type: `python3 -m venv env` and `source env/bin/activate`

### 2. Instal Rust
* Type: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` and then `source "$HOME/.cargo/env"`

### 3. create new project
* Type: `cargo new (project name)` (my Eg: `cargo new src`)
* Create main.rs and lib.rs for the src project
* Cargo build: it is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The `cargo build` command can be run from the root directory of the project. It uses the information specified in the project's Cargo.toml file to determine the dependencies and build configuration of the project.



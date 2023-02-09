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
* Cargo build: it is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The `cargo build` command can be run from the root directory of the project.
* Set up Cargo.toml to determine the dependencies and build configuration of the project.
<img width="728" alt="Screen Shot 2023-02-09 at 1 42 11 PM" src="https://user-images.githubusercontent.com/112274822/217907444-fb11682a-6699-493b-a945-17fd73c8888a.png">

* Set up Dockerfile for APP webdocker
<img width="489" alt="Screen Shot 2023-02-09 at 1 45 39 PM" src="https://user-images.githubusercontent.com/112274822/217908173-d0f572ee-847a-4be6-b1fc-18108e0d62c9.png">

* Create a Makefile: it is a special file that lists a set of rules for compiling a project. These rules include targets, which can be an action make needs to take (eg. "clean" or "build") or the files/objects make will need to build, and the commands that need to be run in order to build that target. 
<img width="354" alt="Screen Shot 2023-02-09 at 1 51 30 PM" src="https://user-images.githubusercontent.com/112274822/217909371-9e17b610-ec15-47dc-9a0b-eff5163b287b.png">

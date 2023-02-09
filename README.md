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

* Create a Makefile: it is a special file that lists a set of rules for compiling a project. These rules include targets, which can be an action make needs to take or the files/objects make will need to build, and the commands that need to be run in order to build that target. 
<img width="354" alt="Screen Shot 2023-02-09 at 1 51 30 PM" src="https://user-images.githubusercontent.com/112274822/217909371-9e17b610-ec15-47dc-9a0b-eff5163b287b.png">

## Run Microservice
### 1. Use `make format` and `make lint` to fix code format and check code errors
<img width="544" alt="Screen Shot 2023-02-09 at 2 06 34 PM" src="https://user-images.githubusercontent.com/112274822/217913027-44f99e34-1d47-4197-a37a-9ffc3a13ed7d.png">

### 2. Run web microservice 
* After creating main.rs and lib.rs, in terminal, directly type: `cargo run` (Press CTRL+C to quit)
<img width="515" alt="Screen Shot 2023-02-09 at 2 07 26 PM" src="https://user-images.githubusercontent.com/112274822/217913210-375631a1-f8c3-4ce4-9a0a-8945059485ab.png">

* Usage of an example (Test the URL): https://helenyjx-ubiquitous-guide-v957g7j5xjqfvgx-8080.preview.app.github.dev/movie

* A. type: "/" that returns a message : "Hello, random best movies around the world!"
<img width="642" alt="Screen Shot 2023-02-09 at 2 01 26 PM" src="https://user-images.githubusercontent.com/112274822/217912099-51c491d1-59f3-4006-aaa8-c1f38ef4598c.png">

* B. type: "/movie" that returns a random best movie in the list of the world top 10 best movies
<img width="697" alt="Screen Shot 2023-02-09 at 2 02 33 PM" src="https://user-images.githubusercontent.com/112274822/217912257-f90b27dd-ad64-4bb3-8746-f6419320cac7.png">

* C. type: "/version" that returns the version of the service 
<img width="701" alt="Screen Shot 2023-02-09 at 2 04 38 PM" src="https://user-images.githubusercontent.com/112274822/217912657-35ce00a3-cadd-4251-808a-c367e0e5c1c6.png">

## Use automatic deployment platforms to deploy the project
### A. Via AWS APP Runner

1. Go to github and then copy the http link for cloning my repo of the project 2

<img width="993" alt="Screen Shot 2023-02-09 at 4 32 23 PM" src="https://user-images.githubusercontent.com/112274822/217944275-01f1edb5-41ab-4be3-9db3-674cfce23fa8.png">

2. Go to AWS Cloud9, then click "Create environment"
<img width="1039" alt="Screen Shot 2023-02-09 at 4 35 43 PM" src="https://user-images.githubusercontent.com/112274822/217944516-734f0e95-e0cc-4392-9216-e4181919b2d2.png">

* In the terminal, copy the clone link and type : `git clone https://github.com/nogibjj/Jiaxin-P2-Microservice-Rust.git`


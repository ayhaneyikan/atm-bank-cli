# ATM & Bank

Note: still very much in-progress

- [Introduction](#introduction)
- [Project Execution](#running-the-project)
- [ATM Discussion](#atm)
- [Bank Discussion](#bank)
- [Message Design](#message-design)

## Introduction

This is project is an exploration of secure TCP communication between separate services.
The focal point of the project is the `bank` module which is a server that contains rudimentary information about users' accounts.
The server will spawn threads to handle an arbitrary number of clients.
The `ATM` module is the client service which connects to the bank and enables a user to make various requests reading or updating the bank state.

The roots of this project were from a network security class during my undergrad.
Groups were tasked with creating a similar Bank and ATM setup which would seek to secure it's data transfers as much as possible.
And while of course no groups could possibly avoid all attack types during the second stage of the project, my group did quite well.
That original assignment was written in C and while I was working on it, I felt compelled to try it in a language I liked a little more.
I also noticed several places where the assignment required designs which intentionally left room for security errors.
This project is my attempt to design a similar but improved ATM and Bank CLI which also has expanded functionality.

## Running the Project

In order to see the project in action, follow the steps below.

1. Clone the repository to a machine with [rust installed](https://www.rust-lang.org/tools/install).
2. Build the project: `cargo b`
3. Run the bank server in one terminal window: `cargo r --bin bank`
4. Run at least one instance of the ATM in another window: `cargo r --bin atm`
5. Explore interactions using the available commands.
   - Begin by creating a user account utilizing the bank comandline
   - After at least one account has been created, you can utilize an ATM instance to authenitcate as that user and view/modify the user's balance remotely.

## ATM

## Bank

## Message Design

As of 1/21/2024 messages between clients and server are not being encrypted.
Consideration has begun for this already though.
A lot of which began based off of the original work done for this project.
For a detailed look at some of the design decisions that went into message definitions, see [the README within `common/`](./common/src/README.md).
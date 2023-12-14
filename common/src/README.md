# Message Structure

## Overview

To serve as a reference guide when sending and receiving messages between the bank and an ATM instance.
With the goal of securing TCP communications between the two, there are a couple design decisions that have been made.

1. Messages will be encrypted using a public key encryption scheme.
    - the goal being to deny a potential adversary the ability to flat out read messages.
    - this is the most basic measure that can be taken, especially if pretending to design a bank.
2. Messages send by any party will always be the same fixed length. (e.g. messages will always be 25 bytes)
    - the goal is to deny an adversary the ability to distinguish between message *types* based on length.
    - e.g. an attacker would be unable to tell if a user just requested to check their balance or if they requested to withdraw $1000.
3. The first byte of the message is a counter set upon sending the message.
    - the goal is to prevent replay attacks. Each party compares the counter in the received message against their own internally maintained counter. If there is a mismatch, the message is discarded.
4. This design currently does not protect against drop attacks.

## Message Types

### General Message (bi-directional)

| byte #    | purpose |
| --------- | ------- |
| 0         | message counter |
| 1         | message request type |
| 2-25      | message body |

### Authenticate User

After user attempts to begin a session in an ATM, the ATM must first confirm with the bank that the user exists and has the correct PIN.

`RequestType::AuthUser = 0`

#### ATM

| byte #    | purpose |
| --------- | ------- |
| 0         | message counter |
| 1         | message request type |
| 2-21      | username up to 20 characters |
| 22-25     | pin exactly 4 characters |

#### Bank

| byte #    | purpose |
| --------- | ------- |
| 0         | message counter |
| 1         | message request type |
| 2         | authentication success indicator |
| 3-25      | garbage values - ignored |

### Check Balance

ATM retreival of user balance from the bank

`RequestType::Balance = 1`

#### ATM

| byte #    | purpose |
| --------- | ------- |
| 0         | message counter |
| 1         | message request type |
| 

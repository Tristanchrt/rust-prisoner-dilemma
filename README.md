# Prisoner Dilemma game - RUST - TCP - GUI (SLINT)


## Overview

The project demonstrates the classic game theory scenario of the Prisoner's Dilemma, allowing multiple players to interact and make decisions that affect their game strategies.

### Game Rules

* Each player has two choices: cooperate or defect.
* If both players cooperate, they both receive a moderate reward.
* If both players defect, they both receive a small reward.
* If one player cooperates and the other defects, the defector receives a higher reward, and the cooperator receives nothing.


## Features

* **TCP Communication** : Implements a server-client architecture for multiplayer gameplay using TCP sockets.
* **Slint GUI** : Integrates the Slint framework to create an intuitive graphical user interface for players to make decisions and observe game outcomes.
* **Prisoner's Dilemma Logic** : Implements the game logic, including the decision-making process based on strategies chosen by players.


## Usage

1. **Clone the Repository**
2. **Build and Run**
3. **Using the Application**
4. **Run Test**

## Code Structure

- back : Logic of the game works with mutlithreading for handle multiple tcp connection at the same time
- settings: Game logic, communication protocol, 
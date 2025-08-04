# FrostWire - Linux Firewall + Packet Inspector (CLI)

A lightweight Linux firewall and packet inspection tool implemented as a CLI application using **Rust** for high-performance packet sniffing and **Bash** for firewall configuration.

---

## Overview

This project combines a minimal Linux firewall setup with a real-time packet inspector running in the terminal. The Rust component uses the [`pnet`](https://github.com/libpnet/libpnet) library to capture and display network packet details, while the Bash script sets up basic firewall rules using `iptables` and launches the Rust inspector.

---

## Features

- **Packet Inspection**  
  - Sniffs Ethernet frames on the default non-loopback network interface  
  - Parses IPv4 and TCP headers  
  - Displays source/destination IP addresses and TCP ports for captured packets

- **Firewall Setup**  
  - Simple firewall rules management via a Bash script  
  - Flush existing iptables rules on startup  
  - Blocks inbound TCP traffic on a specific port (default: 8080)  
  - Allows localhost traffic and established connections  

---

## Prerequisites

- Linux system with `iptables` installed  
- Rust toolchain (https://rustup.rs)  
- Permissions to configure firewall (run script with `sudo` or as root)  

---

## Installation & Usage

1. Clone the repository or create a new Rust project.



bash
Copy
Edit
sudo ./scripts/firewall.sh

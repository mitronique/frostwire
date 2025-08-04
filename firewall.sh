#!/bin/bash


set -e

echo "Resetare reguli iptables..."
sudo iptables -F
sudo iptables -X

echo "blocking inbound trafic on port 8080..."
sudo iptables -A INPUT -p tcp --dport 8080 -j DROP

echo "permiting traffic to input"
sudo iptables -A INPUT -i lo -j ACCEPT
sudo iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT

echo "iptables rules:"
sudo iptables -L -v

echo "packet inspector is launching"
cargo run --release

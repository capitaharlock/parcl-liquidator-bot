#!/bin/bash

# Infinite loop
while true; do
  # Execute the Solana airdrop command
  solana airdrop 3 A8u6cMJkpbyrdGse7tvgm4ZtZvJPvMZsTsBpa1LQAmMy --url https://api.devnet.solana.com
  
  # Wait for one hour (3600 seconds)
  sleep 4000
done

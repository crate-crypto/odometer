#!/bin/sh
echo "Running custom entrypoint script..."

# Define variables
GENESIS_FILE="/config/genesis.json"
DATA_DIR="/data"
NETWORK_ID=1700
JWT_SECRET_FILE="/config/jwt.hex"

# Import the genesis block
# TODO: Check perf!
rm -rf /data
geth init --datadir $DATA_DIR $GENESIS_FILE 

# # Start the geth node
geth  --syncmode=full \
      --nodiscover \
      --datadir=$DATA_DIR \
      --networkid=$NETWORK_ID \
      --authrpc.addr=0.0.0.0 \
      --authrpc.port=8551 \
      --authrpc.vhosts=* \
      --authrpc.jwtsecret=$JWT_SECRET_FILE
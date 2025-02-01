#!/bin/sh
echo "Running custom entrypoint script..."

# Import the genesis block
# TODO: Check perf!
rm -rf /data

# Execute the init command.
# The init command is responsible for setting up the genesis state.
if [ -n "$INIT_COMMAND" ]; then
    eval "$INIT_COMMAND"
fi

# Execute the command in RUN_COMMAND variable.
# The RUN_COMMAND is responsible for starting the node.
if [ -n "$RUN_COMMAND" ]; then
    eval "$RUN_COMMAND"
fi
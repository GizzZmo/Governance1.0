#!/bin/bash

# Governance 1.0 Deployment Script
# This script deploys all contracts in the modular governance suite.

set -e

echo "Deploying Governance 1.0 contracts..."

# Replace these with your Move CLI or Sui CLI commands as appropriate
MOVE_CLI="move"
PACKAGE_PATH="./contracts"

echo "Deploying governance_token..."
$MOVE_CLI publish $PACKAGE_PATH/governance_token.rs

echo "Deploying governance..."
$MOVE_CLI publish $PACKAGE_PATH/governance.rs

echo "Deploying delegation-staking..."
$MOVE_CLI publish $PACKAGE_PATH/delegation-staking.rs

echo "Deploying treasury..."
$MOVE_CLI publish $PACKAGE_PATH/treasury.rs

echo "Deploying proposal-handler..."
$MOVE_CLI publish $PACKAGE_PATH/proposal-handler.rs

echo "All contracts deployed successfully!"

# Optionally run tests
echo "Running tests..."
$MOVE_CLI test ./tests

echo "Deployment and testing complete."

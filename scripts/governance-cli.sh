#!/bin/bash

# Governance CLI Script
# Provides simple CLI interface for interacting with Governance contracts

COMMAND=$1
shift

case $COMMAND in
  submit-proposal)
    CREATOR=$1
    DESC=$2
    TYPE=$3
    move call --function Governance::submit_proposal --args $CREATOR "$DESC" $TYPE
    ;;
  vote)
    PROPOSAL_ID=$1
    VOTER_ADDR=$2
    VOTES=$3
    SUPPORT=$4
    VETO=$5
    move call --function Governance::hybrid_vote --args $PROPOSAL_ID $VOTER_ADDR $VOTES $SUPPORT $VETO
    ;;
  execute)
    PROPOSAL_ID=$1
    EXECUTOR=$2
    move call --function Governance::execute_proposal --args $EXECUTOR $PROPOSAL_ID
    ;;
  *)
    echo "Unknown command. Available: submit-proposal, vote, execute"
    ;;
esac

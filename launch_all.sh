#!/bin/bash
killjobs() {
    for x in $(jobs | awk -F '[][]' '{print $2}' ) ; do 
        kill %$x
    done
}
trap killjobs EXIT

set -a # automatically export all variables
source .env
set +a

for f in ./config/*.env; do
    target/${BRANCH}/abra_liquidator config/`basename $f` &
done;

## Arbitrum
#set -a # automatically export all variables
#source .env.arbitrum
#set +a

#for f in ./config.arbitrum/*.env; do
#    target/${BRANCH}/abra_liquidator config.arbitrum/`basename $f` &
#done;

## BSC
#set -a # automatically export all variables
#source .env.bsc
#set +a
#
#for f in ./config.bsc/*.env; do
#    target/${BRANCH}/abra_liquidator config.bsc/`basename $f` &
#    sleep 1
#done;

wait
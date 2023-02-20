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

target/${BRANCH}/abra_liquidator config/${1}
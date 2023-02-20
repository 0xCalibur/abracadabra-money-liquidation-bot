-include .env
export

SCRIPT_DIR = ./script
FILES_CONTAINING_CONSOLE := $(shell grep -rlw --max-count=1 --include=\*.sol 'contracts' -e 'console\.sol')
FILES_CONTAINING_CONSOLE2 := $(shell grep -rlw --max-count=1 --include=\*.sol 'contracts' -e 'console2\.sol')

.DEFAULT_GOAL := build

clean:
	cargo clean
	forge clean
rebuild: clean build
build-rs:
	cargo build
build-rs-release:
	cargo build --release
build-contracts:
	forge build
build: build-rs build-contracts
build-release: build-rs-release build-contracts
install: init
init:
	git submodule update --init --recursive
	forge install

test:
	forge test -vv
test-gas-report:
	forge test -vv --gas-report
trace:
	forge test -vvvv 

remappings:
	forge remappings > remappings.txt

check-console-log:
ifneq ($(FILES_CONTAINING_CONSOLE),)
	$(error $(FILES_CONTAINING_CONSOLE) contains console.log)
endif
ifneq ($(FILES_CONTAINING_CONSOLE2),)
	$(error $(FILES_CONTAINING_CONSOLE2) contains console2.log)
endif

deploy-simulation: check-console-log
	$(foreach file, $(wildcard $(SCRIPT_DIR)/*.s.sol), \
		echo "Simulating $(file)..."; \
		forge script $(file) --rpc-url $(rpc) --private-key $(pk) -vvvv; \
	)

deploy: check-console-log
	$(foreach file, $(wildcard $(SCRIPT_DIR)/*.s.sol), \
		echo "Running $(file)..."; \
		forge script $(file) --rpc-url $(rpc) --private-key $(pk) --broadcast -vvvv; \
	)

## Mainnet
mainnet-deploy-simulation: rpc:=${MAINNET_RPC_URL}
mainnet-deploy-simulation: pk:=${PRIVATE_KEY}
mainnet-deploy-simulation: deploy-simulation
mainnet-deploy: rpc:=${MAINNET_RPC_URL}
mainnet-deploy: pk:=${PRIVATE_KEY}
mainnet-deploy: deploy

## Arbitrum
arbitrum-deploy-simulation: rpc:=${ARBITRUM_RPC_URL}
arbitrum-deploy-simulation: pk:=${PRIVATE_KEY}
arbitrum-deploy-simulation: deploy-simulation
arbitrum-deploy: rpc:=${ARBITRUM_RPC_URL}
arbitrum-deploy: pk:=${PRIVATE_KEY}
arbitrum-deploy: deploy

## Arbitrum
optimism-deploy-simulation: rpc:=${OPTIMISM_RPC_URL}
optimism-deploy-simulation: pk:=${PRIVATE_KEY}
optimism-deploy-simulation: deploy-simulation
optimism-deploy: rpc:=${OPTIMISM_RPC_URL}
optimism-deploy: pk:=${PRIVATE_KEY}
optimism-deploy: deploy

## Fantom
bsc-deploy-simulation: rpc:=${BSC_RPC_URL}
bsc-deploy-simulation: pk:=${PRIVATE_KEY}
bsc-deploy-simulation: deploy-simulation
bsc-deploy: rpc:=${BSC_RPC_URL}
bsc-deploy: pk:=${PRIVATE_KEY}
bsc-deploy: deploy

## Local
_local-deploy:
	$(foreach file, $(wildcard $(SCRIPT_DIR)/*.s.sol), \
		forge script $(file) --silent --rpc-url $(rpc) --private-key $(pk) --broadcast; \
	)
local-deploy: rpc:="http://127.0.0.1:8545"
local-deploy: pk:=${LOCAL_PRIVATE_KEY}
local-deploy: USE_ANVIL:=1
local-deploy: _local-deploy
	-RUST_BACKTRACE=1 cargo run

start-anvil:
	-@killall -9 anvil
	anvil --fork-url ${MAINNET_RPC_URL} --fork-block-number 14759615 &
	sleep 3

local: start-anvil local-deploy
	-@killall -9 anvil

.PHONY: test test-archives check-console-log check-git-clean gen run
.SILENT: deploy-simulation deploy
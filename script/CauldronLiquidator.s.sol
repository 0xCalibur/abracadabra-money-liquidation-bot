// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "utils/BaseScript.sol";
import "/CauldronLiquidator.sol";
import "/FTTSwapper.sol";

contract CauldronLiquidatorScript is BaseScript {
    function run() public returns (CauldronLiquidator liquidator) {
        vm.startBroadcast();

        liquidator = new CauldronLiquidator();

        vm.stopBroadcast();
    }
}

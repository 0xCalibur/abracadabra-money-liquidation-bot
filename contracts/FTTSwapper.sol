// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import "BoringSolidity/interfaces/IERC20.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import "interfaces/IBentoBoxV1.sol";
import "./ISwapperV1.sol";
import "./ICurvePool.sol";

interface Tether {
    function approve(address spender, uint256 value) external;

    function balanceOf(address user) external view returns (uint256);

    function transfer(address to, uint256 value) external;
}

contract FTTSwapper is ISwapperV1 {
    ICurveThreeCryptoPool public constant TRICRYPTO = ICurveThreeCryptoPool(0xD51a44d3FaE010294C616388b506AcdA1bfAAE46);
    ICurvePool private constant MIM3POOL = ICurvePool(0x5a6A4D54456819380173272A5E8E9B9904BdF41B);
    IBentoBoxV1 public constant bentoBox = IBentoBoxV1(0xd96f48665a1410C0cd669A88898ecA36B9Fc2cce);
    IERC20 public constant FTT = IERC20(0x50D1c9771902476076eCFc8B2A83Ad6b9355a4c9);
    IERC20 public constant MIM = IERC20(0x99D8a9C45b2ecA8864373A26D1459e3Dff1e17F3);
    ISwapRouter public constant SWAPROUTER = ISwapRouter(0xE592427A0AEce92De3Edee1F18E0157C05861564);
    Tether public constant USDT = Tether(0xdAC17F958D2ee523a2206206994597C13D831ec7);
    IERC20 public constant WETH = IERC20(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);

    constructor() {
        FTT.approve(address(SWAPROUTER), type(uint256).max);
        USDT.approve(address(MIM3POOL), type(uint256).max);
        MIM.approve(address(bentoBox), type(uint256).max);
        WETH.approve(address(TRICRYPTO), type(uint256).max);
    }

    function swap(
        address,
        address,
        address recipient,
        uint256 shareToMin,
        uint256 shareFrom
    ) public override returns (uint256 extraShare, uint256 shareReturned) {
        (uint256 amountOut, ) = bentoBox.withdraw(IERC20(address(FTT)), address(this), address(this), 0, shareFrom);

        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter.ExactInputSingleParams({
            tokenIn: address(FTT),
            tokenOut: address(WETH),
            fee: 3000,
            recipient: address(this),
            deadline: type(uint256).max,
            amountIn: amountOut,
            amountOutMinimum: 0,
            sqrtPriceLimitX96: 0
        });

        amountOut = SWAPROUTER.exactInputSingle(params);
        TRICRYPTO.exchange(2, 0, amountOut, 0);
        amountOut = MIM3POOL.exchange_underlying(3, 0, USDT.balanceOf(address(this)), 0, address(bentoBox));
        (, shareReturned) = bentoBox.deposit(MIM, address(bentoBox), recipient, amountOut, 0);
        extraShare = shareReturned - shareToMin;
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.16;

import {IFunctionGateway} from "../../interfaces/IFunctionGateway.sol";

library Secp256r1 {
    /// @dev Requests for a Secp256r1 signature to be verified.
    /// @param _gateway The gateway to the function.
    /// @param _functionId The id of the function.
    /// @param _r The r value of the signature.
    /// @param _s The s value of the signature.
    /// @param _callbackSelector The selector of the callback function.
    /// @param _context The context of the runtime.
    function request(
        address _gateway,
        bytes32 _functionId,
        uint256 _r,
        uint256 _s,
        bytes4 _callbackSelector,
        bytes memory _context
    ) internal returns (bytes32) {
        bytes memory input = abi.encode(_r, _s);
        return IFunctionGateway(_gateway).request(_functionId, input, _callbackSelector, _context);
    }

    /// @dev Decodes the output of the function.
    /// @param _output The output of the function.
    function decode(bytes memory _output) internal pure returns (bool) {
        return abi.decode(_output, (bool));
    }
}

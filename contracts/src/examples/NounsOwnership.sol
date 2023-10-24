// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {StorageOracle} from "src/examples/StorageOracle.sol";

/// @notice Fetch Noun ownership from L1 and project on the chain this contract is deployed on.
contract NounsOwnership {
    /// @dev L1 Nouns NFT contract: https://etherscan.io/address/0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03
    address internal constant NOUNS_ADDRESS = 0x9C8fF314C9Bc7F6e59A9d9225Fb22946427eDC03;

    /// @dev Storage Slot in the contract for `mapping(uint256 => address) private _owners;`
    uint256 internal constant OWNERS_SLOT = 3;

    /// @notice The StorageOracle contract address.
    address public immutable STORAGE_ORACLE;

    constructor(address _storageOracle) {
        STORAGE_ORACLE = _storageOracle;
    }

    /// @notice Requests that the ownership of a given noun be accouted for in voting.
    /// @dev This can be called for an existing noun to update the owner.
    /// @param _tokenId The token Id of the noun to claim. For example, to claim noun #100, this should be
    /// 		100.
    function claimOwner(uint256 _tokenId) external returns (bytes32 requestId) {
        uint256 slot = uint256(keccak256(abi.encode(_tokenId, OWNERS_SLOT)));
        requestId = StorageOracle(STORAGE_ORACLE).requestStorageSlot(NOUNS_ADDRESS, slot);
    }

    /// @param _tokenId The token Id of the noun to check ownership of.
    function ownerOf(uint256 _tokenId) external view returns (address owner) {
        (bytes32 value,) = StorageOracle(STORAGE_ORACLE).slots(
            NOUNS_ADDRESS, uint256(keccak256(abi.encode(_tokenId, OWNERS_SLOT)))
        );
        return address(uint160(uint256(value)));
    }

    /// @param _tokenId The token Id of the noun to last updated block of.
    function lastUpdatedBlock(uint256 _tokenId) external view returns (uint256 blockNumber) {
        (, blockNumber) = StorageOracle(STORAGE_ORACLE).slots(
            NOUNS_ADDRESS, uint256(keccak256(abi.encode(_tokenId, OWNERS_SLOT)))
        );
    }
}

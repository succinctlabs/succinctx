contract Examples {
    function requestStepWithCallback(uint256 targetAttestedSlot) {
        bytes32 syncCommitteeRoot = syncCommittee[targetAttestedSlot / 8192];
        bytes memory input = abi.encodePacked(
            syncCommitteeRoot,
            targetAttestedSlot
        );
        bytes32 requestId = IFunctionGateway(FUNCTION_GATEWAY).zkCallback(
            FUNCTION_ID,
            input,
            this.stepCallback.selector,
            abi.encode(targetAttestedSlot)
        );
    }

    function stepCallback(
        bytes memory output,
        bytes memory context_
    ) onlyGateway {
        (bytes32 finalizedSlotLE, uint256 participation) = abi.decodePacked(
            output,
            (bytes32, uint256)
        );
        require(participation >= 2 / 3 * 512);
        bytes32 targetAttestedSlot = abi.decode(context_, (bytes32));
        headers[targetAttestedSlot] = finalizedSlotLE;
    }

    function skip(
        uint64 _trustedBlock,
        uint64 _requestedBlock,
    ) external {
        bytes32 trustedHeader = blockHeightToHeaderHash[_trustedBlock];
        if (trustedHeader == bytes32(0)) {
            revert("Trusted header not found");
        }
        bytes32 id = functionNameToId["skip"];
        if (id == bytes32(0)) {
            revert("Function ID for skip not found");
        }
        require(_requestedBlock > _trustedBlock);
        require(_requestedBlock - _trustedBlock <= 512); // TODO: change this constant
        require(_requestedBlock > head); // TODO: do we need this?

        (bool ok, bytes memory result) = IFunctionGateway(gateway).zkCall(
            STEP_FUNCTION_ID,
            abi.encodePacked(trustedHeader, _trustedBlock, _requestedBlock)
        );

        if (ok == true) {
            bytes32 targetHeader = abi.decode(result, (bytes32));
            headers[_requestedBlock] = targetHeader;
            emit HeaderSkipProcessed(_trustedBlock, _requestedBlock, requestId);
        } else {
            emit HeadSkipRequested(trustedHeader, _trustedBlock, _requestedBlock);
        }
    }
}

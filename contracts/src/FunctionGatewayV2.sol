contract FunctionGatewayUpdated {
    mapping(bytes32 => address) functionIdToVerifier;

    function callbackRequestV1(
        bytes32 _functionId,
        bytes memory _input,
        bytes4 _callbackSelector,
        bytes memory _context,
        uint8 _preference // Slow, medium, fast, aggregation or not, etc.
    ) external payable returns (bytes32) {
        requests[nonce++] = keccak256(FunctionRequest({
            functionId: _functionId,
            inputBytes: _input,
            context: _context,
            callbackAddress: msg.sender,
            callbackSelector: _callbackSelector
        }));
        // TODO: emit event with preference
    }

    function fulfillCallbackRequestV1(
        bytes32 _requestId,
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _output,
        bytes memory _proof,
        bytes memory _context,
        address _callbackAddress,
        bytes32 _callbackSelector
    ) external {
        bytes32 requestHash = keccak256(FunctionRequest({
            functionId: _functionId,
            inputBytes: _input,
            context: _context,
            callbackAddress: _callbackAddress,
            callbackSelector: _callbackSelector
        }));
        require(requests[_requestId] == requestHash, "Request not found");

        address verifier = functionIdToVerifier[_functionId];
        bytes32 inputHash = sha256(_input);
        bytes32 outputHash = sha256(_output);
        if (
            !IFunctionVerifier(verifier).verify(
                inputHash,
                outputHash,
                _proof
            )
        ) {
            revert InvalidProof();
        }

        (bool status, ) = _callbackAddress.call(
            abi.encodeWithSelector(_callbackSelector, _input, _output, _context)
        );
        if (!status) {
            revert CallbackFailed(_callbackAddress, _callbackSelector);
        }

        delete requests[_requestId];
    }

    function storeRequestV1(
        bytes32 _functionId,
        bytes memory _input
    ) {
        emit StoreRequest(_functionId, _input, uint256(0), address(0), bytes4(0));
    }

    function storeRequestV1(
        bytes32 _functionId,
        bytes memory _input,
        uint256 _chain_id,
        address _address,
        bytes4 _selector,
        uint8 preference // Slow, medium, fast, aggregation or not, etc.
    ) {
        emit StoreRequest(_functionId, _input, _chain_id, _address, _selector);
    }

    function fulfillStoreRequest(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _output,
        bytes memory _proof,
        address _address,
        bytes4 _selector
    ) {
        address verifier = functionIdToVerifier[_functionId];
        bytes32 inputHash = sha256(_input);
        bytes32 outputHash = sha256(_output);
        if (
            !IFunctionVerifier(verifier).verify(
                inputHash,
                outputHash,
                _proof
            )
        ) {
            revert InvalidProof();
        }

        // Default case
        oracle[_functionId][inputHash] = outputHash;

        if (_address != address(0)) {
            _address.call(
                abi.encodeWithSelector(_selector, _functionId, _input, _output)
            );
            delete oracle[_functionId][inputHash];
        }
    }

    function verifyStored(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _output
    ) {
        address verifier = functionIdToVerifier[_functionId];
        bytes32 inputHash = sha256(_input);
        bytes32 outputHash = sha256(_output);
        require(oracle[_functionId][inputHash] == outputHash, "Invalid output");
    }
}

contract AnotherContract {
    function requestStepWithCallback(uint256 targetSlot) {
        bytes32 syncCommitteeRoot = syncCommittee[targetSlot / 8192];
        bytes memory input = abi.encodePacked(targetSlot, syncCommitteeRoot);
        bytes32 requestId = IFunctionGateway(FUNCTION_GATEWAY).callbackRequest(
            FUNCTION_ID,
            input,
            this.stepCallback.selector,
            context_
        );
    }

    function stepCallback(bytes memory input, bytes memory output, bytes memory context_) onlyGateway {
        Request memory currentRequest = IGateway.currentRequest(); // If you want to get the current request that is being processed
        // uint256 targetSlot, _ = abi.decode(input);
        bytes32 targetHeader = abi.decode(output, (bytes32));
        headers[targetSlot] = targetHeader;
    }

    function statelessStepRequest(uint256 targetSlot) {
        bytes32 syncCommitteeRoot = syncCommittee[targetSlot / 8192];
        bytes memory input = abi.encodePacked(targetSlot, syncCommitteeRoot);
        IFunctionGateway.storeRequestV1(FUNCTION_ID, input, chain_id, address(this), this.step.selector);
    }

    // Stateless Request
    function step(bytes32 functionId, bytes memory input, bytes memory output) {
        require(FUNCTION_ID == functionId);
        require(IFunctionGateway(FUNCTION_GATEWAY).verifyStored(_functionId, inputs, _output));

        uint256 targetSlot, _ = packedReader.readUint256(input, );
        bytes32 syncCommitteeRoot = packedReader.readUint256(input, (bytes32));

        (uint256 targetSlot, bytes32 syncCommitteeRoot) = abi.decode(input, (uint256, bytes32));
        assert(syncCommittee[targetSlot / 8192] == syncCommitteeRoot);

        (bytes32 targetHeaderRoot, bytes32 nextSyncCommitteeRoot) = abi.decode(output, (bytes32, bytes32));
        headers[targetSlot] = targetHeaderRoot;
        syncCommittees[targetSlot / 8192 + 1] = nextSyncCommitteeRoot;
    }
}
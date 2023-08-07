// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package bindings

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// FunctionGatewayMetaData contains all meta data concerning the FunctionGateway contract.
var FunctionGatewayMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scalar\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_feeVault\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"CallbackAlreadyFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"callbackAddress\",\"type\":\"address\"},{\"internalType\":\"bytes4\",\"name\":\"callbackSelector\",\"type\":\"bytes4\"}],\"name\":\"CallbackFailed\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"contextHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"ContextMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"proofId\",\"type\":\"bytes32\"}],\"name\":\"FunctionAlreadyRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"proofId\",\"type\":\"bytes32\"}],\"name\":\"FunctionNotRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"inputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"InputsRootMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expected\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"actual\",\"type\":\"uint256\"}],\"name\":\"InsufficientFeeAmount\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"inputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"name\":\"InvalidProof\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expected\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"actual\",\"type\":\"uint256\"}],\"name\":\"LengthMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"actualOwner\",\"type\":\"address\"}],\"name\":\"NotFunctionOwner\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"OutputMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"OutputsRootMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"ProofAlreadyFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"ProofNotFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refundAccount\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"refundAmount\",\"type\":\"uint256\"}],\"name\":\"RefundFailed\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"RequestNotFound\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"VerificationKeysRootMismatch\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"output\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"CallbackFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32[]\",\"name\":\"requestIds\",\"type\":\"bytes32[]\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"aggregateProof\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"inputsRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"verificationKeyRoot\",\"type\":\"bytes32\"}],\"name\":\"ProofBatchFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"name\":\"ProofFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"inputs\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"name\":\"ProofRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"scalar\",\"type\":\"uint256\"}],\"name\":\"ScalarUpdated\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"AGGREGATION_FUNCTION_ID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DEFAULT_GAS_LIMIT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\"}],\"name\":\"calculateFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"calculateFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_requestId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_output\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"callback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"feeVault\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_requestId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"}],\"name\":\"fulfill\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_requestIds\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes\",\"name\":\"_aggregateProof\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_inputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"_outputHashes\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32\",\"name\":\"_outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_verificationKeyRoot\",\"type\":\"bytes32\"}],\"name\":\"fulfillBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"registerFunction\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_input\",\"type\":\"bytes\"},{\"internalType\":\"bytes4\",\"name\":\"_callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_refundAccount\",\"type\":\"address\"}],\"name\":\"request\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_input\",\"type\":\"bytes\"},{\"internalType\":\"bytes4\",\"name\":\"_callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"request\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"requests\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"inputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"contextHash\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"callbackAddress\",\"type\":\"address\"},{\"internalType\":\"bytes4\",\"name\":\"callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bool\",\"name\":\"proofFulfilled\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"callbackFulfilled\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"scalar\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"updateFunctionOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"}],\"name\":\"updateFunctionVerifier\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scalar\",\"type\":\"uint256\"}],\"name\":\"updateScalar\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifierOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifiers\",\"outputs\":[{\"internalType\":\"contractIFunctionVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
	Bin: "0x60a0604052620f42406003553480156200001857600080fd5b5060405162001db538038062001db58339810160408190526200003b91620000db565b62000046336200006c565b60068390556001600160a01b03821660805262000063816200006c565b5050506200011c565b600280546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b80516001600160a01b0381168114620000d657600080fd5b919050565b600080600060608486031215620000f157600080fd5b835192506200010360208501620000be565b91506200011360408501620000be565b90509250925092565b608051611c6f62000146600039600081816101d50152818161132101526113700152611c6f6000f3fe6080604052600436106101355760003560e01c80639d866985116100ab578063e23b04101161006f578063e23b041014610405578063e7ddf4c614610425578063efe1c95014610445578063f2fde38b1461047b578063f45e65d81461049b578063f7da7486146104b157600080fd5b80639d866985146102eb578063affed0e0146103b1578063c30d9826146103c7578063d6be695a146103dc578063e2362c31146103f257600080fd5b8063715018a6116100fd578063715018a61461022f57806387c5621a146102445780638ab4be9e146102645780638b4d7bc4146102845780638bcfc3a0146102975780638da5cb5b146102cd57600080fd5b8063178f7b401461013a57806337ea88471461016d5780633bb600391461018f578063478222c2146101c35780636f652c951461020f575b600080fd5b34801561014657600080fd5b5061015a610155366004611474565b6104d1565b6040519081526020015b60405180910390f35b34801561017957600080fd5b5061018d6101883660046115c4565b610509565b005b34801561019b57600080fd5b5061015a7fcf91d3a65d6f619b1560b4409a7377da358299d073f6633a90fe3313a88b47f581565b3480156101cf57600080fd5b506101f77f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610164565b34801561021b57600080fd5b5061018d61022a36600461167f565b6109e1565b34801561023b57600080fd5b5061018d610aa0565b34801561025057600080fd5b5061018d61025f3660046116ab565b610ab4565b34801561027057600080fd5b5061018d61027f3660046116fb565b610c30565b61015a610292366004611776565b610e6d565b3480156102a357600080fd5b506101f76102b2366004611474565b6001602052600090815260409020546001600160a01b031681565b3480156102d957600080fd5b506002546001600160a01b03166101f7565b3480156102f757600080fd5b5061035f610306366004611474565b60056020526000908152604090208054600182015460028301546003840154600490940154929391929091906001600160a01b03811690600160a01b810460e01b9060ff600160c01b8204811691600160c81b90041688565b6040805198895260208901979097529587019490945260608601929092526001600160a01b031660808501526001600160e01b03191660a0840152151560c0830152151560e082015261010001610164565b3480156103bd57600080fd5b5061015a60045481565b3480156103d357600080fd5b5061015a61100f565b3480156103e857600080fd5b5061015a60035481565b61015a61040036600461180d565b611021565b34801561041157600080fd5b5061018d610420366004611474565b61103c565b34801561043157600080fd5b5061018d61044036600461167f565b61107f565b34801561045157600080fd5b506101f7610460366004611474565b6000602081905290815260409020546001600160a01b031681565b34801561048757600080fd5b5061018d61049636600461188b565b61113e565b3480156104a757600080fd5b5061015a60065481565b3480156104bd57600080fd5b5061018d6104cc3660046118ad565b6111b7565b60006006546000036104ed576104e7823a6118ff565b92915050565b6006546104fa833a6118ff565b6104e791906118ff565b919050565b6000865167ffffffffffffffff8111156105255761052561148d565b60405190808252806020026020018201604052801561054e578160200160208202803683370190505b5090506000875167ffffffffffffffff81111561056d5761056d61148d565b604051908082528060200260200182016040528015610596578160200160208202803683370190505b50905060005b88518110156107095760008982815181106105b9576105b9611916565b6020908102919091018101516000818152600590925260409091206004810154919250906001600160a01b031661060b57604051630d1c383160e11b8152600481018390526024015b60405180910390fd5b6004810154600160c01b900460ff161561063b57604051631aea9acb60e31b815260048101839052602401610602565b806001015485848151811061065257610652611916565b60209081029190910181019190915281546000908152808252604090819020548151634f44a2e960e11b815291516001600160a01b0390911692639e8945d292600480820193918290030181865afa1580156106b2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106d6919061192c565b8484815181106106e8576106e8611916565b6020026020010181815250505050808061070190611945565b91505061059c565b50845188511461073957875185516040516355c5b3e360e11b815260048101929092526024820152604401610602565b8160405160200161074a9190611999565b60405160208183030381529060405280519060200120861461078357858260405163e2920b9160e01b81526004016106029291906119ac565b846040516020016107949190611999565b6040516020818303038152906040528051906020012084146107cd578385604051637ccf42f360e01b81526004016106029291906119ac565b806040516020016107de9190611999565b60405160208183030381529060405280519060200120831461081757828160405163693d503560e11b81526004016106029291906119ac565b60005b88518110156108ae57600089828151811061083757610837611916565b602090810291909101810151600081815260059092526040909120600401805460ff60c01b1916600160c01b179055875190915087908390811061087d5761087d611916565b60209081029190910181015160009283526005909152604090912060020155806108a681611945565b91505061081a565b507fcf91d3a65d6f619b1560b4409a7377da358299d073f6633a90fe3313a88b47f560009081526020527fdfbb683d42ec23abfd9b50088f945b2feb0772147412dcd9441f8e87a3f0ff9e546040516303784b1960e61b81526001600160a01b0390911690819063de12c6409061092d908a9089908d90600401611a1d565b6020604051808303816000875af115801561094c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109709190611a3c565b610995578087868a6040516316c7141360e31b81526004016106029493929190611a5e565b7f9f5bcf5fecad905a6b02f0a6c02a52568005592a0d6c0711752b20ca854e23028989898989896040516109ce96959493929190611a95565b60405180910390a1505050505050505050565b6000828152602081905260409020546001600160a01b0316610a195760405163632e273160e11b815260048101839052602401610602565b6000828152600160205260409020546001600160a01b03163314610a725760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610602565b60009182526020829052604090912080546001600160a01b0319166001600160a01b03909216919091179055565b610aa8611230565b610ab2600061128a565b565b600083815260056020526040902060048101546001600160a01b0316610af057604051630d1c383160e11b815260048101859052602401610602565b6004810154600160c01b900460ff1615610b2057604051631aea9acb60e31b815260048101859052602401610602565b6004808201805460ff60c01b1916600160c01b1790556002820184905581546000908152602081905260409081902054600184015491516303784b1960e61b81526001600160a01b0390911692839263de12c64092610b83928991899101611a1d565b6020604051808303816000875af1158015610ba2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bc69190611a3c565b610bee5760018201546040516316c7141360e31b815261060291839187908790600401611a5e565b7ffddf097ddc1205e34fd4700d12ad51b32ccad4f117f7ac879a74d20b145209b4858585604051610c2193929190611a1d565b60405180910390a15050505050565b60008381526005602052604090206004810154600160c81b900460ff1615610c6e5760405163b08540e560e01b815260048101859052602401610602565b60048101546001600160a01b0316610c9c57604051630d1c383160e11b815260048101859052602401610602565b81516020830120600382015414610cca5783826040516389116ecd60e01b8152600401610602929190611aea565b82516020840120600282015414610cf8578383604051633cfc30e360e01b8152600401610602929190611aea565b6004810154600160c01b900460ff16610d2757604051635ca8297160e11b815260048101859052602401610602565b60048101805460ff60c81b198116600160c81b17918290556040516000926001600160a01b0390921691600160a01b900460e01b90610d6c9087908790602401611b03565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610daa9190611b28565b6000604051808303816000865af19150503d8060008114610de7576040519150601f19603f3d011682016040523d82523d6000602084013e610dec565b606091505b5050905080610e3a5760048281015460405163bc4a234960e01b81526001600160a01b03821692810192909252600160a01b900460e01b6001600160e01b0319166024820152604401610602565b7f4157c302cad5507e9c624680b653ae4a290e304cb0ff86a730bceda763ec878d858585604051610c2193929190611b44565b845160208087019190912084518583012060408051610100810182528a815293840183905260009084018190526060840182905233608085018190526001600160e01b0319891660a086015260c0850182905260e085018290529093908490610eda9088908890346112dc565b9050600060045483604051602001610ef3929190611b6f565b60408051601f198184030181528282528051602091820120600081815260058352839020875181559187015160018301559186015160028201556060860151600382015560808601516004918201805460a089015160c08a015160e0808c01511515600160c81b0260ff60c81b19921515600160c01b029290921661ffff60c01b199390911c600160a01b026001600160c01b03199094166001600160a01b03909616959095179290921716929092179190911790555490925082917f78248b3a4298ac22184cd31fb142c8da8cf242175e64a58fc06b900dce6aeb9190610fe2908f908e908e908990611bed565b60405180910390a360048054906000610ffa83611945565b90915550909c9b505050505050505050505050565b600061101c6003546104d1565b905090565b60006110338585858560035432610e6d565b95945050505050565b611044611230565b60068190556040518181527f3336cd9708eaf2769a0f0dc0679f30e80f15dcd88d1921b5a16858e8b85c591a9060200160405180910390a150565b6000828152602081905260409020546001600160a01b03166110b75760405163632e273160e11b815260048101839052602401610602565b6000828152600160205260409020546001600160a01b031633146111105760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610602565b60009182526001602052604090912080546001600160a01b0319166001600160a01b03909216919091179055565b611146611230565b6001600160a01b0381166111ab5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610602565b6111b48161128a565b50565b6000838152602081905260409020546001600160a01b0316156111f057604051635e34c78f60e01b815260048101849052602401610602565b60009283526020838152604080852080546001600160a01b03199081166001600160a01b0396871617909155600190925290932080549093169116179055565b6002546001600160a01b03163314610ab25760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610602565b600280546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60006112e7856104d1565b90508082101561131457604051630961b65b60e41b81526004810182905260248101839052604401610602565b60008111801561134c57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031615155b156113cf576040516333bb7f9160e01b81526001600160a01b0384811660048301527f000000000000000000000000000000000000000000000000000000000000000016906333bb7f919083906024016000604051808303818588803b1580156113b557600080fd5b505af11580156113c9573d6000803e3d6000fd5b50505050505b60006113db8284611c26565b9050801561146b576000856001600160a01b03168260405160006040518083038185875af1925050503d8060008114611430576040519150601f19603f3d011682016040523d82523d6000602084013e611435565b606091505b5050905080611469576040516357b9d85960e11b81526001600160a01b038716600482015260248101839052604401610602565b505b50949350505050565b60006020828403121561148657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156114cc576114cc61148d565b604052919050565b600082601f8301126114e557600080fd5b8135602067ffffffffffffffff8211156115015761150161148d565b8160051b6115108282016114a3565b928352848101820192828101908785111561152a57600080fd5b83870192505b8483101561154957823582529183019190830190611530565b979650505050505050565b600082601f83011261156557600080fd5b813567ffffffffffffffff81111561157f5761157f61148d565b611592601f8201601f19166020016114a3565b8181528460208386010111156115a757600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060008060c087890312156115dd57600080fd5b863567ffffffffffffffff808211156115f557600080fd5b6116018a838b016114d4565b9750602089013591508082111561161757600080fd5b6116238a838b01611554565b965060408901359550606089013591508082111561164057600080fd5b5061164d89828a016114d4565b9350506080870135915060a087013590509295509295509295565b80356001600160a01b038116811461050457600080fd5b6000806040838503121561169257600080fd5b823591506116a260208401611668565b90509250929050565b6000806000606084860312156116c057600080fd5b8335925060208401359150604084013567ffffffffffffffff8111156116e557600080fd5b6116f186828701611554565b9150509250925092565b60008060006060848603121561171057600080fd5b83359250602084013567ffffffffffffffff8082111561172f57600080fd5b61173b87838801611554565b9350604086013591508082111561175157600080fd5b506116f186828701611554565b80356001600160e01b03198116811461050457600080fd5b60008060008060008060c0878903121561178f57600080fd5b86359550602087013567ffffffffffffffff808211156117ae57600080fd5b6117ba8a838b01611554565b96506117c860408a0161175e565b955060608901359150808211156117de57600080fd5b506117eb89828a01611554565b9350506080870135915061180160a08801611668565b90509295509295509295565b6000806000806080858703121561182357600080fd5b84359350602085013567ffffffffffffffff8082111561184257600080fd5b61184e88838901611554565b945061185c6040880161175e565b9350606087013591508082111561187257600080fd5b5061187f87828801611554565b91505092959194509250565b60006020828403121561189d57600080fd5b6118a682611668565b9392505050565b6000806000606084860312156118c257600080fd5b833592506118d260208501611668565b91506118e060408501611668565b90509250925092565b634e487b7160e01b600052601160045260246000fd5b80820281158282048414176104e7576104e76118e9565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561193e57600080fd5b5051919050565b600060018201611957576119576118e9565b5060010190565b600081518084526020808501945080840160005b8381101561198e57815187529582019590820190600101611972565b509495945050505050565b6020815260006118a6602083018461195e565b8281526040602082015260006119c5604083018461195e565b949350505050565b60005b838110156119e85781810151838201526020016119d0565b50506000910152565b60008151808452611a098160208601602086016119cd565b601f01601f19169290920160200192915050565b83815282602082015260606040820152600061103360608301846119f1565b600060208284031215611a4e57600080fd5b815180151581146118a657600080fd5b60018060a01b0385168152836020820152826040820152608060608201526000611a8b60808301846119f1565b9695505050505050565b60c081526000611aa860c083018961195e565b8281036020840152611aba81896119f1565b90508660408401528281036060840152611ad4818761195e565b6080840195909552505060a00152949350505050565b8281526040602082015260006119c560408301846119f1565b604081526000611b1660408301856119f1565b828103602084015261103381856119f1565b60008251611b3a8184602087016119cd565b9190910192915050565b838152606060208201526000611b5d60608301856119f1565b8281036040840152611a8b81856119f1565b6000610120820190508382528251602083015260208301516040830152604083015160608301526060830151608083015260018060a01b0360808401511660a083015263ffffffff60e01b60a08401511660c083015260c0830151151560e083015260e0830151611be561010084018215159052565b509392505050565b608081526000611c0060808301876119f1565b8281036020840152611c1281876119f1565b604084019590955250506060015292915050565b818103818111156104e7576104e76118e956fea2646970667358221220ba8a09b59401f23eb40c2f5b469da478c851acc2f40f3b41a9637d1fb14f21f364736f6c63430008140033",
}

// FunctionGatewayABI is the input ABI used to generate the binding from.
// Deprecated: Use FunctionGatewayMetaData.ABI instead.
var FunctionGatewayABI = FunctionGatewayMetaData.ABI

// FunctionGatewayBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use FunctionGatewayMetaData.Bin instead.
var FunctionGatewayBin = FunctionGatewayMetaData.Bin

// DeployFunctionGateway deploys a new Ethereum contract, binding an instance of FunctionGateway to it.
func DeployFunctionGateway(auth *bind.TransactOpts, backend bind.ContractBackend, _scalar *big.Int, _feeVault common.Address, _owner common.Address) (common.Address, *types.Transaction, *FunctionGateway, error) {
	parsed, err := FunctionGatewayMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(FunctionGatewayBin), backend, _scalar, _feeVault, _owner)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &FunctionGateway{FunctionGatewayCaller: FunctionGatewayCaller{contract: contract}, FunctionGatewayTransactor: FunctionGatewayTransactor{contract: contract}, FunctionGatewayFilterer: FunctionGatewayFilterer{contract: contract}}, nil
}

// FunctionGateway is an auto generated Go binding around an Ethereum contract.
type FunctionGateway struct {
	FunctionGatewayCaller     // Read-only binding to the contract
	FunctionGatewayTransactor // Write-only binding to the contract
	FunctionGatewayFilterer   // Log filterer for contract events
}

// FunctionGatewayCaller is an auto generated read-only Go binding around an Ethereum contract.
type FunctionGatewayCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FunctionGatewayTransactor is an auto generated write-only Go binding around an Ethereum contract.
type FunctionGatewayTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FunctionGatewayFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type FunctionGatewayFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FunctionGatewaySession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type FunctionGatewaySession struct {
	Contract     *FunctionGateway  // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// FunctionGatewayCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type FunctionGatewayCallerSession struct {
	Contract *FunctionGatewayCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts          // Call options to use throughout this session
}

// FunctionGatewayTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type FunctionGatewayTransactorSession struct {
	Contract     *FunctionGatewayTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts          // Transaction auth options to use throughout this session
}

// FunctionGatewayRaw is an auto generated low-level Go binding around an Ethereum contract.
type FunctionGatewayRaw struct {
	Contract *FunctionGateway // Generic contract binding to access the raw methods on
}

// FunctionGatewayCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type FunctionGatewayCallerRaw struct {
	Contract *FunctionGatewayCaller // Generic read-only contract binding to access the raw methods on
}

// FunctionGatewayTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type FunctionGatewayTransactorRaw struct {
	Contract *FunctionGatewayTransactor // Generic write-only contract binding to access the raw methods on
}

// NewFunctionGateway creates a new instance of FunctionGateway, bound to a specific deployed contract.
func NewFunctionGateway(address common.Address, backend bind.ContractBackend) (*FunctionGateway, error) {
	contract, err := bindFunctionGateway(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &FunctionGateway{FunctionGatewayCaller: FunctionGatewayCaller{contract: contract}, FunctionGatewayTransactor: FunctionGatewayTransactor{contract: contract}, FunctionGatewayFilterer: FunctionGatewayFilterer{contract: contract}}, nil
}

// NewFunctionGatewayCaller creates a new read-only instance of FunctionGateway, bound to a specific deployed contract.
func NewFunctionGatewayCaller(address common.Address, caller bind.ContractCaller) (*FunctionGatewayCaller, error) {
	contract, err := bindFunctionGateway(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayCaller{contract: contract}, nil
}

// NewFunctionGatewayTransactor creates a new write-only instance of FunctionGateway, bound to a specific deployed contract.
func NewFunctionGatewayTransactor(address common.Address, transactor bind.ContractTransactor) (*FunctionGatewayTransactor, error) {
	contract, err := bindFunctionGateway(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayTransactor{contract: contract}, nil
}

// NewFunctionGatewayFilterer creates a new log filterer instance of FunctionGateway, bound to a specific deployed contract.
func NewFunctionGatewayFilterer(address common.Address, filterer bind.ContractFilterer) (*FunctionGatewayFilterer, error) {
	contract, err := bindFunctionGateway(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayFilterer{contract: contract}, nil
}

// bindFunctionGateway binds a generic wrapper to an already deployed contract.
func bindFunctionGateway(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := FunctionGatewayMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_FunctionGateway *FunctionGatewayRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _FunctionGateway.Contract.FunctionGatewayCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_FunctionGateway *FunctionGatewayRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FunctionGateway.Contract.FunctionGatewayTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_FunctionGateway *FunctionGatewayRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _FunctionGateway.Contract.FunctionGatewayTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_FunctionGateway *FunctionGatewayCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _FunctionGateway.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_FunctionGateway *FunctionGatewayTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FunctionGateway.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_FunctionGateway *FunctionGatewayTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _FunctionGateway.Contract.contract.Transact(opts, method, params...)
}

// AGGREGATIONFUNCTIONID is a free data retrieval call binding the contract method 0x3bb60039.
//
// Solidity: function AGGREGATION_FUNCTION_ID() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCaller) AGGREGATIONFUNCTIONID(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "AGGREGATION_FUNCTION_ID")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AGGREGATIONFUNCTIONID is a free data retrieval call binding the contract method 0x3bb60039.
//
// Solidity: function AGGREGATION_FUNCTION_ID() view returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) AGGREGATIONFUNCTIONID() ([32]byte, error) {
	return _FunctionGateway.Contract.AGGREGATIONFUNCTIONID(&_FunctionGateway.CallOpts)
}

// AGGREGATIONFUNCTIONID is a free data retrieval call binding the contract method 0x3bb60039.
//
// Solidity: function AGGREGATION_FUNCTION_ID() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCallerSession) AGGREGATIONFUNCTIONID() ([32]byte, error) {
	return _FunctionGateway.Contract.AGGREGATIONFUNCTIONID(&_FunctionGateway.CallOpts)
}

// DEFAULTGASLIMIT is a free data retrieval call binding the contract method 0xd6be695a.
//
// Solidity: function DEFAULT_GAS_LIMIT() view returns(uint256)
func (_FunctionGateway *FunctionGatewayCaller) DEFAULTGASLIMIT(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "DEFAULT_GAS_LIMIT")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// DEFAULTGASLIMIT is a free data retrieval call binding the contract method 0xd6be695a.
//
// Solidity: function DEFAULT_GAS_LIMIT() view returns(uint256)
func (_FunctionGateway *FunctionGatewaySession) DEFAULTGASLIMIT() (*big.Int, error) {
	return _FunctionGateway.Contract.DEFAULTGASLIMIT(&_FunctionGateway.CallOpts)
}

// DEFAULTGASLIMIT is a free data retrieval call binding the contract method 0xd6be695a.
//
// Solidity: function DEFAULT_GAS_LIMIT() view returns(uint256)
func (_FunctionGateway *FunctionGatewayCallerSession) DEFAULTGASLIMIT() (*big.Int, error) {
	return _FunctionGateway.Contract.DEFAULTGASLIMIT(&_FunctionGateway.CallOpts)
}

// CalculateFeeAmount is a free data retrieval call binding the contract method 0x178f7b40.
//
// Solidity: function calculateFeeAmount(uint256 _gasLimit) view returns(uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayCaller) CalculateFeeAmount(opts *bind.CallOpts, _gasLimit *big.Int) (*big.Int, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "calculateFeeAmount", _gasLimit)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// CalculateFeeAmount is a free data retrieval call binding the contract method 0x178f7b40.
//
// Solidity: function calculateFeeAmount(uint256 _gasLimit) view returns(uint256 feeAmount)
func (_FunctionGateway *FunctionGatewaySession) CalculateFeeAmount(_gasLimit *big.Int) (*big.Int, error) {
	return _FunctionGateway.Contract.CalculateFeeAmount(&_FunctionGateway.CallOpts, _gasLimit)
}

// CalculateFeeAmount is a free data retrieval call binding the contract method 0x178f7b40.
//
// Solidity: function calculateFeeAmount(uint256 _gasLimit) view returns(uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayCallerSession) CalculateFeeAmount(_gasLimit *big.Int) (*big.Int, error) {
	return _FunctionGateway.Contract.CalculateFeeAmount(&_FunctionGateway.CallOpts, _gasLimit)
}

// CalculateFeeAmount0 is a free data retrieval call binding the contract method 0xc30d9826.
//
// Solidity: function calculateFeeAmount() view returns(uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayCaller) CalculateFeeAmount0(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "calculateFeeAmount0")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// CalculateFeeAmount0 is a free data retrieval call binding the contract method 0xc30d9826.
//
// Solidity: function calculateFeeAmount() view returns(uint256 feeAmount)
func (_FunctionGateway *FunctionGatewaySession) CalculateFeeAmount0() (*big.Int, error) {
	return _FunctionGateway.Contract.CalculateFeeAmount0(&_FunctionGateway.CallOpts)
}

// CalculateFeeAmount0 is a free data retrieval call binding the contract method 0xc30d9826.
//
// Solidity: function calculateFeeAmount() view returns(uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayCallerSession) CalculateFeeAmount0() (*big.Int, error) {
	return _FunctionGateway.Contract.CalculateFeeAmount0(&_FunctionGateway.CallOpts)
}

// FeeVault is a free data retrieval call binding the contract method 0x478222c2.
//
// Solidity: function feeVault() view returns(address)
func (_FunctionGateway *FunctionGatewayCaller) FeeVault(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "feeVault")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// FeeVault is a free data retrieval call binding the contract method 0x478222c2.
//
// Solidity: function feeVault() view returns(address)
func (_FunctionGateway *FunctionGatewaySession) FeeVault() (common.Address, error) {
	return _FunctionGateway.Contract.FeeVault(&_FunctionGateway.CallOpts)
}

// FeeVault is a free data retrieval call binding the contract method 0x478222c2.
//
// Solidity: function feeVault() view returns(address)
func (_FunctionGateway *FunctionGatewayCallerSession) FeeVault() (common.Address, error) {
	return _FunctionGateway.Contract.FeeVault(&_FunctionGateway.CallOpts)
}

// Nonce is a free data retrieval call binding the contract method 0xaffed0e0.
//
// Solidity: function nonce() view returns(uint256)
func (_FunctionGateway *FunctionGatewayCaller) Nonce(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "nonce")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Nonce is a free data retrieval call binding the contract method 0xaffed0e0.
//
// Solidity: function nonce() view returns(uint256)
func (_FunctionGateway *FunctionGatewaySession) Nonce() (*big.Int, error) {
	return _FunctionGateway.Contract.Nonce(&_FunctionGateway.CallOpts)
}

// Nonce is a free data retrieval call binding the contract method 0xaffed0e0.
//
// Solidity: function nonce() view returns(uint256)
func (_FunctionGateway *FunctionGatewayCallerSession) Nonce() (*big.Int, error) {
	return _FunctionGateway.Contract.Nonce(&_FunctionGateway.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_FunctionGateway *FunctionGatewayCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_FunctionGateway *FunctionGatewaySession) Owner() (common.Address, error) {
	return _FunctionGateway.Contract.Owner(&_FunctionGateway.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_FunctionGateway *FunctionGatewayCallerSession) Owner() (common.Address, error) {
	return _FunctionGateway.Contract.Owner(&_FunctionGateway.CallOpts)
}

// Requests is a free data retrieval call binding the contract method 0x9d866985.
//
// Solidity: function requests(bytes32 ) view returns(bytes32 functionId, bytes32 inputHash, bytes32 outputHash, bytes32 contextHash, address callbackAddress, bytes4 callbackSelector, bool proofFulfilled, bool callbackFulfilled)
func (_FunctionGateway *FunctionGatewayCaller) Requests(opts *bind.CallOpts, arg0 [32]byte) (struct {
	FunctionId        [32]byte
	InputHash         [32]byte
	OutputHash        [32]byte
	ContextHash       [32]byte
	CallbackAddress   common.Address
	CallbackSelector  [4]byte
	ProofFulfilled    bool
	CallbackFulfilled bool
}, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "requests", arg0)

	outstruct := new(struct {
		FunctionId        [32]byte
		InputHash         [32]byte
		OutputHash        [32]byte
		ContextHash       [32]byte
		CallbackAddress   common.Address
		CallbackSelector  [4]byte
		ProofFulfilled    bool
		CallbackFulfilled bool
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.FunctionId = *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)
	outstruct.InputHash = *abi.ConvertType(out[1], new([32]byte)).(*[32]byte)
	outstruct.OutputHash = *abi.ConvertType(out[2], new([32]byte)).(*[32]byte)
	outstruct.ContextHash = *abi.ConvertType(out[3], new([32]byte)).(*[32]byte)
	outstruct.CallbackAddress = *abi.ConvertType(out[4], new(common.Address)).(*common.Address)
	outstruct.CallbackSelector = *abi.ConvertType(out[5], new([4]byte)).(*[4]byte)
	outstruct.ProofFulfilled = *abi.ConvertType(out[6], new(bool)).(*bool)
	outstruct.CallbackFulfilled = *abi.ConvertType(out[7], new(bool)).(*bool)

	return *outstruct, err

}

// Requests is a free data retrieval call binding the contract method 0x9d866985.
//
// Solidity: function requests(bytes32 ) view returns(bytes32 functionId, bytes32 inputHash, bytes32 outputHash, bytes32 contextHash, address callbackAddress, bytes4 callbackSelector, bool proofFulfilled, bool callbackFulfilled)
func (_FunctionGateway *FunctionGatewaySession) Requests(arg0 [32]byte) (struct {
	FunctionId        [32]byte
	InputHash         [32]byte
	OutputHash        [32]byte
	ContextHash       [32]byte
	CallbackAddress   common.Address
	CallbackSelector  [4]byte
	ProofFulfilled    bool
	CallbackFulfilled bool
}, error) {
	return _FunctionGateway.Contract.Requests(&_FunctionGateway.CallOpts, arg0)
}

// Requests is a free data retrieval call binding the contract method 0x9d866985.
//
// Solidity: function requests(bytes32 ) view returns(bytes32 functionId, bytes32 inputHash, bytes32 outputHash, bytes32 contextHash, address callbackAddress, bytes4 callbackSelector, bool proofFulfilled, bool callbackFulfilled)
func (_FunctionGateway *FunctionGatewayCallerSession) Requests(arg0 [32]byte) (struct {
	FunctionId        [32]byte
	InputHash         [32]byte
	OutputHash        [32]byte
	ContextHash       [32]byte
	CallbackAddress   common.Address
	CallbackSelector  [4]byte
	ProofFulfilled    bool
	CallbackFulfilled bool
}, error) {
	return _FunctionGateway.Contract.Requests(&_FunctionGateway.CallOpts, arg0)
}

// Scalar is a free data retrieval call binding the contract method 0xf45e65d8.
//
// Solidity: function scalar() view returns(uint256)
func (_FunctionGateway *FunctionGatewayCaller) Scalar(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "scalar")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Scalar is a free data retrieval call binding the contract method 0xf45e65d8.
//
// Solidity: function scalar() view returns(uint256)
func (_FunctionGateway *FunctionGatewaySession) Scalar() (*big.Int, error) {
	return _FunctionGateway.Contract.Scalar(&_FunctionGateway.CallOpts)
}

// Scalar is a free data retrieval call binding the contract method 0xf45e65d8.
//
// Solidity: function scalar() view returns(uint256)
func (_FunctionGateway *FunctionGatewayCallerSession) Scalar() (*big.Int, error) {
	return _FunctionGateway.Contract.Scalar(&_FunctionGateway.CallOpts)
}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_FunctionGateway *FunctionGatewayCaller) VerifierOwners(opts *bind.CallOpts, arg0 [32]byte) (common.Address, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "verifierOwners", arg0)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_FunctionGateway *FunctionGatewaySession) VerifierOwners(arg0 [32]byte) (common.Address, error) {
	return _FunctionGateway.Contract.VerifierOwners(&_FunctionGateway.CallOpts, arg0)
}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_FunctionGateway *FunctionGatewayCallerSession) VerifierOwners(arg0 [32]byte) (common.Address, error) {
	return _FunctionGateway.Contract.VerifierOwners(&_FunctionGateway.CallOpts, arg0)
}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_FunctionGateway *FunctionGatewayCaller) Verifiers(opts *bind.CallOpts, arg0 [32]byte) (common.Address, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "verifiers", arg0)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_FunctionGateway *FunctionGatewaySession) Verifiers(arg0 [32]byte) (common.Address, error) {
	return _FunctionGateway.Contract.Verifiers(&_FunctionGateway.CallOpts, arg0)
}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_FunctionGateway *FunctionGatewayCallerSession) Verifiers(arg0 [32]byte) (common.Address, error) {
	return _FunctionGateway.Contract.Verifiers(&_FunctionGateway.CallOpts, arg0)
}

// Callback is a paid mutator transaction binding the contract method 0x8ab4be9e.
//
// Solidity: function callback(bytes32 _requestId, bytes _output, bytes _context) returns()
func (_FunctionGateway *FunctionGatewayTransactor) Callback(opts *bind.TransactOpts, _requestId [32]byte, _output []byte, _context []byte) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "callback", _requestId, _output, _context)
}

// Callback is a paid mutator transaction binding the contract method 0x8ab4be9e.
//
// Solidity: function callback(bytes32 _requestId, bytes _output, bytes _context) returns()
func (_FunctionGateway *FunctionGatewaySession) Callback(_requestId [32]byte, _output []byte, _context []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Callback(&_FunctionGateway.TransactOpts, _requestId, _output, _context)
}

// Callback is a paid mutator transaction binding the contract method 0x8ab4be9e.
//
// Solidity: function callback(bytes32 _requestId, bytes _output, bytes _context) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) Callback(_requestId [32]byte, _output []byte, _context []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Callback(&_FunctionGateway.TransactOpts, _requestId, _output, _context)
}

// Fulfill is a paid mutator transaction binding the contract method 0x87c5621a.
//
// Solidity: function fulfill(bytes32 _requestId, bytes32 _outputHash, bytes _proof) returns()
func (_FunctionGateway *FunctionGatewayTransactor) Fulfill(opts *bind.TransactOpts, _requestId [32]byte, _outputHash [32]byte, _proof []byte) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "fulfill", _requestId, _outputHash, _proof)
}

// Fulfill is a paid mutator transaction binding the contract method 0x87c5621a.
//
// Solidity: function fulfill(bytes32 _requestId, bytes32 _outputHash, bytes _proof) returns()
func (_FunctionGateway *FunctionGatewaySession) Fulfill(_requestId [32]byte, _outputHash [32]byte, _proof []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Fulfill(&_FunctionGateway.TransactOpts, _requestId, _outputHash, _proof)
}

// Fulfill is a paid mutator transaction binding the contract method 0x87c5621a.
//
// Solidity: function fulfill(bytes32 _requestId, bytes32 _outputHash, bytes _proof) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) Fulfill(_requestId [32]byte, _outputHash [32]byte, _proof []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Fulfill(&_FunctionGateway.TransactOpts, _requestId, _outputHash, _proof)
}

// FulfillBatch is a paid mutator transaction binding the contract method 0x37ea8847.
//
// Solidity: function fulfillBatch(bytes32[] _requestIds, bytes _aggregateProof, bytes32 _inputsRoot, bytes32[] _outputHashes, bytes32 _outputsRoot, bytes32 _verificationKeyRoot) returns()
func (_FunctionGateway *FunctionGatewayTransactor) FulfillBatch(opts *bind.TransactOpts, _requestIds [][32]byte, _aggregateProof []byte, _inputsRoot [32]byte, _outputHashes [][32]byte, _outputsRoot [32]byte, _verificationKeyRoot [32]byte) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "fulfillBatch", _requestIds, _aggregateProof, _inputsRoot, _outputHashes, _outputsRoot, _verificationKeyRoot)
}

// FulfillBatch is a paid mutator transaction binding the contract method 0x37ea8847.
//
// Solidity: function fulfillBatch(bytes32[] _requestIds, bytes _aggregateProof, bytes32 _inputsRoot, bytes32[] _outputHashes, bytes32 _outputsRoot, bytes32 _verificationKeyRoot) returns()
func (_FunctionGateway *FunctionGatewaySession) FulfillBatch(_requestIds [][32]byte, _aggregateProof []byte, _inputsRoot [32]byte, _outputHashes [][32]byte, _outputsRoot [32]byte, _verificationKeyRoot [32]byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.FulfillBatch(&_FunctionGateway.TransactOpts, _requestIds, _aggregateProof, _inputsRoot, _outputHashes, _outputsRoot, _verificationKeyRoot)
}

// FulfillBatch is a paid mutator transaction binding the contract method 0x37ea8847.
//
// Solidity: function fulfillBatch(bytes32[] _requestIds, bytes _aggregateProof, bytes32 _inputsRoot, bytes32[] _outputHashes, bytes32 _outputsRoot, bytes32 _verificationKeyRoot) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) FulfillBatch(_requestIds [][32]byte, _aggregateProof []byte, _inputsRoot [32]byte, _outputHashes [][32]byte, _outputsRoot [32]byte, _verificationKeyRoot [32]byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.FulfillBatch(&_FunctionGateway.TransactOpts, _requestIds, _aggregateProof, _inputsRoot, _outputHashes, _outputsRoot, _verificationKeyRoot)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0xf7da7486.
//
// Solidity: function registerFunction(bytes32 _functionId, address _verifier, address _owner) returns()
func (_FunctionGateway *FunctionGatewayTransactor) RegisterFunction(opts *bind.TransactOpts, _functionId [32]byte, _verifier common.Address, _owner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "registerFunction", _functionId, _verifier, _owner)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0xf7da7486.
//
// Solidity: function registerFunction(bytes32 _functionId, address _verifier, address _owner) returns()
func (_FunctionGateway *FunctionGatewaySession) RegisterFunction(_functionId [32]byte, _verifier common.Address, _owner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RegisterFunction(&_FunctionGateway.TransactOpts, _functionId, _verifier, _owner)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0xf7da7486.
//
// Solidity: function registerFunction(bytes32 _functionId, address _verifier, address _owner) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) RegisterFunction(_functionId [32]byte, _verifier common.Address, _owner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RegisterFunction(&_FunctionGateway.TransactOpts, _functionId, _verifier, _owner)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_FunctionGateway *FunctionGatewayTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_FunctionGateway *FunctionGatewaySession) RenounceOwnership() (*types.Transaction, error) {
	return _FunctionGateway.Contract.RenounceOwnership(&_FunctionGateway.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _FunctionGateway.Contract.RenounceOwnership(&_FunctionGateway.TransactOpts)
}

// Request is a paid mutator transaction binding the contract method 0x8b4d7bc4.
//
// Solidity: function request(bytes32 _functionId, bytes _input, bytes4 _callbackSelector, bytes _context, uint256 _gasLimit, address _refundAccount) payable returns(bytes32)
func (_FunctionGateway *FunctionGatewayTransactor) Request(opts *bind.TransactOpts, _functionId [32]byte, _input []byte, _callbackSelector [4]byte, _context []byte, _gasLimit *big.Int, _refundAccount common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "request", _functionId, _input, _callbackSelector, _context, _gasLimit, _refundAccount)
}

// Request is a paid mutator transaction binding the contract method 0x8b4d7bc4.
//
// Solidity: function request(bytes32 _functionId, bytes _input, bytes4 _callbackSelector, bytes _context, uint256 _gasLimit, address _refundAccount) payable returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) Request(_functionId [32]byte, _input []byte, _callbackSelector [4]byte, _context []byte, _gasLimit *big.Int, _refundAccount common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Request(&_FunctionGateway.TransactOpts, _functionId, _input, _callbackSelector, _context, _gasLimit, _refundAccount)
}

// Request is a paid mutator transaction binding the contract method 0x8b4d7bc4.
//
// Solidity: function request(bytes32 _functionId, bytes _input, bytes4 _callbackSelector, bytes _context, uint256 _gasLimit, address _refundAccount) payable returns(bytes32)
func (_FunctionGateway *FunctionGatewayTransactorSession) Request(_functionId [32]byte, _input []byte, _callbackSelector [4]byte, _context []byte, _gasLimit *big.Int, _refundAccount common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Request(&_FunctionGateway.TransactOpts, _functionId, _input, _callbackSelector, _context, _gasLimit, _refundAccount)
}

// Request0 is a paid mutator transaction binding the contract method 0xe2362c31.
//
// Solidity: function request(bytes32 _functionId, bytes _input, bytes4 _callbackSelector, bytes _context) payable returns(bytes32)
func (_FunctionGateway *FunctionGatewayTransactor) Request0(opts *bind.TransactOpts, _functionId [32]byte, _input []byte, _callbackSelector [4]byte, _context []byte) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "request0", _functionId, _input, _callbackSelector, _context)
}

// Request0 is a paid mutator transaction binding the contract method 0xe2362c31.
//
// Solidity: function request(bytes32 _functionId, bytes _input, bytes4 _callbackSelector, bytes _context) payable returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) Request0(_functionId [32]byte, _input []byte, _callbackSelector [4]byte, _context []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Request0(&_FunctionGateway.TransactOpts, _functionId, _input, _callbackSelector, _context)
}

// Request0 is a paid mutator transaction binding the contract method 0xe2362c31.
//
// Solidity: function request(bytes32 _functionId, bytes _input, bytes4 _callbackSelector, bytes _context) payable returns(bytes32)
func (_FunctionGateway *FunctionGatewayTransactorSession) Request0(_functionId [32]byte, _input []byte, _callbackSelector [4]byte, _context []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Request0(&_FunctionGateway.TransactOpts, _functionId, _input, _callbackSelector, _context)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_FunctionGateway *FunctionGatewayTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_FunctionGateway *FunctionGatewaySession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.TransferOwnership(&_FunctionGateway.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.TransferOwnership(&_FunctionGateway.TransactOpts, newOwner)
}

// UpdateFunctionOwner is a paid mutator transaction binding the contract method 0xe7ddf4c6.
//
// Solidity: function updateFunctionOwner(bytes32 _functionId, address _owner) returns()
func (_FunctionGateway *FunctionGatewayTransactor) UpdateFunctionOwner(opts *bind.TransactOpts, _functionId [32]byte, _owner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "updateFunctionOwner", _functionId, _owner)
}

// UpdateFunctionOwner is a paid mutator transaction binding the contract method 0xe7ddf4c6.
//
// Solidity: function updateFunctionOwner(bytes32 _functionId, address _owner) returns()
func (_FunctionGateway *FunctionGatewaySession) UpdateFunctionOwner(_functionId [32]byte, _owner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateFunctionOwner(&_FunctionGateway.TransactOpts, _functionId, _owner)
}

// UpdateFunctionOwner is a paid mutator transaction binding the contract method 0xe7ddf4c6.
//
// Solidity: function updateFunctionOwner(bytes32 _functionId, address _owner) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) UpdateFunctionOwner(_functionId [32]byte, _owner common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateFunctionOwner(&_FunctionGateway.TransactOpts, _functionId, _owner)
}

// UpdateFunctionVerifier is a paid mutator transaction binding the contract method 0x6f652c95.
//
// Solidity: function updateFunctionVerifier(bytes32 _functionId, address _verifier) returns()
func (_FunctionGateway *FunctionGatewayTransactor) UpdateFunctionVerifier(opts *bind.TransactOpts, _functionId [32]byte, _verifier common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "updateFunctionVerifier", _functionId, _verifier)
}

// UpdateFunctionVerifier is a paid mutator transaction binding the contract method 0x6f652c95.
//
// Solidity: function updateFunctionVerifier(bytes32 _functionId, address _verifier) returns()
func (_FunctionGateway *FunctionGatewaySession) UpdateFunctionVerifier(_functionId [32]byte, _verifier common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateFunctionVerifier(&_FunctionGateway.TransactOpts, _functionId, _verifier)
}

// UpdateFunctionVerifier is a paid mutator transaction binding the contract method 0x6f652c95.
//
// Solidity: function updateFunctionVerifier(bytes32 _functionId, address _verifier) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) UpdateFunctionVerifier(_functionId [32]byte, _verifier common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateFunctionVerifier(&_FunctionGateway.TransactOpts, _functionId, _verifier)
}

// UpdateScalar is a paid mutator transaction binding the contract method 0xe23b0410.
//
// Solidity: function updateScalar(uint256 _scalar) returns()
func (_FunctionGateway *FunctionGatewayTransactor) UpdateScalar(opts *bind.TransactOpts, _scalar *big.Int) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "updateScalar", _scalar)
}

// UpdateScalar is a paid mutator transaction binding the contract method 0xe23b0410.
//
// Solidity: function updateScalar(uint256 _scalar) returns()
func (_FunctionGateway *FunctionGatewaySession) UpdateScalar(_scalar *big.Int) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateScalar(&_FunctionGateway.TransactOpts, _scalar)
}

// UpdateScalar is a paid mutator transaction binding the contract method 0xe23b0410.
//
// Solidity: function updateScalar(uint256 _scalar) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) UpdateScalar(_scalar *big.Int) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateScalar(&_FunctionGateway.TransactOpts, _scalar)
}

// FunctionGatewayCallbackFulfilledIterator is returned from FilterCallbackFulfilled and is used to iterate over the raw logs and unpacked data for CallbackFulfilled events raised by the FunctionGateway contract.
type FunctionGatewayCallbackFulfilledIterator struct {
	Event *FunctionGatewayCallbackFulfilled // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FunctionGatewayCallbackFulfilledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayCallbackFulfilled)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FunctionGatewayCallbackFulfilled)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FunctionGatewayCallbackFulfilledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayCallbackFulfilledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayCallbackFulfilled represents a CallbackFulfilled event raised by the FunctionGateway contract.
type FunctionGatewayCallbackFulfilled struct {
	RequestId [32]byte
	Output    []byte
	Context   []byte
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterCallbackFulfilled is a free log retrieval operation binding the contract event 0x4157c302cad5507e9c624680b653ae4a290e304cb0ff86a730bceda763ec878d.
//
// Solidity: event CallbackFulfilled(bytes32 requestId, bytes output, bytes context)
func (_FunctionGateway *FunctionGatewayFilterer) FilterCallbackFulfilled(opts *bind.FilterOpts) (*FunctionGatewayCallbackFulfilledIterator, error) {

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "CallbackFulfilled")
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayCallbackFulfilledIterator{contract: _FunctionGateway.contract, event: "CallbackFulfilled", logs: logs, sub: sub}, nil
}

// WatchCallbackFulfilled is a free log subscription operation binding the contract event 0x4157c302cad5507e9c624680b653ae4a290e304cb0ff86a730bceda763ec878d.
//
// Solidity: event CallbackFulfilled(bytes32 requestId, bytes output, bytes context)
func (_FunctionGateway *FunctionGatewayFilterer) WatchCallbackFulfilled(opts *bind.WatchOpts, sink chan<- *FunctionGatewayCallbackFulfilled) (event.Subscription, error) {

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "CallbackFulfilled")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayCallbackFulfilled)
				if err := _FunctionGateway.contract.UnpackLog(event, "CallbackFulfilled", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseCallbackFulfilled is a log parse operation binding the contract event 0x4157c302cad5507e9c624680b653ae4a290e304cb0ff86a730bceda763ec878d.
//
// Solidity: event CallbackFulfilled(bytes32 requestId, bytes output, bytes context)
func (_FunctionGateway *FunctionGatewayFilterer) ParseCallbackFulfilled(log types.Log) (*FunctionGatewayCallbackFulfilled, error) {
	event := new(FunctionGatewayCallbackFulfilled)
	if err := _FunctionGateway.contract.UnpackLog(event, "CallbackFulfilled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the FunctionGateway contract.
type FunctionGatewayOwnershipTransferredIterator struct {
	Event *FunctionGatewayOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FunctionGatewayOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FunctionGatewayOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FunctionGatewayOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayOwnershipTransferred represents a OwnershipTransferred event raised by the FunctionGateway contract.
type FunctionGatewayOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_FunctionGateway *FunctionGatewayFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*FunctionGatewayOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayOwnershipTransferredIterator{contract: _FunctionGateway.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_FunctionGateway *FunctionGatewayFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *FunctionGatewayOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayOwnershipTransferred)
				if err := _FunctionGateway.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_FunctionGateway *FunctionGatewayFilterer) ParseOwnershipTransferred(log types.Log) (*FunctionGatewayOwnershipTransferred, error) {
	event := new(FunctionGatewayOwnershipTransferred)
	if err := _FunctionGateway.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayProofBatchFulfilledIterator is returned from FilterProofBatchFulfilled and is used to iterate over the raw logs and unpacked data for ProofBatchFulfilled events raised by the FunctionGateway contract.
type FunctionGatewayProofBatchFulfilledIterator struct {
	Event *FunctionGatewayProofBatchFulfilled // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FunctionGatewayProofBatchFulfilledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayProofBatchFulfilled)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FunctionGatewayProofBatchFulfilled)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FunctionGatewayProofBatchFulfilledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayProofBatchFulfilledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayProofBatchFulfilled represents a ProofBatchFulfilled event raised by the FunctionGateway contract.
type FunctionGatewayProofBatchFulfilled struct {
	RequestIds          [][32]byte
	AggregateProof      []byte
	InputsRoot          [32]byte
	OutputHashes        [][32]byte
	OutputsRoot         [32]byte
	VerificationKeyRoot [32]byte
	Raw                 types.Log // Blockchain specific contextual infos
}

// FilterProofBatchFulfilled is a free log retrieval operation binding the contract event 0x9f5bcf5fecad905a6b02f0a6c02a52568005592a0d6c0711752b20ca854e2302.
//
// Solidity: event ProofBatchFulfilled(bytes32[] requestIds, bytes aggregateProof, bytes32 inputsRoot, bytes32[] outputHashes, bytes32 outputsRoot, bytes32 verificationKeyRoot)
func (_FunctionGateway *FunctionGatewayFilterer) FilterProofBatchFulfilled(opts *bind.FilterOpts) (*FunctionGatewayProofBatchFulfilledIterator, error) {

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "ProofBatchFulfilled")
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayProofBatchFulfilledIterator{contract: _FunctionGateway.contract, event: "ProofBatchFulfilled", logs: logs, sub: sub}, nil
}

// WatchProofBatchFulfilled is a free log subscription operation binding the contract event 0x9f5bcf5fecad905a6b02f0a6c02a52568005592a0d6c0711752b20ca854e2302.
//
// Solidity: event ProofBatchFulfilled(bytes32[] requestIds, bytes aggregateProof, bytes32 inputsRoot, bytes32[] outputHashes, bytes32 outputsRoot, bytes32 verificationKeyRoot)
func (_FunctionGateway *FunctionGatewayFilterer) WatchProofBatchFulfilled(opts *bind.WatchOpts, sink chan<- *FunctionGatewayProofBatchFulfilled) (event.Subscription, error) {

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "ProofBatchFulfilled")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayProofBatchFulfilled)
				if err := _FunctionGateway.contract.UnpackLog(event, "ProofBatchFulfilled", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseProofBatchFulfilled is a log parse operation binding the contract event 0x9f5bcf5fecad905a6b02f0a6c02a52568005592a0d6c0711752b20ca854e2302.
//
// Solidity: event ProofBatchFulfilled(bytes32[] requestIds, bytes aggregateProof, bytes32 inputsRoot, bytes32[] outputHashes, bytes32 outputsRoot, bytes32 verificationKeyRoot)
func (_FunctionGateway *FunctionGatewayFilterer) ParseProofBatchFulfilled(log types.Log) (*FunctionGatewayProofBatchFulfilled, error) {
	event := new(FunctionGatewayProofBatchFulfilled)
	if err := _FunctionGateway.contract.UnpackLog(event, "ProofBatchFulfilled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayProofFulfilledIterator is returned from FilterProofFulfilled and is used to iterate over the raw logs and unpacked data for ProofFulfilled events raised by the FunctionGateway contract.
type FunctionGatewayProofFulfilledIterator struct {
	Event *FunctionGatewayProofFulfilled // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FunctionGatewayProofFulfilledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayProofFulfilled)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FunctionGatewayProofFulfilled)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FunctionGatewayProofFulfilledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayProofFulfilledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayProofFulfilled represents a ProofFulfilled event raised by the FunctionGateway contract.
type FunctionGatewayProofFulfilled struct {
	RequestId  [32]byte
	OutputHash [32]byte
	Proof      []byte
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterProofFulfilled is a free log retrieval operation binding the contract event 0xfddf097ddc1205e34fd4700d12ad51b32ccad4f117f7ac879a74d20b145209b4.
//
// Solidity: event ProofFulfilled(bytes32 requestId, bytes32 outputHash, bytes proof)
func (_FunctionGateway *FunctionGatewayFilterer) FilterProofFulfilled(opts *bind.FilterOpts) (*FunctionGatewayProofFulfilledIterator, error) {

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "ProofFulfilled")
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayProofFulfilledIterator{contract: _FunctionGateway.contract, event: "ProofFulfilled", logs: logs, sub: sub}, nil
}

// WatchProofFulfilled is a free log subscription operation binding the contract event 0xfddf097ddc1205e34fd4700d12ad51b32ccad4f117f7ac879a74d20b145209b4.
//
// Solidity: event ProofFulfilled(bytes32 requestId, bytes32 outputHash, bytes proof)
func (_FunctionGateway *FunctionGatewayFilterer) WatchProofFulfilled(opts *bind.WatchOpts, sink chan<- *FunctionGatewayProofFulfilled) (event.Subscription, error) {

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "ProofFulfilled")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayProofFulfilled)
				if err := _FunctionGateway.contract.UnpackLog(event, "ProofFulfilled", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseProofFulfilled is a log parse operation binding the contract event 0xfddf097ddc1205e34fd4700d12ad51b32ccad4f117f7ac879a74d20b145209b4.
//
// Solidity: event ProofFulfilled(bytes32 requestId, bytes32 outputHash, bytes proof)
func (_FunctionGateway *FunctionGatewayFilterer) ParseProofFulfilled(log types.Log) (*FunctionGatewayProofFulfilled, error) {
	event := new(FunctionGatewayProofFulfilled)
	if err := _FunctionGateway.contract.UnpackLog(event, "ProofFulfilled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayProofRequestedIterator is returned from FilterProofRequested and is used to iterate over the raw logs and unpacked data for ProofRequested events raised by the FunctionGateway contract.
type FunctionGatewayProofRequestedIterator struct {
	Event *FunctionGatewayProofRequested // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FunctionGatewayProofRequestedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayProofRequested)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FunctionGatewayProofRequested)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FunctionGatewayProofRequestedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayProofRequestedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayProofRequested represents a ProofRequested event raised by the FunctionGateway contract.
type FunctionGatewayProofRequested struct {
	Nonce     *big.Int
	RequestId [32]byte
	Inputs    []byte
	Context   []byte
	GasLimit  *big.Int
	FeeAmount *big.Int
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterProofRequested is a free log retrieval operation binding the contract event 0x78248b3a4298ac22184cd31fb142c8da8cf242175e64a58fc06b900dce6aeb91.
//
// Solidity: event ProofRequested(uint256 indexed nonce, bytes32 indexed requestId, bytes inputs, bytes context, uint256 gasLimit, uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayFilterer) FilterProofRequested(opts *bind.FilterOpts, nonce []*big.Int, requestId [][32]byte) (*FunctionGatewayProofRequestedIterator, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var requestIdRule []interface{}
	for _, requestIdItem := range requestId {
		requestIdRule = append(requestIdRule, requestIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "ProofRequested", nonceRule, requestIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayProofRequestedIterator{contract: _FunctionGateway.contract, event: "ProofRequested", logs: logs, sub: sub}, nil
}

// WatchProofRequested is a free log subscription operation binding the contract event 0x78248b3a4298ac22184cd31fb142c8da8cf242175e64a58fc06b900dce6aeb91.
//
// Solidity: event ProofRequested(uint256 indexed nonce, bytes32 indexed requestId, bytes inputs, bytes context, uint256 gasLimit, uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayFilterer) WatchProofRequested(opts *bind.WatchOpts, sink chan<- *FunctionGatewayProofRequested, nonce []*big.Int, requestId [][32]byte) (event.Subscription, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var requestIdRule []interface{}
	for _, requestIdItem := range requestId {
		requestIdRule = append(requestIdRule, requestIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "ProofRequested", nonceRule, requestIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayProofRequested)
				if err := _FunctionGateway.contract.UnpackLog(event, "ProofRequested", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseProofRequested is a log parse operation binding the contract event 0x78248b3a4298ac22184cd31fb142c8da8cf242175e64a58fc06b900dce6aeb91.
//
// Solidity: event ProofRequested(uint256 indexed nonce, bytes32 indexed requestId, bytes inputs, bytes context, uint256 gasLimit, uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayFilterer) ParseProofRequested(log types.Log) (*FunctionGatewayProofRequested, error) {
	event := new(FunctionGatewayProofRequested)
	if err := _FunctionGateway.contract.UnpackLog(event, "ProofRequested", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayScalarUpdatedIterator is returned from FilterScalarUpdated and is used to iterate over the raw logs and unpacked data for ScalarUpdated events raised by the FunctionGateway contract.
type FunctionGatewayScalarUpdatedIterator struct {
	Event *FunctionGatewayScalarUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FunctionGatewayScalarUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayScalarUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FunctionGatewayScalarUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FunctionGatewayScalarUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayScalarUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayScalarUpdated represents a ScalarUpdated event raised by the FunctionGateway contract.
type FunctionGatewayScalarUpdated struct {
	Scalar *big.Int
	Raw    types.Log // Blockchain specific contextual infos
}

// FilterScalarUpdated is a free log retrieval operation binding the contract event 0x3336cd9708eaf2769a0f0dc0679f30e80f15dcd88d1921b5a16858e8b85c591a.
//
// Solidity: event ScalarUpdated(uint256 scalar)
func (_FunctionGateway *FunctionGatewayFilterer) FilterScalarUpdated(opts *bind.FilterOpts) (*FunctionGatewayScalarUpdatedIterator, error) {

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "ScalarUpdated")
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayScalarUpdatedIterator{contract: _FunctionGateway.contract, event: "ScalarUpdated", logs: logs, sub: sub}, nil
}

// WatchScalarUpdated is a free log subscription operation binding the contract event 0x3336cd9708eaf2769a0f0dc0679f30e80f15dcd88d1921b5a16858e8b85c591a.
//
// Solidity: event ScalarUpdated(uint256 scalar)
func (_FunctionGateway *FunctionGatewayFilterer) WatchScalarUpdated(opts *bind.WatchOpts, sink chan<- *FunctionGatewayScalarUpdated) (event.Subscription, error) {

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "ScalarUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayScalarUpdated)
				if err := _FunctionGateway.contract.UnpackLog(event, "ScalarUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseScalarUpdated is a log parse operation binding the contract event 0x3336cd9708eaf2769a0f0dc0679f30e80f15dcd88d1921b5a16858e8b85c591a.
//
// Solidity: event ScalarUpdated(uint256 scalar)
func (_FunctionGateway *FunctionGatewayFilterer) ParseScalarUpdated(log types.Log) (*FunctionGatewayScalarUpdated, error) {
	event := new(FunctionGatewayScalarUpdated)
	if err := _FunctionGateway.contract.UnpackLog(event, "ScalarUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

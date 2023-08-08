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
	ABI: "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scalar\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_feeVault\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"CallbackAlreadyFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"callbackAddress\",\"type\":\"address\"},{\"internalType\":\"bytes4\",\"name\":\"callbackSelector\",\"type\":\"bytes4\"}],\"name\":\"CallbackFailed\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"contextHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"ContextMismatch\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"EmptyBytecode\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"FailedDeploy\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"name\":\"FunctionAlreadyRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"inputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"InputsRootMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expected\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"actual\",\"type\":\"uint256\"}],\"name\":\"InsufficientFeeAmount\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"inputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"name\":\"InvalidProof\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expected\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"actual\",\"type\":\"uint256\"}],\"name\":\"LengthMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"actualOwner\",\"type\":\"address\"}],\"name\":\"NotFunctionOwner\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"OutputMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"OutputsRootMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"ProofAlreadyFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"ProofNotFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refundAccount\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"refundAmount\",\"type\":\"uint256\"}],\"name\":\"RefundFailed\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"RequestNotFound\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"VerificationKeysRootMismatch\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"VerifierCannotBeZero\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"output\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"CallbackFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"bytecodeHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"deployedAddress\",\"type\":\"address\"}],\"name\":\"Deployed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"FunctionOwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"FunctionRegistered\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"name\":\"FunctionVerifierUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32[]\",\"name\":\"requestIds\",\"type\":\"bytes32[]\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"aggregateProof\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"inputsRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"verificationKeyRoot\",\"type\":\"bytes32\"}],\"name\":\"ProofBatchFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"name\":\"ProofFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"inputs\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"name\":\"ProofRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"scalar\",\"type\":\"uint256\"}],\"name\":\"ScalarUpdated\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"AGGREGATION_FUNCTION_ID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DEFAULT_GAS_LIMIT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\"}],\"name\":\"calculateFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"calculateFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_requestId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_output\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"callback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_bytecode\",\"type\":\"bytes\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"deployAndRegisterFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_bytecode\",\"type\":\"bytes\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"deployAndUpdateFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"feeVault\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_requestId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"}],\"name\":\"fulfill\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_requestIds\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes\",\"name\":\"_aggregateProof\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_inputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"_outputHashes\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32\",\"name\":\"_outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_verificationKeyRoot\",\"type\":\"bytes32\"}],\"name\":\"fulfillBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"getFunctionId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"registerFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_input\",\"type\":\"bytes\"},{\"internalType\":\"bytes4\",\"name\":\"_callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_refundAccount\",\"type\":\"address\"}],\"name\":\"request\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_input\",\"type\":\"bytes\"},{\"internalType\":\"bytes4\",\"name\":\"_callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"request\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"requests\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"inputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"contextHash\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"callbackAddress\",\"type\":\"address\"},{\"internalType\":\"bytes4\",\"name\":\"callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bool\",\"name\":\"proofFulfilled\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"callbackFulfilled\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"scalar\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"updateFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scalar\",\"type\":\"uint256\"}],\"name\":\"updateScalar\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifierOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifiers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
	Bin: "0x60a0604052620f42406003553480156200001857600080fd5b506040516200215e3803806200215e8339810160408190526200003b91620000db565b62000046336200006c565b60068390556001600160a01b03821660805262000063816200006c565b5050506200011c565b600280546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b80516001600160a01b0381168114620000d657600080fd5b919050565b600080600060608486031215620000f157600080fd5b835192506200010360208501620000be565b91506200011360408501620000be565b90509250925092565b60805161201862000146600039600081816101eb0152818161162a015261167901526120186000f3fe60806040526004361061014b5760003560e01c80639538f56f116100b6578063d6be695a1161006f578063d6be695a1461048f578063e2362c31146104a5578063e23b0410146104b8578063efe1c950146104d8578063f2fde38b1461050e578063f45e65d81461052e57600080fd5b80639538f56f1461033e5780639d8669851461035e578063affed0e014610424578063b63755e51461043a578063bd58c4bb1461045a578063c30d98261461047a57600080fd5b8063715018a611610108578063715018a61461028257806387c5621a146102975780638ab4be9e146102b75780638b4d7bc4146102d75780638bcfc3a0146102ea5780638da5cb5b1461032057600080fd5b8063178f7b401461015057806337ea8847146101835780633bb60039146101a5578063478222c2146101d95780635c74ad561461022557806368ff41b114610262575b600080fd5b34801561015c57600080fd5b5061017061016b36600461177d565b610544565b6040519081526020015b60405180910390f35b34801561018f57600080fd5b506101a361019e3660046118cd565b61057c565b005b3480156101b157600080fd5b506101707fcf91d3a65d6f619b1560b4409a7377da358299d073f6633a90fe3313a88b47f581565b3480156101e557600080fd5b5061020d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161017a565b34801561023157600080fd5b50610245610240366004611971565b610a57565b604080519283526001600160a01b0390911660208301520161017a565b34801561026e57600080fd5b5061017061027d3660046119ec565b610b37565b34801561028e57600080fd5b506101a3610c25565b3480156102a357600080fd5b506101a36102b2366004611a30565b610c39565b3480156102c357600080fd5b506101a36102d2366004611a80565b610db5565b6101706102e5366004611afb565b610ff2565b3480156102f657600080fd5b5061020d61030536600461177d565b6001602052600090815260409020546001600160a01b031681565b34801561032c57600080fd5b506002546001600160a01b031661020d565b34801561034a57600080fd5b506101706103593660046119ec565b6111c8565b34801561036a57600080fd5b506103d261037936600461177d565b60056020526000908152604090208054600182015460028301546003840154600490940154929391929091906001600160a01b03811690600160a01b810460e01b9060ff600160c01b8204811691600160c81b90041688565b6040805198895260208901979097529587019490945260608601929092526001600160a01b031660808501526001600160e01b03191660a0840152151560c0830152151560e08201526101000161017a565b34801561043057600080fd5b5061017060045481565b34801561044657600080fd5b50610245610455366004611971565b6111fb565b34801561046657600080fd5b506101706104753660046119ec565b6112cc565b34801561048657600080fd5b506101706113b6565b34801561049b57600080fd5b5061017060035481565b6101706104b3366004611b92565b6113c8565b3480156104c457600080fd5b506101a36104d336600461177d565b6113e3565b3480156104e457600080fd5b5061020d6104f336600461177d565b6000602081905290815260409020546001600160a01b031681565b34801561051a57600080fd5b506101a3610529366004611c10565b611426565b34801561053a57600080fd5b5061017060065481565b60006006546000036105605761055a823a611c48565b92915050565b60065461056d833a611c48565b61055a9190611c48565b919050565b6000865167ffffffffffffffff81111561059857610598611796565b6040519080825280602002602001820160405280156105c1578160200160208202803683370190505b5090506000875167ffffffffffffffff8111156105e0576105e0611796565b604051908082528060200260200182016040528015610609578160200160208202803683370190505b50905060005b885181101561077f57600089828151811061062c5761062c611c5f565b6020908102919091018101516000818152600590925260409091206004810154919250906001600160a01b031661067e57604051630d1c383160e11b8152600481018390526024015b60405180910390fd5b6004810154600160c01b900460ff16156106ae57604051631aea9acb60e31b815260048101839052602401610675565b80600101548584815181106106c5576106c5611c5f565b60209081029190910181019190915281546000908152808252604090819020548151634f44a2e960e11b815291516001600160a01b03909116928392639e8945d292600480830193928290030181865afa158015610727573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061074b9190611c75565b85858151811061075d5761075d611c5f565b602002602001018181525050505050808061077790611c8e565b91505061060f565b5084518851146107af57875185516040516355c5b3e360e11b815260048101929092526024820152604401610675565b816040516020016107c09190611ce2565b6040516020818303038152906040528051906020012086146107f957858260405163e2920b9160e01b8152600401610675929190611cf5565b8460405160200161080a9190611ce2565b604051602081830303815290604052805190602001208414610843578385604051637ccf42f360e01b8152600401610675929190611cf5565b806040516020016108549190611ce2565b60405160208183030381529060405280519060200120831461088d57828160405163693d503560e11b8152600401610675929190611cf5565b60005b88518110156109245760008982815181106108ad576108ad611c5f565b602090810291909101810151600081815260059092526040909120600401805460ff60c01b1916600160c01b17905587519091508790839081106108f3576108f3611c5f565b602090810291909101810151600092835260059091526040909120600201558061091c81611c8e565b915050610890565b507fcf91d3a65d6f619b1560b4409a7377da358299d073f6633a90fe3313a88b47f560009081526020527fdfbb683d42ec23abfd9b50088f945b2feb0772147412dcd9441f8e87a3f0ff9e546040516303784b1960e61b81526001600160a01b0390911690819063de12c640906109a3908a9089908d90600401611d66565b6020604051808303816000875af11580156109c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109e69190611d85565b610a0b578087868a6040516316c7141360e31b81526004016106759493929190611da7565b7f9f5bcf5fecad905a6b02f0a6c02a52568005592a0d6c0711752b20ca854e2302898989898989604051610a4496959493929190611dde565b60405180910390a1505050505050505050565b600080610a6433846111c8565b6000818152602081905260409020549092506001600160a01b031615610aa057604051635e34c78f60e01b815260048101839052602401610675565b600082815260016020526040902080546001600160a01b03191633179055610ac8848361149f565b6000838152602081905260409081902080546001600160a01b0319166001600160a01b0384161790555190915082907f52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea90610b2890849087903390611e33565b60405180910390a29250929050565b6000610b4333836111c8565b6000818152602081905260409020549091506001600160a01b031615610b7f57604051635e34c78f60e01b815260048101829052602401610675565b6001600160a01b038316610ba6576040516302d48d1f60e61b815260040160405180910390fd5b60008181526020818152604080832080546001600160a01b03199081166001600160a01b03891617909155600190925291829020805433921682179055905182917f52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea91610c17918791879190611e33565b60405180910390a292915050565b610c2d611539565b610c376000611593565b565b600083815260056020526040902060048101546001600160a01b0316610c7557604051630d1c383160e11b815260048101859052602401610675565b6004810154600160c01b900460ff1615610ca557604051631aea9acb60e31b815260048101859052602401610675565b6004808201805460ff60c01b1916600160c01b1790556002820184905581546000908152602081905260409081902054600184015491516303784b1960e61b81526001600160a01b0390911692839263de12c64092610d08928991899101611d66565b6020604051808303816000875af1158015610d27573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d4b9190611d85565b610d735760018201546040516316c7141360e31b815261067591839187908790600401611da7565b7ffddf097ddc1205e34fd4700d12ad51b32ccad4f117f7ac879a74d20b145209b4858585604051610da693929190611d66565b60405180910390a15050505050565b60008381526005602052604090206004810154600160c81b900460ff1615610df35760405163b08540e560e01b815260048101859052602401610675565b60048101546001600160a01b0316610e2157604051630d1c383160e11b815260048101859052602401610675565b81516020830120600382015414610e4f5783826040516389116ecd60e01b8152600401610675929190611e68565b82516020840120600282015414610e7d578383604051633cfc30e360e01b8152600401610675929190611e68565b6004810154600160c01b900460ff16610eac57604051635ca8297160e11b815260048101859052602401610675565b60048101805460ff60c81b198116600160c81b17918290556040516000926001600160a01b0390921691600160a01b900460e01b90610ef19087908790602401611e81565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610f2f9190611ea6565b6000604051808303816000865af19150503d8060008114610f6c576040519150601f19603f3d011682016040523d82523d6000602084013e610f71565b606091505b5050905080610fbf5760048281015460405163bc4a234960e01b81526001600160a01b03821692810192909252600160a01b900460e01b6001600160e01b0319166024820152604401610675565b7f4157c302cad5507e9c624680b653ae4a290e304cb0ff86a730bceda763ec878d858585604051610da693929190611ec2565b845160208087019190912084518583012060408051610100810182528a815293840183905260009084018190526060840182905233608085018190526001600160e01b0319891660a086015260c0850182905260e08501829052909390849061105f9088908890346115e5565b9050600060045483604051602001611078929190611eed565b60405160208183030381529060405280519060200120905082600560008381526020019081526020016000206000820151816000015560208201518160010155604082015181600201556060820151816003015560808201518160040160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060a08201518160040160146101000a81548163ffffffff021916908360e01c021790555060c08201518160040160186101000a81548160ff02191690831515021790555060e08201518160040160196101000a81548160ff0219169083151502179055509050508b6004547f3fb5c9bd4c90dcd3781879795c37f8645d9421602f4ba57c651f3005938c7260838e8d8d8860405161119b959493929190611f6b565b60405180910390a3600480549060006111b383611c8e565b90915550909c9b505050505050505050505050565b600082826040516020016111dd929190611fab565b60405160208183030381529060405280519060200120905092915050565b60008061120833846111c8565b6000818152600160205260409020549092506001600160a01b031633146112645760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610675565b61126e848361149f565b6000838152602081815260409182902080546001600160a01b0319166001600160a01b038516908117909155915191825291925083917ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369101610b28565b60006112d833836111c8565b6000818152600160205260409020549091506001600160a01b031633146113345760008181526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610675565b6001600160a01b03831661135b576040516302d48d1f60e61b815260040160405180910390fd5b6000818152602081815260409182902080546001600160a01b0319166001600160a01b038716908117909155915191825282917ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369101610c17565b60006113c3600354610544565b905090565b60006113da8585858560035432610ff2565b95945050505050565b6113eb611539565b60068190556040518181527f3336cd9708eaf2769a0f0dc0679f30e80f15dcd88d1921b5a16858e8b85c591a9060200160405180910390a150565b61142e611539565b6001600160a01b0381166114935760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610675565b61149c81611593565b50565b600082516000036114c3576040516321744a5960e01b815260040160405180910390fd5b818351602085016000f590506001600160a01b0381166114f657604051632081741d60e11b815260040160405180910390fd5b825160208401206040516001600160a01b0383169184917f27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f53290600090a492915050565b6002546001600160a01b03163314610c375760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610675565b600280546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60006115f085610544565b90508082101561161d57604051630961b65b60e41b81526004810182905260248101839052604401610675565b60008111801561165557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031615155b156116d8576040516333bb7f9160e01b81526001600160a01b0384811660048301527f000000000000000000000000000000000000000000000000000000000000000016906333bb7f919083906024016000604051808303818588803b1580156116be57600080fd5b505af11580156116d2573d6000803e3d6000fd5b50505050505b60006116e48284611fcf565b90508015611774576000856001600160a01b03168260405160006040518083038185875af1925050503d8060008114611739576040519150601f19603f3d011682016040523d82523d6000602084013e61173e565b606091505b5050905080611772576040516357b9d85960e11b81526001600160a01b038716600482015260248101839052604401610675565b505b50949350505050565b60006020828403121561178f57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156117d5576117d5611796565b604052919050565b600082601f8301126117ee57600080fd5b8135602067ffffffffffffffff82111561180a5761180a611796565b8160051b6118198282016117ac565b928352848101820192828101908785111561183357600080fd5b83870192505b8483101561185257823582529183019190830190611839565b979650505050505050565b600082601f83011261186e57600080fd5b813567ffffffffffffffff81111561188857611888611796565b61189b601f8201601f19166020016117ac565b8181528460208386010111156118b057600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060008060c087890312156118e657600080fd5b863567ffffffffffffffff808211156118fe57600080fd5b61190a8a838b016117dd565b9750602089013591508082111561192057600080fd5b61192c8a838b0161185d565b965060408901359550606089013591508082111561194957600080fd5b5061195689828a016117dd565b9350506080870135915060a087013590509295509295509295565b6000806040838503121561198457600080fd5b823567ffffffffffffffff8082111561199c57600080fd5b6119a88683870161185d565b935060208501359150808211156119be57600080fd5b506119cb8582860161185d565b9150509250929050565b80356001600160a01b038116811461057757600080fd5b600080604083850312156119ff57600080fd5b611a08836119d5565b9150602083013567ffffffffffffffff811115611a2457600080fd5b6119cb8582860161185d565b600080600060608486031215611a4557600080fd5b8335925060208401359150604084013567ffffffffffffffff811115611a6a57600080fd5b611a768682870161185d565b9150509250925092565b600080600060608486031215611a9557600080fd5b83359250602084013567ffffffffffffffff80821115611ab457600080fd5b611ac08783880161185d565b93506040860135915080821115611ad657600080fd5b50611a768682870161185d565b80356001600160e01b03198116811461057757600080fd5b60008060008060008060c08789031215611b1457600080fd5b86359550602087013567ffffffffffffffff80821115611b3357600080fd5b611b3f8a838b0161185d565b9650611b4d60408a01611ae3565b95506060890135915080821115611b6357600080fd5b50611b7089828a0161185d565b93505060808701359150611b8660a088016119d5565b90509295509295509295565b60008060008060808587031215611ba857600080fd5b84359350602085013567ffffffffffffffff80821115611bc757600080fd5b611bd38883890161185d565b9450611be160408801611ae3565b93506060870135915080821115611bf757600080fd5b50611c048782880161185d565b91505092959194509250565b600060208284031215611c2257600080fd5b611c2b826119d5565b9392505050565b634e487b7160e01b600052601160045260246000fd5b808202811582820484141761055a5761055a611c32565b634e487b7160e01b600052603260045260246000fd5b600060208284031215611c8757600080fd5b5051919050565b600060018201611ca057611ca0611c32565b5060010190565b600081518084526020808501945080840160005b83811015611cd757815187529582019590820190600101611cbb565b509495945050505050565b602081526000611c2b6020830184611ca7565b828152604060208201526000611d0e6040830184611ca7565b949350505050565b60005b83811015611d31578181015183820152602001611d19565b50506000910152565b60008151808452611d52816020860160208601611d16565b601f01601f19169290920160200192915050565b8381528260208201526060604082015260006113da6060830184611d3a565b600060208284031215611d9757600080fd5b81518015158114611c2b57600080fd5b60018060a01b0385168152836020820152826040820152608060608201526000611dd46080830184611d3a565b9695505050505050565b60c081526000611df160c0830189611ca7565b8281036020840152611e038189611d3a565b90508660408401528281036060840152611e1d8187611ca7565b6080840195909552505060a00152949350505050565b600060018060a01b03808616835260606020840152611e556060840186611d3a565b9150808416604084015250949350505050565b828152604060208201526000611d0e6040830184611d3a565b604081526000611e946040830185611d3a565b82810360208401526113da8185611d3a565b60008251611eb8818460208701611d16565b9190910192915050565b838152606060208201526000611edb6060830185611d3a565b8281036040840152611dd48185611d3a565b6000610120820190508382528251602083015260208301516040830152604083015160608301526060830151608083015260018060a01b0360808401511660a083015263ffffffff60e01b60a08401511660c083015260c0830151151560e083015260e0830151611f6361010084018215159052565b509392505050565b85815260a060208201526000611f8460a0830187611d3a565b8281036040840152611f968187611d3a565b60608401959095525050608001529392505050565b6001600160a01b0383168152604060208201819052600090611d0e90830184611d3a565b8181038181111561055a5761055a611c3256fea26469706673582212207e05ad42c0fb91e92fd132549cb1e80b79854d0d2e569bacaa03014061e4ddbe64736f6c63430008140033",
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

// GetFunctionId is a free data retrieval call binding the contract method 0x9538f56f.
//
// Solidity: function getFunctionId(address _owner, string _name) pure returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewayCaller) GetFunctionId(opts *bind.CallOpts, _owner common.Address, _name string) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "getFunctionId", _owner, _name)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetFunctionId is a free data retrieval call binding the contract method 0x9538f56f.
//
// Solidity: function getFunctionId(address _owner, string _name) pure returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewaySession) GetFunctionId(_owner common.Address, _name string) ([32]byte, error) {
	return _FunctionGateway.Contract.GetFunctionId(&_FunctionGateway.CallOpts, _owner, _name)
}

// GetFunctionId is a free data retrieval call binding the contract method 0x9538f56f.
//
// Solidity: function getFunctionId(address _owner, string _name) pure returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewayCallerSession) GetFunctionId(_owner common.Address, _name string) ([32]byte, error) {
	return _FunctionGateway.Contract.GetFunctionId(&_FunctionGateway.CallOpts, _owner, _name)
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

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x5c74ad56.
//
// Solidity: function deployAndRegisterFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionGateway *FunctionGatewayTransactor) DeployAndRegisterFunction(opts *bind.TransactOpts, _bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "deployAndRegisterFunction", _bytecode, _name)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x5c74ad56.
//
// Solidity: function deployAndRegisterFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionGateway *FunctionGatewaySession) DeployAndRegisterFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.DeployAndRegisterFunction(&_FunctionGateway.TransactOpts, _bytecode, _name)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x5c74ad56.
//
// Solidity: function deployAndRegisterFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionGateway *FunctionGatewayTransactorSession) DeployAndRegisterFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.DeployAndRegisterFunction(&_FunctionGateway.TransactOpts, _bytecode, _name)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xb63755e5.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionGateway *FunctionGatewayTransactor) DeployAndUpdateFunction(opts *bind.TransactOpts, _bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "deployAndUpdateFunction", _bytecode, _name)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xb63755e5.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionGateway *FunctionGatewaySession) DeployAndUpdateFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.DeployAndUpdateFunction(&_FunctionGateway.TransactOpts, _bytecode, _name)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xb63755e5.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionGateway *FunctionGatewayTransactorSession) DeployAndUpdateFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.DeployAndUpdateFunction(&_FunctionGateway.TransactOpts, _bytecode, _name)
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

// RegisterFunction is a paid mutator transaction binding the contract method 0x68ff41b1.
//
// Solidity: function registerFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewayTransactor) RegisterFunction(opts *bind.TransactOpts, _verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "registerFunction", _verifier, _name)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x68ff41b1.
//
// Solidity: function registerFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewaySession) RegisterFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RegisterFunction(&_FunctionGateway.TransactOpts, _verifier, _name)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x68ff41b1.
//
// Solidity: function registerFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewayTransactorSession) RegisterFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RegisterFunction(&_FunctionGateway.TransactOpts, _verifier, _name)
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

// UpdateFunction is a paid mutator transaction binding the contract method 0xbd58c4bb.
//
// Solidity: function updateFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewayTransactor) UpdateFunction(opts *bind.TransactOpts, _verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "updateFunction", _verifier, _name)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xbd58c4bb.
//
// Solidity: function updateFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewaySession) UpdateFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateFunction(&_FunctionGateway.TransactOpts, _verifier, _name)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xbd58c4bb.
//
// Solidity: function updateFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionGateway *FunctionGatewayTransactorSession) UpdateFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpdateFunction(&_FunctionGateway.TransactOpts, _verifier, _name)
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

// FunctionGatewayDeployedIterator is returned from FilterDeployed and is used to iterate over the raw logs and unpacked data for Deployed events raised by the FunctionGateway contract.
type FunctionGatewayDeployedIterator struct {
	Event *FunctionGatewayDeployed // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayDeployedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayDeployed)
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
		it.Event = new(FunctionGatewayDeployed)
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
func (it *FunctionGatewayDeployedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayDeployedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayDeployed represents a Deployed event raised by the FunctionGateway contract.
type FunctionGatewayDeployed struct {
	BytecodeHash    [32]byte
	Salt            [32]byte
	DeployedAddress common.Address
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterDeployed is a free log retrieval operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_FunctionGateway *FunctionGatewayFilterer) FilterDeployed(opts *bind.FilterOpts, bytecodeHash [][32]byte, salt [][32]byte, deployedAddress []common.Address) (*FunctionGatewayDeployedIterator, error) {

	var bytecodeHashRule []interface{}
	for _, bytecodeHashItem := range bytecodeHash {
		bytecodeHashRule = append(bytecodeHashRule, bytecodeHashItem)
	}
	var saltRule []interface{}
	for _, saltItem := range salt {
		saltRule = append(saltRule, saltItem)
	}
	var deployedAddressRule []interface{}
	for _, deployedAddressItem := range deployedAddress {
		deployedAddressRule = append(deployedAddressRule, deployedAddressItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "Deployed", bytecodeHashRule, saltRule, deployedAddressRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayDeployedIterator{contract: _FunctionGateway.contract, event: "Deployed", logs: logs, sub: sub}, nil
}

// WatchDeployed is a free log subscription operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_FunctionGateway *FunctionGatewayFilterer) WatchDeployed(opts *bind.WatchOpts, sink chan<- *FunctionGatewayDeployed, bytecodeHash [][32]byte, salt [][32]byte, deployedAddress []common.Address) (event.Subscription, error) {

	var bytecodeHashRule []interface{}
	for _, bytecodeHashItem := range bytecodeHash {
		bytecodeHashRule = append(bytecodeHashRule, bytecodeHashItem)
	}
	var saltRule []interface{}
	for _, saltItem := range salt {
		saltRule = append(saltRule, saltItem)
	}
	var deployedAddressRule []interface{}
	for _, deployedAddressItem := range deployedAddress {
		deployedAddressRule = append(deployedAddressRule, deployedAddressItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "Deployed", bytecodeHashRule, saltRule, deployedAddressRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayDeployed)
				if err := _FunctionGateway.contract.UnpackLog(event, "Deployed", log); err != nil {
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

// ParseDeployed is a log parse operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_FunctionGateway *FunctionGatewayFilterer) ParseDeployed(log types.Log) (*FunctionGatewayDeployed, error) {
	event := new(FunctionGatewayDeployed)
	if err := _FunctionGateway.contract.UnpackLog(event, "Deployed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayFunctionOwnerUpdatedIterator is returned from FilterFunctionOwnerUpdated and is used to iterate over the raw logs and unpacked data for FunctionOwnerUpdated events raised by the FunctionGateway contract.
type FunctionGatewayFunctionOwnerUpdatedIterator struct {
	Event *FunctionGatewayFunctionOwnerUpdated // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayFunctionOwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayFunctionOwnerUpdated)
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
		it.Event = new(FunctionGatewayFunctionOwnerUpdated)
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
func (it *FunctionGatewayFunctionOwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayFunctionOwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayFunctionOwnerUpdated represents a FunctionOwnerUpdated event raised by the FunctionGateway contract.
type FunctionGatewayFunctionOwnerUpdated struct {
	FunctionId [32]byte
	Owner      common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionOwnerUpdated is a free log retrieval operation binding the contract event 0x376b0a13fca0286b5c7c73288ea980eb9d131fc8b996f7a46a49e0f90269aadf.
//
// Solidity: event FunctionOwnerUpdated(bytes32 indexed functionId, address owner)
func (_FunctionGateway *FunctionGatewayFilterer) FilterFunctionOwnerUpdated(opts *bind.FilterOpts, functionId [][32]byte) (*FunctionGatewayFunctionOwnerUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "FunctionOwnerUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayFunctionOwnerUpdatedIterator{contract: _FunctionGateway.contract, event: "FunctionOwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchFunctionOwnerUpdated is a free log subscription operation binding the contract event 0x376b0a13fca0286b5c7c73288ea980eb9d131fc8b996f7a46a49e0f90269aadf.
//
// Solidity: event FunctionOwnerUpdated(bytes32 indexed functionId, address owner)
func (_FunctionGateway *FunctionGatewayFilterer) WatchFunctionOwnerUpdated(opts *bind.WatchOpts, sink chan<- *FunctionGatewayFunctionOwnerUpdated, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "FunctionOwnerUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayFunctionOwnerUpdated)
				if err := _FunctionGateway.contract.UnpackLog(event, "FunctionOwnerUpdated", log); err != nil {
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

// ParseFunctionOwnerUpdated is a log parse operation binding the contract event 0x376b0a13fca0286b5c7c73288ea980eb9d131fc8b996f7a46a49e0f90269aadf.
//
// Solidity: event FunctionOwnerUpdated(bytes32 indexed functionId, address owner)
func (_FunctionGateway *FunctionGatewayFilterer) ParseFunctionOwnerUpdated(log types.Log) (*FunctionGatewayFunctionOwnerUpdated, error) {
	event := new(FunctionGatewayFunctionOwnerUpdated)
	if err := _FunctionGateway.contract.UnpackLog(event, "FunctionOwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayFunctionRegisteredIterator is returned from FilterFunctionRegistered and is used to iterate over the raw logs and unpacked data for FunctionRegistered events raised by the FunctionGateway contract.
type FunctionGatewayFunctionRegisteredIterator struct {
	Event *FunctionGatewayFunctionRegistered // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayFunctionRegisteredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayFunctionRegistered)
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
		it.Event = new(FunctionGatewayFunctionRegistered)
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
func (it *FunctionGatewayFunctionRegisteredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayFunctionRegisteredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayFunctionRegistered represents a FunctionRegistered event raised by the FunctionGateway contract.
type FunctionGatewayFunctionRegistered struct {
	FunctionId [32]byte
	Verifier   common.Address
	Name       string
	Owner      common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionRegistered is a free log retrieval operation binding the contract event 0x52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, string name, address owner)
func (_FunctionGateway *FunctionGatewayFilterer) FilterFunctionRegistered(opts *bind.FilterOpts, functionId [][32]byte) (*FunctionGatewayFunctionRegisteredIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "FunctionRegistered", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayFunctionRegisteredIterator{contract: _FunctionGateway.contract, event: "FunctionRegistered", logs: logs, sub: sub}, nil
}

// WatchFunctionRegistered is a free log subscription operation binding the contract event 0x52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, string name, address owner)
func (_FunctionGateway *FunctionGatewayFilterer) WatchFunctionRegistered(opts *bind.WatchOpts, sink chan<- *FunctionGatewayFunctionRegistered, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "FunctionRegistered", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayFunctionRegistered)
				if err := _FunctionGateway.contract.UnpackLog(event, "FunctionRegistered", log); err != nil {
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

// ParseFunctionRegistered is a log parse operation binding the contract event 0x52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, string name, address owner)
func (_FunctionGateway *FunctionGatewayFilterer) ParseFunctionRegistered(log types.Log) (*FunctionGatewayFunctionRegistered, error) {
	event := new(FunctionGatewayFunctionRegistered)
	if err := _FunctionGateway.contract.UnpackLog(event, "FunctionRegistered", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayFunctionVerifierUpdatedIterator is returned from FilterFunctionVerifierUpdated and is used to iterate over the raw logs and unpacked data for FunctionVerifierUpdated events raised by the FunctionGateway contract.
type FunctionGatewayFunctionVerifierUpdatedIterator struct {
	Event *FunctionGatewayFunctionVerifierUpdated // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayFunctionVerifierUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayFunctionVerifierUpdated)
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
		it.Event = new(FunctionGatewayFunctionVerifierUpdated)
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
func (it *FunctionGatewayFunctionVerifierUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayFunctionVerifierUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayFunctionVerifierUpdated represents a FunctionVerifierUpdated event raised by the FunctionGateway contract.
type FunctionGatewayFunctionVerifierUpdated struct {
	FunctionId [32]byte
	Verifier   common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionVerifierUpdated is a free log retrieval operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_FunctionGateway *FunctionGatewayFilterer) FilterFunctionVerifierUpdated(opts *bind.FilterOpts, functionId [][32]byte) (*FunctionGatewayFunctionVerifierUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "FunctionVerifierUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayFunctionVerifierUpdatedIterator{contract: _FunctionGateway.contract, event: "FunctionVerifierUpdated", logs: logs, sub: sub}, nil
}

// WatchFunctionVerifierUpdated is a free log subscription operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_FunctionGateway *FunctionGatewayFilterer) WatchFunctionVerifierUpdated(opts *bind.WatchOpts, sink chan<- *FunctionGatewayFunctionVerifierUpdated, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "FunctionVerifierUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayFunctionVerifierUpdated)
				if err := _FunctionGateway.contract.UnpackLog(event, "FunctionVerifierUpdated", log); err != nil {
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

// ParseFunctionVerifierUpdated is a log parse operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_FunctionGateway *FunctionGatewayFilterer) ParseFunctionVerifierUpdated(log types.Log) (*FunctionGatewayFunctionVerifierUpdated, error) {
	event := new(FunctionGatewayFunctionVerifierUpdated)
	if err := _FunctionGateway.contract.UnpackLog(event, "FunctionVerifierUpdated", log); err != nil {
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
	Nonce      *big.Int
	FunctionId [32]byte
	RequestId  [32]byte
	Inputs     []byte
	Context    []byte
	GasLimit   *big.Int
	FeeAmount  *big.Int
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterProofRequested is a free log retrieval operation binding the contract event 0x3fb5c9bd4c90dcd3781879795c37f8645d9421602f4ba57c651f3005938c7260.
//
// Solidity: event ProofRequested(uint256 indexed nonce, bytes32 indexed functionId, bytes32 requestId, bytes inputs, bytes context, uint256 gasLimit, uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayFilterer) FilterProofRequested(opts *bind.FilterOpts, nonce []*big.Int, functionId [][32]byte) (*FunctionGatewayProofRequestedIterator, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "ProofRequested", nonceRule, functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayProofRequestedIterator{contract: _FunctionGateway.contract, event: "ProofRequested", logs: logs, sub: sub}, nil
}

// WatchProofRequested is a free log subscription operation binding the contract event 0x3fb5c9bd4c90dcd3781879795c37f8645d9421602f4ba57c651f3005938c7260.
//
// Solidity: event ProofRequested(uint256 indexed nonce, bytes32 indexed functionId, bytes32 requestId, bytes inputs, bytes context, uint256 gasLimit, uint256 feeAmount)
func (_FunctionGateway *FunctionGatewayFilterer) WatchProofRequested(opts *bind.WatchOpts, sink chan<- *FunctionGatewayProofRequested, nonce []*big.Int, functionId [][32]byte) (event.Subscription, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "ProofRequested", nonceRule, functionIdRule)
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

// ParseProofRequested is a log parse operation binding the contract event 0x3fb5c9bd4c90dcd3781879795c37f8645d9421602f4ba57c651f3005938c7260.
//
// Solidity: event ProofRequested(uint256 indexed nonce, bytes32 indexed functionId, bytes32 requestId, bytes inputs, bytes context, uint256 gasLimit, uint256 feeAmount)
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

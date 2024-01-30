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

// SuccinctGatewayMetaData contains all meta data concerning the SuccinctGateway contract.
var SuccinctGatewayMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"function\",\"name\":\"addCustomProver\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_prover\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"addDefaultProver\",\"inputs\":[{\"name\":\"_prover\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"allowedProvers\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"deployAndRegisterFunction\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_bytecode\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"verifier\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deployAndUpdateFunction\",\"inputs\":[{\"name\":\"_bytecode\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"verifier\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"feeVault\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"fulfillCall\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_input\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_output\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_proof\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_callbackAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_callbackData\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"fulfillCallback\",\"inputs\":[{\"name\":\"_nonce\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_inputHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_callbackAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_callbackSelector\",\"type\":\"bytes4\",\"internalType\":\"bytes4\"},{\"name\":\"_callbackGasLimit\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"_context\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_output\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_proof\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getFunctionId\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_feeVault\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_defaultProver\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"isCallback\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nonce\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"recover\",\"inputs\":[{\"name\":\"_to\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"registerFunction\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_verifier\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"removeCustomProver\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_prover\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"removeDefaultProver\",\"inputs\":[{\"name\":\"_prover\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"requestCall\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_input\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_entryAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_entryCalldata\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_entryGasLimit\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"requestCallback\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_input\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_context\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"_callbackSelector\",\"type\":\"bytes4\",\"internalType\":\"bytes4\"},{\"name\":\"_callbackGasLimit\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"requests\",\"inputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setFeeVault\",\"inputs\":[{\"name\":\"_feeVault\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setWhitelistStatus\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_status\",\"type\":\"uint8\",\"internalType\":\"enumWhitelistStatus\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateFunction\",\"inputs\":[{\"name\":\"_verifier\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifiedCall\",\"inputs\":[{\"name\":\"_functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_input\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifiedFunctionId\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifiedInputHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifiedOutput\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifierOwners\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"verifiers\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"whitelistStatus\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"enumWhitelistStatus\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"Call\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"inputHash\",\"type\":\"bytes32\",\"indexed\":false,\"internalType\":\"bytes32\"},{\"name\":\"outputHash\",\"type\":\"bytes32\",\"indexed\":false,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Deployed\",\"inputs\":[{\"name\":\"bytecodeHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"deployedAddress\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"FunctionRegistered\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"verifier\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"indexed\":false,\"internalType\":\"bytes32\"},{\"name\":\"owner\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"FunctionVerifierUpdated\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"verifier\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ProverUpdated\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"prover\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"added\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RequestCall\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"input\",\"type\":\"bytes\",\"indexed\":false,\"internalType\":\"bytes\"},{\"name\":\"entryAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"entryCalldata\",\"type\":\"bytes\",\"indexed\":false,\"internalType\":\"bytes\"},{\"name\":\"entryGasLimit\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"sender\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"feeAmount\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RequestCallback\",\"inputs\":[{\"name\":\"nonce\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"input\",\"type\":\"bytes\",\"indexed\":false,\"internalType\":\"bytes\"},{\"name\":\"context\",\"type\":\"bytes\",\"indexed\":false,\"internalType\":\"bytes\"},{\"name\":\"callbackAddress\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"callbackSelector\",\"type\":\"bytes4\",\"indexed\":false,\"internalType\":\"bytes4\"},{\"name\":\"callbackGasLimit\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"feeAmount\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RequestFulfilled\",\"inputs\":[{\"name\":\"nonce\",\"type\":\"uint32\",\"indexed\":true,\"internalType\":\"uint32\"},{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"inputHash\",\"type\":\"bytes32\",\"indexed\":false,\"internalType\":\"bytes32\"},{\"name\":\"outputHash\",\"type\":\"bytes32\",\"indexed\":false,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"SetFeeVault\",\"inputs\":[{\"name\":\"oldFeeVault\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newFeeVault\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"WhitelistStatusUpdated\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"status\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"enumWhitelistStatus\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"CallFailed\",\"inputs\":[{\"name\":\"callbackAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"callbackData\",\"type\":\"bytes\",\"internalType\":\"bytes\"}]},{\"type\":\"error\",\"name\":\"CallbackFailed\",\"inputs\":[{\"name\":\"callbackSelector\",\"type\":\"bytes4\",\"internalType\":\"bytes4\"},{\"name\":\"output\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"context\",\"type\":\"bytes\",\"internalType\":\"bytes\"}]},{\"type\":\"error\",\"name\":\"EmptyBytecode\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FailedDeploy\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FunctionAlreadyRegistered\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"InvalidCall\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"input\",\"type\":\"bytes\",\"internalType\":\"bytes\"}]},{\"type\":\"error\",\"name\":\"InvalidProof\",\"inputs\":[{\"name\":\"verifier\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"inputHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"outputHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"proof\",\"type\":\"bytes\",\"internalType\":\"bytes\"}]},{\"type\":\"error\",\"name\":\"InvalidRequest\",\"inputs\":[{\"name\":\"nonce\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"expectedRequestHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"requestHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"NotFunctionOwner\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"actualOwner\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"type\":\"error\",\"name\":\"OnlyProver\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"type\":\"error\",\"name\":\"RecoverFailed\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"ReentrantFulfill\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"VerifierAlreadyUpdated\",\"inputs\":[{\"name\":\"functionId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"type\":\"error\",\"name\":\"VerifierCannotBeZero\",\"inputs\":[]}]",
	Bin: "0x608060405234801561001057600080fd5b50612763806100206000396000f3fe6080604052600436106101e35760003560e01c80638157ce2b11610102578063c0c53b8b11610095578063ed4ec5e111610064578063ed4ec5e1146105dc578063efe1c95014610617578063f2fde38b1461064d578063ff0b2f8a1461066d57600080fd5b8063c0c53b8b1461055c578063ca6b63c71461057c578063d0bf5dab1461059c578063dd469fa4146105bc57600080fd5b8063affed0e0116100d1578063affed0e0146104d0578063b3f04fdf14610509578063b97ed3ca1461051c578063bac2a1061461053c57600080fd5b80638157ce2b146104465780638bcfc3a01461045c5780638da5cb5b14610492578063a591f97f146104b057600080fd5b8063478222c21161017a578063715018a611610149578063715018a6146103db5780637413555d146103f057806378370ebd1461041057806380e0bbb01461043057600080fd5b8063478222c214610326578063493e9b811461035e5780635705ae431461039b5780635e5da3b6146103bb57600080fd5b8063176e62fd116101b6578063176e62fd146102965780633a446f68146102b6578063420d334c146102f3578063436a61d51461031357600080fd5b806305d7c1cf146101e85780630ab469b014610217578063164d420714610252578063173869f014610274575b600080fd5b3480156101f457600080fd5b50606c546102029060ff1681565b60405190151581526020015b60405180910390f35b34801561022357600080fd5b50610244610232366004611cf7565b60686020526000908152604090205481565b60405190815260200161020e565b34801561025e57600080fd5b5061027261026d366004611d30565b61068d565b005b34801561028057600080fd5b5061028961073d565b60405161020e9190611dac565b3480156102a257600080fd5b506102896102b1366004611e62565b6107cb565b3480156102c257600080fd5b506102d66102d1366004611ea9565b6108ee565b604080519283526001600160a01b0390911660208301520161020e565b3480156102ff57600080fd5b5061027261030e366004611f00565b610969565b610272610321366004611f34565b610a2e565b34801561033257600080fd5b50606754610346906001600160a01b031681565b6040516001600160a01b03909116815260200161020e565b34801561036a57600080fd5b5061038e610379366004611fc3565b606d6020526000908152604090205460ff1681565b60405161020e9190611ff2565b3480156103a757600080fd5b506102726103b636600461201a565b610ae8565b3480156103c757600080fd5b506102446103d6366004612044565b610b69565b3480156103e757600080fd5b50610272610bd6565b3480156103fc57600080fd5b5061027261040b366004612080565b610bea565b34801561041c57600080fd5b5061027261042b3660046120b3565b610c43565b34801561043c57600080fd5b50610244606a5481565b34801561045257600080fd5b5061024460695481565b34801561046857600080fd5b50610346610477366004611fc3565b6001602052600090815260409020546001600160a01b031681565b34801561049e57600080fd5b506035546001600160a01b0316610346565b3480156104bc57600080fd5b506102726104cb366004612080565b610fb2565b3480156104dc57600080fd5b506067546104f490600160a01b900463ffffffff1681565b60405163ffffffff909116815260200161020e565b610244610517366004612192565b611016565b34801561052857600080fd5b50610272610537366004611d30565b6111cc565b34801561054857600080fd5b50610272610557366004612215565b611275565b34801561056857600080fd5b506102726105773660046122db565b611560565b34801561058857600080fd5b5061024461059736600461201a565b6116bd565b3480156105a857600080fd5b506102446105b736600461201a565b611719565b3480156105c857600080fd5b506102726105d7366004612080565b611756565b3480156105e857600080fd5b506102026105f7366004611d30565b606e60209081526000928352604080842090915290825290205460ff1681565b34801561062357600080fd5b50610346610632366004611fc3565b6000602081905290815260409020546001600160a01b031681565b34801561065957600080fd5b50610272610668366004612080565b6117af565b34801561067957600080fd5b506102d661068836600461231e565b611828565b6000828152600160205260409020546001600160a01b031633146106eb5760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b0390911660248201526044015b60405180910390fd5b6000828152606e602090815260408083206001600160a01b038516808552908352818420805460ff1916905590519283529184916000805160206126ee83398151915291015b60405180910390a35050565b606b805461074a90612363565b80601f016020809104026020016040519081016040528092919081815260200182805461077690612363565b80156107c35780601f10610798576101008083540402835291602001916107c3565b820191906000526020600020905b8154815290600101906020018083116107a657829003601f168201915b505050505081565b606060006002836040516107df919061239d565b602060405180830381855afa1580156107fc573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061081f91906123b9565b905083606954148015610833575080606a54145b156108cb57606b805461084590612363565b80601f016020809104026020016040519081016040528092919081815260200182805461087190612363565b80156108be5780601f10610893576101008083540402835291602001916108be565b820191906000526020600020905b8154815290600101906020018083116108a157829003601f168201915b50505050509150506108e8565b838360405163aa74a2cb60e01b81526004016106e29291906123d2565b92915050565b6000806108fb8584611719565b91506109078483611892565b905061091482868361192c565b604080516001600160a01b03838116825260208201869052871681830152905183917fdfa40ee17618caabca7b46eb80031e3d05a27a30de957d29b4a2a71426db5a63919081900360600190a2935093915050565b6000828152600160205260409020546001600160a01b031633146109c25760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b0390911660248201526044016106e2565b6000828152606d60205260409020805482919060ff191660018360028111156109ed576109ed611fdc565b0217905550817f1cfd8c214b5ba0c1306317bd132fa41ca741da227af86a75e9010708561be80e82604051610a229190611ff2565b60405180910390a25050565b847f88632d59d3df3bee2ce2a06fbb05e1c8542c44cb2e7339bb1812500a978644d3858585853334604051610a68969594939291906123f3565b60405180910390a26067546001600160a01b031615610ae1576067546040516333bb7f9160e01b81523360048201526001600160a01b03909116906333bb7f919034906024016000604051808303818588803b158015610ac757600080fd5b505af1158015610adb573d6000803e3d6000fd5b50505050505b5050505050565b610af06119cd565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114610b3d576040519150601f19603f3d011682016040523d82523d6000602084013e610b42565b606091505b5050905080610b6457604051630e9cdfa160e31b815260040160405180910390fd5b505050565b6000610b758483611719565b9050610b8281858561192c565b604080516001600160a01b03858116825260208201859052861681830152905182917fdfa40ee17618caabca7b46eb80031e3d05a27a30de957d29b4a2a71426db5a63919081900360600190a29392505050565b610bde6119cd565b610be86000611a27565b565b610bf26119cd565b6001600160a01b038116600081815260008051602061270e83398151915260209081526040808320805460ff19169055518281526000805160206126ee83398151915291015b60405180910390a350565b606c5460ff1680610c55575060695415155b80610c615750606a5415155b80610c795750606b8054610c7490612363565b151590505b15610c97576040516360d8fdfb60e11b815260040160405180910390fd5b87600080828152606d602052604090205460ff166002811115610cbc57610cbc611fdc565b148015610ce6575033600090815260008051602061270e833981519152602052604090205460ff16155b15610d0d57604051632e194a0560e21b8152600481018290523360248201526044016106e2565b60016000828152606d602052604090205460ff166002811115610d3257610d32611fdc565b148015610d5957506000818152606e6020908152604080832033845290915290205460ff16155b15610d8057604051632e194a0560e21b8152600481018290523360248201526044016106e2565b835160208501206000610d988c8c8c858d8d8d611a79565b63ffffffff8d166000908152606860205260409020549091508114610df75763ffffffff8c16600081815260686020526040908190205490516310fc041760e31b815260048101929092526024820152604481018290526064016106e2565b63ffffffff8c1660009081526068602052604080822082905551600290610e1f90889061239d565b602060405180830381855afa158015610e3c573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190610e5f91906123b9565b9050610e6d8c8c8388611af9565b606c805460ff191660011790556040516000906001600160a01b038c169063ffffffff8b16908c90610ea5908b908d9060240161244c565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610ee3919061239d565b60006040518083038160008787f1925050503d8060008114610f21576040519150601f19603f3d011682016040523d82523d6000602084013e610f26565b606091505b5050606c805460ff19169055905080610f58578987896040516315c9414b60e11b81526004016106e29392919061247a565b8c8e63ffffffff167f361a2fc76bc9f35b079dd353fd7fdd8aaf61f1a7979cf59653225692c19bbff28e85604051610f9a929190918252602082015260400190565b60405180910390a35050505050505050505050505050565b610fba6119cd565b6067546040516001600160a01b038084169216907ff0cca8e172b90b70922c6757d918f7a532326dfd3e9f3c5b117a616d2bb0721290600090a3606780546001600160a01b0319166001600160a01b0392909216919091179055565b600080600286604051611029919061239d565b602060405180830381855afa158015611046573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061106991906123b9565b8551602087012060675491925090339060009061109890600160a01b900463ffffffff168b8686868c8c611a79565b6067805463ffffffff600160a01b91829004811660009081526068602052604090819020859055925492519394508d9391909204909116907f22a09d598b323a3c65d69787dd6fd143dd8e4d2f91733c247113167df31e3e9390611107908d908d9088908e908e9034906124b8565b60405180910390a360678054600160a01b900463ffffffff1690601461112c8361251c565b825463ffffffff9182166101009390930a9283029190920219909116179055506067546001600160a01b0316156111bf576067546040516333bb7f9160e01b81526001600160a01b038481166004830152909116906333bb7f919034906024016000604051808303818588803b1580156111a557600080fd5b505af11580156111b9573d6000803e3d6000fd5b50505050505b9998505050505050505050565b6000828152600160205260409020546001600160a01b031633146112255760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b0390911660248201526044016106e2565b6000828152606e602090815260408083206001600160a01b03851680855290835292819020805460ff19166001908117909155905190815284916000805160206126ee8339815191529101610731565b606c5460ff1680611287575060695415155b806112935750606a5415155b806112ab5750606b80546112a690612363565b151590505b156112c9576040516360d8fdfb60e11b815260040160405180910390fd5b85600080828152606d602052604090205460ff1660028111156112ee576112ee611fdc565b148015611318575033600090815260008051602061270e833981519152602052604090205460ff16155b1561133f57604051632e194a0560e21b8152600481018290523360248201526044016106e2565b60016000828152606d602052604090205460ff16600281111561136457611364611fdc565b14801561138b57506000818152606e6020908152604080832033845290915290205460ff16155b156113b257604051632e194a0560e21b8152600481018290523360248201526044016106e2565b60006002876040516113c4919061239d565b602060405180830381855afa1580156113e1573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061140491906123b9565b90506000600287604051611418919061239d565b602060405180830381855afa158015611435573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061145891906123b9565b905061146689838389611af9565b6069899055606a829055606b61147c888261259b565b506000856001600160a01b031685604051611497919061239d565b6000604051808303816000865af19150503d80600081146114d4576040519150601f19603f3d011682016040523d82523d6000602084013e6114d9565b606091505b50509050806114ff578585604051636c544f3360e01b81526004016106e292919061265b565b606960009055606a60009055606b60006115199190611c90565b60408051848152602081018490528b917f41d7122d18af9f0c92f23bcea9d5fa416cadcd1ed2fc8e544a3c89b841ecfd15910160405180910390a250505050505050505050565b600254610100900460ff16158080156115805750600254600160ff909116105b8061159a5750303b15801561159a575060025460ff166001145b6115fd5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016106e2565b6002805460ff191660011790558015611620576002805461ff0019166101001790555b61162984611a27565b606780546001600160a01b0319166001600160a01b03858116919091179091558216600090815260008051602061270e83398151915260205260409020805460ff1916600117905580156116b7576002805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60006116c93383611719565b90506116d58184611ba4565b6040516001600160a01b038416815281907ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369060200160405180910390a292915050565b604080516001600160a01b038416602082015290810182905260009060600160405160208183030381529060405280519060200120905092915050565b61175e6119cd565b6001600160a01b038116600081815260008051602061270e83398151915260209081526040808320805460ff1916600190811790915590519081526000805160206126ee8339815191529101610c38565b6117b76119cd565b6001600160a01b03811661181c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016106e2565b61182581611a27565b50565b6000806118353384611719565b91506118418483611892565b905061184d8282611ba4565b6040516001600160a01b038216815282907ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369060200160405180910390a29250929050565b600082516000036118b6576040516321744a5960e01b815260040160405180910390fd5b818351602085016000f590506001600160a01b0381166118e957604051632081741d60e11b815260040160405180910390fd5b825160208401206040516001600160a01b0383169184917f27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f53290600090a492915050565b6001600160a01b038116611953576040516302d48d1f60e61b815260040160405180910390fd5b6000838152602081905260409020546001600160a01b03161561198c57604051635e34c78f60e01b8152600481018490526024016106e2565b600092835260016020908152604080852080546001600160a01b03199081166001600160a01b03968716179091559185905290932080549093169116179055565b6035546001600160a01b03163314610be85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016106e2565b603580546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6040516001600160e01b031960e089811b821660208401526024830189905260448301889052606483018790526bffffffffffffffffffffffff19606087901b166084840152818516609884015283901b16609c82015260009060a001604051602081830303815290604052805190602001209050979650505050505050565b600084815260208190526040908190205490516303784b1960e61b81526001600160a01b0390911690819063de12c64090611b3c9087908790879060040161267f565b6020604051808303816000875af1158015611b5b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b7f919061269e565b610ae157808484846040516316c7141360e31b81526004016106e294939291906126c0565b6001600160a01b038116611bcb576040516302d48d1f60e61b815260040160405180910390fd5b6000828152600160205260409020546001600160a01b03163314611c245760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b0390911660248201526044016106e2565b6000828152602081905260409020546001600160a01b0390811690821603611c62576040516321d0cb4d60e01b8152600481018390526024016106e2565b60009182526020829052604090912080546001600160a01b0319166001600160a01b03909216919091179055565b508054611c9c90612363565b6000825580601f10611cac575050565b601f01602090049060005260206000209081019061182591905b80821115611cda5760008155600101611cc6565b5090565b803563ffffffff81168114611cf257600080fd5b919050565b600060208284031215611d0957600080fd5b611d1282611cde565b9392505050565b80356001600160a01b0381168114611cf257600080fd5b60008060408385031215611d4357600080fd5b82359150611d5360208401611d19565b90509250929050565b60005b83811015611d77578181015183820152602001611d5f565b50506000910152565b60008151808452611d98816020860160208601611d5c565b601f01601f19169290920160200192915050565b602081526000611d126020830184611d80565b634e487b7160e01b600052604160045260246000fd5b600082601f830112611de657600080fd5b813567ffffffffffffffff80821115611e0157611e01611dbf565b604051601f8301601f19908116603f01168101908282118183101715611e2957611e29611dbf565b81604052838152866020858801011115611e4257600080fd5b836020870160208301376000602085830101528094505050505092915050565b60008060408385031215611e7557600080fd5b82359150602083013567ffffffffffffffff811115611e9357600080fd5b611e9f85828601611dd5565b9150509250929050565b600080600060608486031215611ebe57600080fd5b611ec784611d19565b9250602084013567ffffffffffffffff811115611ee357600080fd5b611eef86828701611dd5565b925050604084013590509250925092565b60008060408385031215611f1357600080fd5b82359150602083013560038110611f2957600080fd5b809150509250929050565b600080600080600060a08688031215611f4c57600080fd5b85359450602086013567ffffffffffffffff80821115611f6b57600080fd5b611f7789838a01611dd5565b9550611f8560408901611d19565b94506060880135915080821115611f9b57600080fd5b50611fa888828901611dd5565b925050611fb760808701611cde565b90509295509295909350565b600060208284031215611fd557600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b602081016003831061201457634e487b7160e01b600052602160045260246000fd5b91905290565b6000806040838503121561202d57600080fd5b61203683611d19565b946020939093013593505050565b60008060006060848603121561205957600080fd5b61206284611d19565b925061207060208501611d19565b9150604084013590509250925092565b60006020828403121561209257600080fd5b611d1282611d19565b80356001600160e01b031981168114611cf257600080fd5b60008060008060008060008060006101208a8c0312156120d257600080fd5b6120db8a611cde565b985060208a0135975060408a013596506120f760608b01611d19565b955061210560808b0161209b565b945061211360a08b01611cde565b935060c08a013567ffffffffffffffff8082111561213057600080fd5b61213c8d838e01611dd5565b945060e08c013591508082111561215257600080fd5b61215e8d838e01611dd5565b93506101008c013591508082111561217557600080fd5b506121828c828d01611dd5565b9150509295985092959850929598565b600080600080600060a086880312156121aa57600080fd5b85359450602086013567ffffffffffffffff808211156121c957600080fd5b6121d589838a01611dd5565b955060408801359150808211156121eb57600080fd5b506121f888828901611dd5565b9350506122076060870161209b565b9150611fb760808701611cde565b60008060008060008060c0878903121561222e57600080fd5b86359550602087013567ffffffffffffffff8082111561224d57600080fd5b6122598a838b01611dd5565b9650604089013591508082111561226f57600080fd5b61227b8a838b01611dd5565b9550606089013591508082111561229157600080fd5b61229d8a838b01611dd5565b94506122ab60808a01611d19565b935060a08901359150808211156122c157600080fd5b506122ce89828a01611dd5565b9150509295509295509295565b6000806000606084860312156122f057600080fd5b6122f984611d19565b925061230760208501611d19565b915061231560408501611d19565b90509250925092565b6000806040838503121561233157600080fd5b823567ffffffffffffffff81111561234857600080fd5b61235485828601611dd5565b95602094909401359450505050565b600181811c9082168061237757607f821691505b60208210810361239757634e487b7160e01b600052602260045260246000fd5b50919050565b600082516123af818460208701611d5c565b9190910192915050565b6000602082840312156123cb57600080fd5b5051919050565b8281526040602082015260006123eb6040830184611d80565b949350505050565b60c08152600061240660c0830189611d80565b6001600160a01b03888116602085015283820360408501526124288289611d80565b63ffffffff9790971660608501529490941660808301525060a00152509392505050565b60408152600061245f6040830185611d80565b82810360208401526124718185611d80565b95945050505050565b63ffffffff60e01b8416815260606020820152600061249c6060830185611d80565b82810360408401526124ae8185611d80565b9695505050505050565b60c0815260006124cb60c0830189611d80565b82810360208401526124dd8189611d80565b6001600160a01b0397909716604084015250506001600160e01b031993909316606084015263ffffffff91909116608083015260a09091015292915050565b600063ffffffff80831681810361254357634e487b7160e01b600052601160045260246000fd5b6001019392505050565b601f821115610b6457600081815260208120601f850160051c810160208610156125745750805b601f850160051c820191505b8181101561259357828155600101612580565b505050505050565b815167ffffffffffffffff8111156125b5576125b5611dbf565b6125c9816125c38454612363565b8461254d565b602080601f8311600181146125fe57600084156125e65750858301515b600019600386901b1c1916600185901b178555612593565b600085815260208120601f198616915b8281101561262d5788860151825594840194600190910190840161260e565b508582101561264b5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b6001600160a01b03831681526040602082018190526000906123eb90830184611d80565b8381528260208201526060604082015260006124716060830184611d80565b6000602082840312156126b057600080fd5b81518015158114611d1257600080fd5b60018060a01b03851681528360208201528260408201526080606082015260006124ae6080830184611d8056fe1c19213e895a5cf2a6027ac01d21289130e55827ec6257a8bb946b4c524d408f136eb4aae73f7618d8559a84c5ff3678edc6b16994db052447ebc43c429b7d6fa2646970667358221220208b35669ccecfbea23363c540810ad67754f9346e367908f54964ea47c129a564736f6c63430008100033",
}

// SuccinctGatewayABI is the input ABI used to generate the binding from.
// Deprecated: Use SuccinctGatewayMetaData.ABI instead.
var SuccinctGatewayABI = SuccinctGatewayMetaData.ABI

// SuccinctGatewayBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use SuccinctGatewayMetaData.Bin instead.
var SuccinctGatewayBin = SuccinctGatewayMetaData.Bin

// DeploySuccinctGateway deploys a new Ethereum contract, binding an instance of SuccinctGateway to it.
func DeploySuccinctGateway(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *SuccinctGateway, error) {
	parsed, err := SuccinctGatewayMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(SuccinctGatewayBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &SuccinctGateway{SuccinctGatewayCaller: SuccinctGatewayCaller{contract: contract}, SuccinctGatewayTransactor: SuccinctGatewayTransactor{contract: contract}, SuccinctGatewayFilterer: SuccinctGatewayFilterer{contract: contract}}, nil
}

// SuccinctGateway is an auto generated Go binding around an Ethereum contract.
type SuccinctGateway struct {
	SuccinctGatewayCaller     // Read-only binding to the contract
	SuccinctGatewayTransactor // Write-only binding to the contract
	SuccinctGatewayFilterer   // Log filterer for contract events
}

// SuccinctGatewayCaller is an auto generated read-only Go binding around an Ethereum contract.
type SuccinctGatewayCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SuccinctGatewayTransactor is an auto generated write-only Go binding around an Ethereum contract.
type SuccinctGatewayTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SuccinctGatewayFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type SuccinctGatewayFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SuccinctGatewaySession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type SuccinctGatewaySession struct {
	Contract     *SuccinctGateway  // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// SuccinctGatewayCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type SuccinctGatewayCallerSession struct {
	Contract *SuccinctGatewayCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts          // Call options to use throughout this session
}

// SuccinctGatewayTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type SuccinctGatewayTransactorSession struct {
	Contract     *SuccinctGatewayTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts          // Transaction auth options to use throughout this session
}

// SuccinctGatewayRaw is an auto generated low-level Go binding around an Ethereum contract.
type SuccinctGatewayRaw struct {
	Contract *SuccinctGateway // Generic contract binding to access the raw methods on
}

// SuccinctGatewayCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type SuccinctGatewayCallerRaw struct {
	Contract *SuccinctGatewayCaller // Generic read-only contract binding to access the raw methods on
}

// SuccinctGatewayTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type SuccinctGatewayTransactorRaw struct {
	Contract *SuccinctGatewayTransactor // Generic write-only contract binding to access the raw methods on
}

// NewSuccinctGateway creates a new instance of SuccinctGateway, bound to a specific deployed contract.
func NewSuccinctGateway(address common.Address, backend bind.ContractBackend) (*SuccinctGateway, error) {
	contract, err := bindSuccinctGateway(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &SuccinctGateway{SuccinctGatewayCaller: SuccinctGatewayCaller{contract: contract}, SuccinctGatewayTransactor: SuccinctGatewayTransactor{contract: contract}, SuccinctGatewayFilterer: SuccinctGatewayFilterer{contract: contract}}, nil
}

// NewSuccinctGatewayCaller creates a new read-only instance of SuccinctGateway, bound to a specific deployed contract.
func NewSuccinctGatewayCaller(address common.Address, caller bind.ContractCaller) (*SuccinctGatewayCaller, error) {
	contract, err := bindSuccinctGateway(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayCaller{contract: contract}, nil
}

// NewSuccinctGatewayTransactor creates a new write-only instance of SuccinctGateway, bound to a specific deployed contract.
func NewSuccinctGatewayTransactor(address common.Address, transactor bind.ContractTransactor) (*SuccinctGatewayTransactor, error) {
	contract, err := bindSuccinctGateway(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayTransactor{contract: contract}, nil
}

// NewSuccinctGatewayFilterer creates a new log filterer instance of SuccinctGateway, bound to a specific deployed contract.
func NewSuccinctGatewayFilterer(address common.Address, filterer bind.ContractFilterer) (*SuccinctGatewayFilterer, error) {
	contract, err := bindSuccinctGateway(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayFilterer{contract: contract}, nil
}

// bindSuccinctGateway binds a generic wrapper to an already deployed contract.
func bindSuccinctGateway(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := SuccinctGatewayMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SuccinctGateway *SuccinctGatewayRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SuccinctGateway.Contract.SuccinctGatewayCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SuccinctGateway *SuccinctGatewayRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.SuccinctGatewayTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SuccinctGateway *SuccinctGatewayRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.SuccinctGatewayTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SuccinctGateway *SuccinctGatewayCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SuccinctGateway.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SuccinctGateway *SuccinctGatewayTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SuccinctGateway *SuccinctGatewayTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.contract.Transact(opts, method, params...)
}

// AllowedProvers is a free data retrieval call binding the contract method 0xed4ec5e1.
//
// Solidity: function allowedProvers(bytes32 , address ) view returns(bool)
func (_SuccinctGateway *SuccinctGatewayCaller) AllowedProvers(opts *bind.CallOpts, arg0 [32]byte, arg1 common.Address) (bool, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "allowedProvers", arg0, arg1)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// AllowedProvers is a free data retrieval call binding the contract method 0xed4ec5e1.
//
// Solidity: function allowedProvers(bytes32 , address ) view returns(bool)
func (_SuccinctGateway *SuccinctGatewaySession) AllowedProvers(arg0 [32]byte, arg1 common.Address) (bool, error) {
	return _SuccinctGateway.Contract.AllowedProvers(&_SuccinctGateway.CallOpts, arg0, arg1)
}

// AllowedProvers is a free data retrieval call binding the contract method 0xed4ec5e1.
//
// Solidity: function allowedProvers(bytes32 , address ) view returns(bool)
func (_SuccinctGateway *SuccinctGatewayCallerSession) AllowedProvers(arg0 [32]byte, arg1 common.Address) (bool, error) {
	return _SuccinctGateway.Contract.AllowedProvers(&_SuccinctGateway.CallOpts, arg0, arg1)
}

// FeeVault is a free data retrieval call binding the contract method 0x478222c2.
//
// Solidity: function feeVault() view returns(address)
func (_SuccinctGateway *SuccinctGatewayCaller) FeeVault(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "feeVault")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// FeeVault is a free data retrieval call binding the contract method 0x478222c2.
//
// Solidity: function feeVault() view returns(address)
func (_SuccinctGateway *SuccinctGatewaySession) FeeVault() (common.Address, error) {
	return _SuccinctGateway.Contract.FeeVault(&_SuccinctGateway.CallOpts)
}

// FeeVault is a free data retrieval call binding the contract method 0x478222c2.
//
// Solidity: function feeVault() view returns(address)
func (_SuccinctGateway *SuccinctGatewayCallerSession) FeeVault() (common.Address, error) {
	return _SuccinctGateway.Contract.FeeVault(&_SuccinctGateway.CallOpts)
}

// GetFunctionId is a free data retrieval call binding the contract method 0xd0bf5dab.
//
// Solidity: function getFunctionId(address _owner, bytes32 _salt) pure returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewayCaller) GetFunctionId(opts *bind.CallOpts, _owner common.Address, _salt [32]byte) ([32]byte, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "getFunctionId", _owner, _salt)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetFunctionId is a free data retrieval call binding the contract method 0xd0bf5dab.
//
// Solidity: function getFunctionId(address _owner, bytes32 _salt) pure returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewaySession) GetFunctionId(_owner common.Address, _salt [32]byte) ([32]byte, error) {
	return _SuccinctGateway.Contract.GetFunctionId(&_SuccinctGateway.CallOpts, _owner, _salt)
}

// GetFunctionId is a free data retrieval call binding the contract method 0xd0bf5dab.
//
// Solidity: function getFunctionId(address _owner, bytes32 _salt) pure returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewayCallerSession) GetFunctionId(_owner common.Address, _salt [32]byte) ([32]byte, error) {
	return _SuccinctGateway.Contract.GetFunctionId(&_SuccinctGateway.CallOpts, _owner, _salt)
}

// IsCallback is a free data retrieval call binding the contract method 0x05d7c1cf.
//
// Solidity: function isCallback() view returns(bool)
func (_SuccinctGateway *SuccinctGatewayCaller) IsCallback(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "isCallback")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsCallback is a free data retrieval call binding the contract method 0x05d7c1cf.
//
// Solidity: function isCallback() view returns(bool)
func (_SuccinctGateway *SuccinctGatewaySession) IsCallback() (bool, error) {
	return _SuccinctGateway.Contract.IsCallback(&_SuccinctGateway.CallOpts)
}

// IsCallback is a free data retrieval call binding the contract method 0x05d7c1cf.
//
// Solidity: function isCallback() view returns(bool)
func (_SuccinctGateway *SuccinctGatewayCallerSession) IsCallback() (bool, error) {
	return _SuccinctGateway.Contract.IsCallback(&_SuccinctGateway.CallOpts)
}

// Nonce is a free data retrieval call binding the contract method 0xaffed0e0.
//
// Solidity: function nonce() view returns(uint32)
func (_SuccinctGateway *SuccinctGatewayCaller) Nonce(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "nonce")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// Nonce is a free data retrieval call binding the contract method 0xaffed0e0.
//
// Solidity: function nonce() view returns(uint32)
func (_SuccinctGateway *SuccinctGatewaySession) Nonce() (uint32, error) {
	return _SuccinctGateway.Contract.Nonce(&_SuccinctGateway.CallOpts)
}

// Nonce is a free data retrieval call binding the contract method 0xaffed0e0.
//
// Solidity: function nonce() view returns(uint32)
func (_SuccinctGateway *SuccinctGatewayCallerSession) Nonce() (uint32, error) {
	return _SuccinctGateway.Contract.Nonce(&_SuccinctGateway.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_SuccinctGateway *SuccinctGatewayCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_SuccinctGateway *SuccinctGatewaySession) Owner() (common.Address, error) {
	return _SuccinctGateway.Contract.Owner(&_SuccinctGateway.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_SuccinctGateway *SuccinctGatewayCallerSession) Owner() (common.Address, error) {
	return _SuccinctGateway.Contract.Owner(&_SuccinctGateway.CallOpts)
}

// Requests is a free data retrieval call binding the contract method 0x0ab469b0.
//
// Solidity: function requests(uint32 ) view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayCaller) Requests(opts *bind.CallOpts, arg0 uint32) ([32]byte, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "requests", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// Requests is a free data retrieval call binding the contract method 0x0ab469b0.
//
// Solidity: function requests(uint32 ) view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewaySession) Requests(arg0 uint32) ([32]byte, error) {
	return _SuccinctGateway.Contract.Requests(&_SuccinctGateway.CallOpts, arg0)
}

// Requests is a free data retrieval call binding the contract method 0x0ab469b0.
//
// Solidity: function requests(uint32 ) view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayCallerSession) Requests(arg0 uint32) ([32]byte, error) {
	return _SuccinctGateway.Contract.Requests(&_SuccinctGateway.CallOpts, arg0)
}

// VerifiedCall is a free data retrieval call binding the contract method 0x176e62fd.
//
// Solidity: function verifiedCall(bytes32 _functionId, bytes _input) view returns(bytes)
func (_SuccinctGateway *SuccinctGatewayCaller) VerifiedCall(opts *bind.CallOpts, _functionId [32]byte, _input []byte) ([]byte, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "verifiedCall", _functionId, _input)

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// VerifiedCall is a free data retrieval call binding the contract method 0x176e62fd.
//
// Solidity: function verifiedCall(bytes32 _functionId, bytes _input) view returns(bytes)
func (_SuccinctGateway *SuccinctGatewaySession) VerifiedCall(_functionId [32]byte, _input []byte) ([]byte, error) {
	return _SuccinctGateway.Contract.VerifiedCall(&_SuccinctGateway.CallOpts, _functionId, _input)
}

// VerifiedCall is a free data retrieval call binding the contract method 0x176e62fd.
//
// Solidity: function verifiedCall(bytes32 _functionId, bytes _input) view returns(bytes)
func (_SuccinctGateway *SuccinctGatewayCallerSession) VerifiedCall(_functionId [32]byte, _input []byte) ([]byte, error) {
	return _SuccinctGateway.Contract.VerifiedCall(&_SuccinctGateway.CallOpts, _functionId, _input)
}

// VerifiedFunctionId is a free data retrieval call binding the contract method 0x8157ce2b.
//
// Solidity: function verifiedFunctionId() view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayCaller) VerifiedFunctionId(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "verifiedFunctionId")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// VerifiedFunctionId is a free data retrieval call binding the contract method 0x8157ce2b.
//
// Solidity: function verifiedFunctionId() view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewaySession) VerifiedFunctionId() ([32]byte, error) {
	return _SuccinctGateway.Contract.VerifiedFunctionId(&_SuccinctGateway.CallOpts)
}

// VerifiedFunctionId is a free data retrieval call binding the contract method 0x8157ce2b.
//
// Solidity: function verifiedFunctionId() view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayCallerSession) VerifiedFunctionId() ([32]byte, error) {
	return _SuccinctGateway.Contract.VerifiedFunctionId(&_SuccinctGateway.CallOpts)
}

// VerifiedInputHash is a free data retrieval call binding the contract method 0x80e0bbb0.
//
// Solidity: function verifiedInputHash() view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayCaller) VerifiedInputHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "verifiedInputHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// VerifiedInputHash is a free data retrieval call binding the contract method 0x80e0bbb0.
//
// Solidity: function verifiedInputHash() view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewaySession) VerifiedInputHash() ([32]byte, error) {
	return _SuccinctGateway.Contract.VerifiedInputHash(&_SuccinctGateway.CallOpts)
}

// VerifiedInputHash is a free data retrieval call binding the contract method 0x80e0bbb0.
//
// Solidity: function verifiedInputHash() view returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayCallerSession) VerifiedInputHash() ([32]byte, error) {
	return _SuccinctGateway.Contract.VerifiedInputHash(&_SuccinctGateway.CallOpts)
}

// VerifiedOutput is a free data retrieval call binding the contract method 0x173869f0.
//
// Solidity: function verifiedOutput() view returns(bytes)
func (_SuccinctGateway *SuccinctGatewayCaller) VerifiedOutput(opts *bind.CallOpts) ([]byte, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "verifiedOutput")

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// VerifiedOutput is a free data retrieval call binding the contract method 0x173869f0.
//
// Solidity: function verifiedOutput() view returns(bytes)
func (_SuccinctGateway *SuccinctGatewaySession) VerifiedOutput() ([]byte, error) {
	return _SuccinctGateway.Contract.VerifiedOutput(&_SuccinctGateway.CallOpts)
}

// VerifiedOutput is a free data retrieval call binding the contract method 0x173869f0.
//
// Solidity: function verifiedOutput() view returns(bytes)
func (_SuccinctGateway *SuccinctGatewayCallerSession) VerifiedOutput() ([]byte, error) {
	return _SuccinctGateway.Contract.VerifiedOutput(&_SuccinctGateway.CallOpts)
}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_SuccinctGateway *SuccinctGatewayCaller) VerifierOwners(opts *bind.CallOpts, arg0 [32]byte) (common.Address, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "verifierOwners", arg0)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_SuccinctGateway *SuccinctGatewaySession) VerifierOwners(arg0 [32]byte) (common.Address, error) {
	return _SuccinctGateway.Contract.VerifierOwners(&_SuccinctGateway.CallOpts, arg0)
}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_SuccinctGateway *SuccinctGatewayCallerSession) VerifierOwners(arg0 [32]byte) (common.Address, error) {
	return _SuccinctGateway.Contract.VerifierOwners(&_SuccinctGateway.CallOpts, arg0)
}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_SuccinctGateway *SuccinctGatewayCaller) Verifiers(opts *bind.CallOpts, arg0 [32]byte) (common.Address, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "verifiers", arg0)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_SuccinctGateway *SuccinctGatewaySession) Verifiers(arg0 [32]byte) (common.Address, error) {
	return _SuccinctGateway.Contract.Verifiers(&_SuccinctGateway.CallOpts, arg0)
}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_SuccinctGateway *SuccinctGatewayCallerSession) Verifiers(arg0 [32]byte) (common.Address, error) {
	return _SuccinctGateway.Contract.Verifiers(&_SuccinctGateway.CallOpts, arg0)
}

// WhitelistStatus is a free data retrieval call binding the contract method 0x493e9b81.
//
// Solidity: function whitelistStatus(bytes32 ) view returns(uint8)
func (_SuccinctGateway *SuccinctGatewayCaller) WhitelistStatus(opts *bind.CallOpts, arg0 [32]byte) (uint8, error) {
	var out []interface{}
	err := _SuccinctGateway.contract.Call(opts, &out, "whitelistStatus", arg0)

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// WhitelistStatus is a free data retrieval call binding the contract method 0x493e9b81.
//
// Solidity: function whitelistStatus(bytes32 ) view returns(uint8)
func (_SuccinctGateway *SuccinctGatewaySession) WhitelistStatus(arg0 [32]byte) (uint8, error) {
	return _SuccinctGateway.Contract.WhitelistStatus(&_SuccinctGateway.CallOpts, arg0)
}

// WhitelistStatus is a free data retrieval call binding the contract method 0x493e9b81.
//
// Solidity: function whitelistStatus(bytes32 ) view returns(uint8)
func (_SuccinctGateway *SuccinctGatewayCallerSession) WhitelistStatus(arg0 [32]byte) (uint8, error) {
	return _SuccinctGateway.Contract.WhitelistStatus(&_SuccinctGateway.CallOpts, arg0)
}

// AddCustomProver is a paid mutator transaction binding the contract method 0xb97ed3ca.
//
// Solidity: function addCustomProver(bytes32 _functionId, address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) AddCustomProver(opts *bind.TransactOpts, _functionId [32]byte, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "addCustomProver", _functionId, _prover)
}

// AddCustomProver is a paid mutator transaction binding the contract method 0xb97ed3ca.
//
// Solidity: function addCustomProver(bytes32 _functionId, address _prover) returns()
func (_SuccinctGateway *SuccinctGatewaySession) AddCustomProver(_functionId [32]byte, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.AddCustomProver(&_SuccinctGateway.TransactOpts, _functionId, _prover)
}

// AddCustomProver is a paid mutator transaction binding the contract method 0xb97ed3ca.
//
// Solidity: function addCustomProver(bytes32 _functionId, address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) AddCustomProver(_functionId [32]byte, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.AddCustomProver(&_SuccinctGateway.TransactOpts, _functionId, _prover)
}

// AddDefaultProver is a paid mutator transaction binding the contract method 0xdd469fa4.
//
// Solidity: function addDefaultProver(address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) AddDefaultProver(opts *bind.TransactOpts, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "addDefaultProver", _prover)
}

// AddDefaultProver is a paid mutator transaction binding the contract method 0xdd469fa4.
//
// Solidity: function addDefaultProver(address _prover) returns()
func (_SuccinctGateway *SuccinctGatewaySession) AddDefaultProver(_prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.AddDefaultProver(&_SuccinctGateway.TransactOpts, _prover)
}

// AddDefaultProver is a paid mutator transaction binding the contract method 0xdd469fa4.
//
// Solidity: function addDefaultProver(address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) AddDefaultProver(_prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.AddDefaultProver(&_SuccinctGateway.TransactOpts, _prover)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x3a446f68.
//
// Solidity: function deployAndRegisterFunction(address _owner, bytes _bytecode, bytes32 _salt) returns(bytes32 functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewayTransactor) DeployAndRegisterFunction(opts *bind.TransactOpts, _owner common.Address, _bytecode []byte, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "deployAndRegisterFunction", _owner, _bytecode, _salt)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x3a446f68.
//
// Solidity: function deployAndRegisterFunction(address _owner, bytes _bytecode, bytes32 _salt) returns(bytes32 functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewaySession) DeployAndRegisterFunction(_owner common.Address, _bytecode []byte, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.DeployAndRegisterFunction(&_SuccinctGateway.TransactOpts, _owner, _bytecode, _salt)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x3a446f68.
//
// Solidity: function deployAndRegisterFunction(address _owner, bytes _bytecode, bytes32 _salt) returns(bytes32 functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewayTransactorSession) DeployAndRegisterFunction(_owner common.Address, _bytecode []byte, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.DeployAndRegisterFunction(&_SuccinctGateway.TransactOpts, _owner, _bytecode, _salt)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xff0b2f8a.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, bytes32 _salt) returns(bytes32 functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewayTransactor) DeployAndUpdateFunction(opts *bind.TransactOpts, _bytecode []byte, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "deployAndUpdateFunction", _bytecode, _salt)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xff0b2f8a.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, bytes32 _salt) returns(bytes32 functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewaySession) DeployAndUpdateFunction(_bytecode []byte, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.DeployAndUpdateFunction(&_SuccinctGateway.TransactOpts, _bytecode, _salt)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xff0b2f8a.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, bytes32 _salt) returns(bytes32 functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewayTransactorSession) DeployAndUpdateFunction(_bytecode []byte, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.DeployAndUpdateFunction(&_SuccinctGateway.TransactOpts, _bytecode, _salt)
}

// FulfillCall is a paid mutator transaction binding the contract method 0xbac2a106.
//
// Solidity: function fulfillCall(bytes32 _functionId, bytes _input, bytes _output, bytes _proof, address _callbackAddress, bytes _callbackData) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) FulfillCall(opts *bind.TransactOpts, _functionId [32]byte, _input []byte, _output []byte, _proof []byte, _callbackAddress common.Address, _callbackData []byte) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "fulfillCall", _functionId, _input, _output, _proof, _callbackAddress, _callbackData)
}

// FulfillCall is a paid mutator transaction binding the contract method 0xbac2a106.
//
// Solidity: function fulfillCall(bytes32 _functionId, bytes _input, bytes _output, bytes _proof, address _callbackAddress, bytes _callbackData) returns()
func (_SuccinctGateway *SuccinctGatewaySession) FulfillCall(_functionId [32]byte, _input []byte, _output []byte, _proof []byte, _callbackAddress common.Address, _callbackData []byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.FulfillCall(&_SuccinctGateway.TransactOpts, _functionId, _input, _output, _proof, _callbackAddress, _callbackData)
}

// FulfillCall is a paid mutator transaction binding the contract method 0xbac2a106.
//
// Solidity: function fulfillCall(bytes32 _functionId, bytes _input, bytes _output, bytes _proof, address _callbackAddress, bytes _callbackData) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) FulfillCall(_functionId [32]byte, _input []byte, _output []byte, _proof []byte, _callbackAddress common.Address, _callbackData []byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.FulfillCall(&_SuccinctGateway.TransactOpts, _functionId, _input, _output, _proof, _callbackAddress, _callbackData)
}

// FulfillCallback is a paid mutator transaction binding the contract method 0x78370ebd.
//
// Solidity: function fulfillCallback(uint32 _nonce, bytes32 _functionId, bytes32 _inputHash, address _callbackAddress, bytes4 _callbackSelector, uint32 _callbackGasLimit, bytes _context, bytes _output, bytes _proof) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) FulfillCallback(opts *bind.TransactOpts, _nonce uint32, _functionId [32]byte, _inputHash [32]byte, _callbackAddress common.Address, _callbackSelector [4]byte, _callbackGasLimit uint32, _context []byte, _output []byte, _proof []byte) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "fulfillCallback", _nonce, _functionId, _inputHash, _callbackAddress, _callbackSelector, _callbackGasLimit, _context, _output, _proof)
}

// FulfillCallback is a paid mutator transaction binding the contract method 0x78370ebd.
//
// Solidity: function fulfillCallback(uint32 _nonce, bytes32 _functionId, bytes32 _inputHash, address _callbackAddress, bytes4 _callbackSelector, uint32 _callbackGasLimit, bytes _context, bytes _output, bytes _proof) returns()
func (_SuccinctGateway *SuccinctGatewaySession) FulfillCallback(_nonce uint32, _functionId [32]byte, _inputHash [32]byte, _callbackAddress common.Address, _callbackSelector [4]byte, _callbackGasLimit uint32, _context []byte, _output []byte, _proof []byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.FulfillCallback(&_SuccinctGateway.TransactOpts, _nonce, _functionId, _inputHash, _callbackAddress, _callbackSelector, _callbackGasLimit, _context, _output, _proof)
}

// FulfillCallback is a paid mutator transaction binding the contract method 0x78370ebd.
//
// Solidity: function fulfillCallback(uint32 _nonce, bytes32 _functionId, bytes32 _inputHash, address _callbackAddress, bytes4 _callbackSelector, uint32 _callbackGasLimit, bytes _context, bytes _output, bytes _proof) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) FulfillCallback(_nonce uint32, _functionId [32]byte, _inputHash [32]byte, _callbackAddress common.Address, _callbackSelector [4]byte, _callbackGasLimit uint32, _context []byte, _output []byte, _proof []byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.FulfillCallback(&_SuccinctGateway.TransactOpts, _nonce, _functionId, _inputHash, _callbackAddress, _callbackSelector, _callbackGasLimit, _context, _output, _proof)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _owner, address _feeVault, address _defaultProver) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) Initialize(opts *bind.TransactOpts, _owner common.Address, _feeVault common.Address, _defaultProver common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "initialize", _owner, _feeVault, _defaultProver)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _owner, address _feeVault, address _defaultProver) returns()
func (_SuccinctGateway *SuccinctGatewaySession) Initialize(_owner common.Address, _feeVault common.Address, _defaultProver common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.Initialize(&_SuccinctGateway.TransactOpts, _owner, _feeVault, _defaultProver)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address _owner, address _feeVault, address _defaultProver) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) Initialize(_owner common.Address, _feeVault common.Address, _defaultProver common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.Initialize(&_SuccinctGateway.TransactOpts, _owner, _feeVault, _defaultProver)
}

// Recover is a paid mutator transaction binding the contract method 0x5705ae43.
//
// Solidity: function recover(address _to, uint256 _amount) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) Recover(opts *bind.TransactOpts, _to common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "recover", _to, _amount)
}

// Recover is a paid mutator transaction binding the contract method 0x5705ae43.
//
// Solidity: function recover(address _to, uint256 _amount) returns()
func (_SuccinctGateway *SuccinctGatewaySession) Recover(_to common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.Recover(&_SuccinctGateway.TransactOpts, _to, _amount)
}

// Recover is a paid mutator transaction binding the contract method 0x5705ae43.
//
// Solidity: function recover(address _to, uint256 _amount) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) Recover(_to common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.Recover(&_SuccinctGateway.TransactOpts, _to, _amount)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x5e5da3b6.
//
// Solidity: function registerFunction(address _owner, address _verifier, bytes32 _salt) returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewayTransactor) RegisterFunction(opts *bind.TransactOpts, _owner common.Address, _verifier common.Address, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "registerFunction", _owner, _verifier, _salt)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x5e5da3b6.
//
// Solidity: function registerFunction(address _owner, address _verifier, bytes32 _salt) returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewaySession) RegisterFunction(_owner common.Address, _verifier common.Address, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RegisterFunction(&_SuccinctGateway.TransactOpts, _owner, _verifier, _salt)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x5e5da3b6.
//
// Solidity: function registerFunction(address _owner, address _verifier, bytes32 _salt) returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewayTransactorSession) RegisterFunction(_owner common.Address, _verifier common.Address, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RegisterFunction(&_SuccinctGateway.TransactOpts, _owner, _verifier, _salt)
}

// RemoveCustomProver is a paid mutator transaction binding the contract method 0x164d4207.
//
// Solidity: function removeCustomProver(bytes32 _functionId, address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) RemoveCustomProver(opts *bind.TransactOpts, _functionId [32]byte, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "removeCustomProver", _functionId, _prover)
}

// RemoveCustomProver is a paid mutator transaction binding the contract method 0x164d4207.
//
// Solidity: function removeCustomProver(bytes32 _functionId, address _prover) returns()
func (_SuccinctGateway *SuccinctGatewaySession) RemoveCustomProver(_functionId [32]byte, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RemoveCustomProver(&_SuccinctGateway.TransactOpts, _functionId, _prover)
}

// RemoveCustomProver is a paid mutator transaction binding the contract method 0x164d4207.
//
// Solidity: function removeCustomProver(bytes32 _functionId, address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) RemoveCustomProver(_functionId [32]byte, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RemoveCustomProver(&_SuccinctGateway.TransactOpts, _functionId, _prover)
}

// RemoveDefaultProver is a paid mutator transaction binding the contract method 0x7413555d.
//
// Solidity: function removeDefaultProver(address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) RemoveDefaultProver(opts *bind.TransactOpts, _prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "removeDefaultProver", _prover)
}

// RemoveDefaultProver is a paid mutator transaction binding the contract method 0x7413555d.
//
// Solidity: function removeDefaultProver(address _prover) returns()
func (_SuccinctGateway *SuccinctGatewaySession) RemoveDefaultProver(_prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RemoveDefaultProver(&_SuccinctGateway.TransactOpts, _prover)
}

// RemoveDefaultProver is a paid mutator transaction binding the contract method 0x7413555d.
//
// Solidity: function removeDefaultProver(address _prover) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) RemoveDefaultProver(_prover common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RemoveDefaultProver(&_SuccinctGateway.TransactOpts, _prover)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_SuccinctGateway *SuccinctGatewaySession) RenounceOwnership() (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RenounceOwnership(&_SuccinctGateway.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RenounceOwnership(&_SuccinctGateway.TransactOpts)
}

// RequestCall is a paid mutator transaction binding the contract method 0x436a61d5.
//
// Solidity: function requestCall(bytes32 _functionId, bytes _input, address _entryAddress, bytes _entryCalldata, uint32 _entryGasLimit) payable returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) RequestCall(opts *bind.TransactOpts, _functionId [32]byte, _input []byte, _entryAddress common.Address, _entryCalldata []byte, _entryGasLimit uint32) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "requestCall", _functionId, _input, _entryAddress, _entryCalldata, _entryGasLimit)
}

// RequestCall is a paid mutator transaction binding the contract method 0x436a61d5.
//
// Solidity: function requestCall(bytes32 _functionId, bytes _input, address _entryAddress, bytes _entryCalldata, uint32 _entryGasLimit) payable returns()
func (_SuccinctGateway *SuccinctGatewaySession) RequestCall(_functionId [32]byte, _input []byte, _entryAddress common.Address, _entryCalldata []byte, _entryGasLimit uint32) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RequestCall(&_SuccinctGateway.TransactOpts, _functionId, _input, _entryAddress, _entryCalldata, _entryGasLimit)
}

// RequestCall is a paid mutator transaction binding the contract method 0x436a61d5.
//
// Solidity: function requestCall(bytes32 _functionId, bytes _input, address _entryAddress, bytes _entryCalldata, uint32 _entryGasLimit) payable returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) RequestCall(_functionId [32]byte, _input []byte, _entryAddress common.Address, _entryCalldata []byte, _entryGasLimit uint32) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RequestCall(&_SuccinctGateway.TransactOpts, _functionId, _input, _entryAddress, _entryCalldata, _entryGasLimit)
}

// RequestCallback is a paid mutator transaction binding the contract method 0xb3f04fdf.
//
// Solidity: function requestCallback(bytes32 _functionId, bytes _input, bytes _context, bytes4 _callbackSelector, uint32 _callbackGasLimit) payable returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayTransactor) RequestCallback(opts *bind.TransactOpts, _functionId [32]byte, _input []byte, _context []byte, _callbackSelector [4]byte, _callbackGasLimit uint32) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "requestCallback", _functionId, _input, _context, _callbackSelector, _callbackGasLimit)
}

// RequestCallback is a paid mutator transaction binding the contract method 0xb3f04fdf.
//
// Solidity: function requestCallback(bytes32 _functionId, bytes _input, bytes _context, bytes4 _callbackSelector, uint32 _callbackGasLimit) payable returns(bytes32)
func (_SuccinctGateway *SuccinctGatewaySession) RequestCallback(_functionId [32]byte, _input []byte, _context []byte, _callbackSelector [4]byte, _callbackGasLimit uint32) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RequestCallback(&_SuccinctGateway.TransactOpts, _functionId, _input, _context, _callbackSelector, _callbackGasLimit)
}

// RequestCallback is a paid mutator transaction binding the contract method 0xb3f04fdf.
//
// Solidity: function requestCallback(bytes32 _functionId, bytes _input, bytes _context, bytes4 _callbackSelector, uint32 _callbackGasLimit) payable returns(bytes32)
func (_SuccinctGateway *SuccinctGatewayTransactorSession) RequestCallback(_functionId [32]byte, _input []byte, _context []byte, _callbackSelector [4]byte, _callbackGasLimit uint32) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.RequestCallback(&_SuccinctGateway.TransactOpts, _functionId, _input, _context, _callbackSelector, _callbackGasLimit)
}

// SetFeeVault is a paid mutator transaction binding the contract method 0xa591f97f.
//
// Solidity: function setFeeVault(address _feeVault) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) SetFeeVault(opts *bind.TransactOpts, _feeVault common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "setFeeVault", _feeVault)
}

// SetFeeVault is a paid mutator transaction binding the contract method 0xa591f97f.
//
// Solidity: function setFeeVault(address _feeVault) returns()
func (_SuccinctGateway *SuccinctGatewaySession) SetFeeVault(_feeVault common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.SetFeeVault(&_SuccinctGateway.TransactOpts, _feeVault)
}

// SetFeeVault is a paid mutator transaction binding the contract method 0xa591f97f.
//
// Solidity: function setFeeVault(address _feeVault) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) SetFeeVault(_feeVault common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.SetFeeVault(&_SuccinctGateway.TransactOpts, _feeVault)
}

// SetWhitelistStatus is a paid mutator transaction binding the contract method 0x420d334c.
//
// Solidity: function setWhitelistStatus(bytes32 _functionId, uint8 _status) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) SetWhitelistStatus(opts *bind.TransactOpts, _functionId [32]byte, _status uint8) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "setWhitelistStatus", _functionId, _status)
}

// SetWhitelistStatus is a paid mutator transaction binding the contract method 0x420d334c.
//
// Solidity: function setWhitelistStatus(bytes32 _functionId, uint8 _status) returns()
func (_SuccinctGateway *SuccinctGatewaySession) SetWhitelistStatus(_functionId [32]byte, _status uint8) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.SetWhitelistStatus(&_SuccinctGateway.TransactOpts, _functionId, _status)
}

// SetWhitelistStatus is a paid mutator transaction binding the contract method 0x420d334c.
//
// Solidity: function setWhitelistStatus(bytes32 _functionId, uint8 _status) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) SetWhitelistStatus(_functionId [32]byte, _status uint8) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.SetWhitelistStatus(&_SuccinctGateway.TransactOpts, _functionId, _status)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_SuccinctGateway *SuccinctGatewayTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_SuccinctGateway *SuccinctGatewaySession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.TransferOwnership(&_SuccinctGateway.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_SuccinctGateway *SuccinctGatewayTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.TransferOwnership(&_SuccinctGateway.TransactOpts, newOwner)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xca6b63c7.
//
// Solidity: function updateFunction(address _verifier, bytes32 _salt) returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewayTransactor) UpdateFunction(opts *bind.TransactOpts, _verifier common.Address, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.contract.Transact(opts, "updateFunction", _verifier, _salt)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xca6b63c7.
//
// Solidity: function updateFunction(address _verifier, bytes32 _salt) returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewaySession) UpdateFunction(_verifier common.Address, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.UpdateFunction(&_SuccinctGateway.TransactOpts, _verifier, _salt)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xca6b63c7.
//
// Solidity: function updateFunction(address _verifier, bytes32 _salt) returns(bytes32 functionId)
func (_SuccinctGateway *SuccinctGatewayTransactorSession) UpdateFunction(_verifier common.Address, _salt [32]byte) (*types.Transaction, error) {
	return _SuccinctGateway.Contract.UpdateFunction(&_SuccinctGateway.TransactOpts, _verifier, _salt)
}

// SuccinctGatewayCallIterator is returned from FilterCall and is used to iterate over the raw logs and unpacked data for Call events raised by the SuccinctGateway contract.
type SuccinctGatewayCallIterator struct {
	Event *SuccinctGatewayCall // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayCallIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayCall)
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
		it.Event = new(SuccinctGatewayCall)
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
func (it *SuccinctGatewayCallIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayCallIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayCall represents a Call event raised by the SuccinctGateway contract.
type SuccinctGatewayCall struct {
	FunctionId [32]byte
	InputHash  [32]byte
	OutputHash [32]byte
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterCall is a free log retrieval operation binding the contract event 0x41d7122d18af9f0c92f23bcea9d5fa416cadcd1ed2fc8e544a3c89b841ecfd15.
//
// Solidity: event Call(bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterCall(opts *bind.FilterOpts, functionId [][32]byte) (*SuccinctGatewayCallIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "Call", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayCallIterator{contract: _SuccinctGateway.contract, event: "Call", logs: logs, sub: sub}, nil
}

// WatchCall is a free log subscription operation binding the contract event 0x41d7122d18af9f0c92f23bcea9d5fa416cadcd1ed2fc8e544a3c89b841ecfd15.
//
// Solidity: event Call(bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchCall(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayCall, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "Call", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayCall)
				if err := _SuccinctGateway.contract.UnpackLog(event, "Call", log); err != nil {
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

// ParseCall is a log parse operation binding the contract event 0x41d7122d18af9f0c92f23bcea9d5fa416cadcd1ed2fc8e544a3c89b841ecfd15.
//
// Solidity: event Call(bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseCall(log types.Log) (*SuccinctGatewayCall, error) {
	event := new(SuccinctGatewayCall)
	if err := _SuccinctGateway.contract.UnpackLog(event, "Call", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayDeployedIterator is returned from FilterDeployed and is used to iterate over the raw logs and unpacked data for Deployed events raised by the SuccinctGateway contract.
type SuccinctGatewayDeployedIterator struct {
	Event *SuccinctGatewayDeployed // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayDeployedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayDeployed)
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
		it.Event = new(SuccinctGatewayDeployed)
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
func (it *SuccinctGatewayDeployedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayDeployedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayDeployed represents a Deployed event raised by the SuccinctGateway contract.
type SuccinctGatewayDeployed struct {
	BytecodeHash    [32]byte
	Salt            [32]byte
	DeployedAddress common.Address
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterDeployed is a free log retrieval operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterDeployed(opts *bind.FilterOpts, bytecodeHash [][32]byte, salt [][32]byte, deployedAddress []common.Address) (*SuccinctGatewayDeployedIterator, error) {

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

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "Deployed", bytecodeHashRule, saltRule, deployedAddressRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayDeployedIterator{contract: _SuccinctGateway.contract, event: "Deployed", logs: logs, sub: sub}, nil
}

// WatchDeployed is a free log subscription operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchDeployed(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayDeployed, bytecodeHash [][32]byte, salt [][32]byte, deployedAddress []common.Address) (event.Subscription, error) {

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

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "Deployed", bytecodeHashRule, saltRule, deployedAddressRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayDeployed)
				if err := _SuccinctGateway.contract.UnpackLog(event, "Deployed", log); err != nil {
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
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseDeployed(log types.Log) (*SuccinctGatewayDeployed, error) {
	event := new(SuccinctGatewayDeployed)
	if err := _SuccinctGateway.contract.UnpackLog(event, "Deployed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayFunctionRegisteredIterator is returned from FilterFunctionRegistered and is used to iterate over the raw logs and unpacked data for FunctionRegistered events raised by the SuccinctGateway contract.
type SuccinctGatewayFunctionRegisteredIterator struct {
	Event *SuccinctGatewayFunctionRegistered // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayFunctionRegisteredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayFunctionRegistered)
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
		it.Event = new(SuccinctGatewayFunctionRegistered)
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
func (it *SuccinctGatewayFunctionRegisteredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayFunctionRegisteredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayFunctionRegistered represents a FunctionRegistered event raised by the SuccinctGateway contract.
type SuccinctGatewayFunctionRegistered struct {
	FunctionId [32]byte
	Verifier   common.Address
	Salt       [32]byte
	Owner      common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionRegistered is a free log retrieval operation binding the contract event 0xdfa40ee17618caabca7b46eb80031e3d05a27a30de957d29b4a2a71426db5a63.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, bytes32 salt, address owner)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterFunctionRegistered(opts *bind.FilterOpts, functionId [][32]byte) (*SuccinctGatewayFunctionRegisteredIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "FunctionRegistered", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayFunctionRegisteredIterator{contract: _SuccinctGateway.contract, event: "FunctionRegistered", logs: logs, sub: sub}, nil
}

// WatchFunctionRegistered is a free log subscription operation binding the contract event 0xdfa40ee17618caabca7b46eb80031e3d05a27a30de957d29b4a2a71426db5a63.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, bytes32 salt, address owner)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchFunctionRegistered(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayFunctionRegistered, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "FunctionRegistered", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayFunctionRegistered)
				if err := _SuccinctGateway.contract.UnpackLog(event, "FunctionRegistered", log); err != nil {
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

// ParseFunctionRegistered is a log parse operation binding the contract event 0xdfa40ee17618caabca7b46eb80031e3d05a27a30de957d29b4a2a71426db5a63.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, bytes32 salt, address owner)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseFunctionRegistered(log types.Log) (*SuccinctGatewayFunctionRegistered, error) {
	event := new(SuccinctGatewayFunctionRegistered)
	if err := _SuccinctGateway.contract.UnpackLog(event, "FunctionRegistered", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayFunctionVerifierUpdatedIterator is returned from FilterFunctionVerifierUpdated and is used to iterate over the raw logs and unpacked data for FunctionVerifierUpdated events raised by the SuccinctGateway contract.
type SuccinctGatewayFunctionVerifierUpdatedIterator struct {
	Event *SuccinctGatewayFunctionVerifierUpdated // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayFunctionVerifierUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayFunctionVerifierUpdated)
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
		it.Event = new(SuccinctGatewayFunctionVerifierUpdated)
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
func (it *SuccinctGatewayFunctionVerifierUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayFunctionVerifierUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayFunctionVerifierUpdated represents a FunctionVerifierUpdated event raised by the SuccinctGateway contract.
type SuccinctGatewayFunctionVerifierUpdated struct {
	FunctionId [32]byte
	Verifier   common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionVerifierUpdated is a free log retrieval operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterFunctionVerifierUpdated(opts *bind.FilterOpts, functionId [][32]byte) (*SuccinctGatewayFunctionVerifierUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "FunctionVerifierUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayFunctionVerifierUpdatedIterator{contract: _SuccinctGateway.contract, event: "FunctionVerifierUpdated", logs: logs, sub: sub}, nil
}

// WatchFunctionVerifierUpdated is a free log subscription operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchFunctionVerifierUpdated(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayFunctionVerifierUpdated, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "FunctionVerifierUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayFunctionVerifierUpdated)
				if err := _SuccinctGateway.contract.UnpackLog(event, "FunctionVerifierUpdated", log); err != nil {
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
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseFunctionVerifierUpdated(log types.Log) (*SuccinctGatewayFunctionVerifierUpdated, error) {
	event := new(SuccinctGatewayFunctionVerifierUpdated)
	if err := _SuccinctGateway.contract.UnpackLog(event, "FunctionVerifierUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the SuccinctGateway contract.
type SuccinctGatewayInitializedIterator struct {
	Event *SuccinctGatewayInitialized // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayInitialized)
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
		it.Event = new(SuccinctGatewayInitialized)
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
func (it *SuccinctGatewayInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayInitialized represents a Initialized event raised by the SuccinctGateway contract.
type SuccinctGatewayInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterInitialized(opts *bind.FilterOpts) (*SuccinctGatewayInitializedIterator, error) {

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayInitializedIterator{contract: _SuccinctGateway.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayInitialized) (event.Subscription, error) {

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayInitialized)
				if err := _SuccinctGateway.contract.UnpackLog(event, "Initialized", log); err != nil {
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

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseInitialized(log types.Log) (*SuccinctGatewayInitialized, error) {
	event := new(SuccinctGatewayInitialized)
	if err := _SuccinctGateway.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the SuccinctGateway contract.
type SuccinctGatewayOwnershipTransferredIterator struct {
	Event *SuccinctGatewayOwnershipTransferred // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayOwnershipTransferred)
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
		it.Event = new(SuccinctGatewayOwnershipTransferred)
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
func (it *SuccinctGatewayOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayOwnershipTransferred represents a OwnershipTransferred event raised by the SuccinctGateway contract.
type SuccinctGatewayOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*SuccinctGatewayOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayOwnershipTransferredIterator{contract: _SuccinctGateway.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayOwnershipTransferred)
				if err := _SuccinctGateway.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
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
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseOwnershipTransferred(log types.Log) (*SuccinctGatewayOwnershipTransferred, error) {
	event := new(SuccinctGatewayOwnershipTransferred)
	if err := _SuccinctGateway.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayProverUpdatedIterator is returned from FilterProverUpdated and is used to iterate over the raw logs and unpacked data for ProverUpdated events raised by the SuccinctGateway contract.
type SuccinctGatewayProverUpdatedIterator struct {
	Event *SuccinctGatewayProverUpdated // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayProverUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayProverUpdated)
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
		it.Event = new(SuccinctGatewayProverUpdated)
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
func (it *SuccinctGatewayProverUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayProverUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayProverUpdated represents a ProverUpdated event raised by the SuccinctGateway contract.
type SuccinctGatewayProverUpdated struct {
	FunctionId [32]byte
	Prover     common.Address
	Added      bool
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterProverUpdated is a free log retrieval operation binding the contract event 0x1c19213e895a5cf2a6027ac01d21289130e55827ec6257a8bb946b4c524d408f.
//
// Solidity: event ProverUpdated(bytes32 indexed functionId, address indexed prover, bool added)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterProverUpdated(opts *bind.FilterOpts, functionId [][32]byte, prover []common.Address) (*SuccinctGatewayProverUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}
	var proverRule []interface{}
	for _, proverItem := range prover {
		proverRule = append(proverRule, proverItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "ProverUpdated", functionIdRule, proverRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayProverUpdatedIterator{contract: _SuccinctGateway.contract, event: "ProverUpdated", logs: logs, sub: sub}, nil
}

// WatchProverUpdated is a free log subscription operation binding the contract event 0x1c19213e895a5cf2a6027ac01d21289130e55827ec6257a8bb946b4c524d408f.
//
// Solidity: event ProverUpdated(bytes32 indexed functionId, address indexed prover, bool added)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchProverUpdated(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayProverUpdated, functionId [][32]byte, prover []common.Address) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}
	var proverRule []interface{}
	for _, proverItem := range prover {
		proverRule = append(proverRule, proverItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "ProverUpdated", functionIdRule, proverRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayProverUpdated)
				if err := _SuccinctGateway.contract.UnpackLog(event, "ProverUpdated", log); err != nil {
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

// ParseProverUpdated is a log parse operation binding the contract event 0x1c19213e895a5cf2a6027ac01d21289130e55827ec6257a8bb946b4c524d408f.
//
// Solidity: event ProverUpdated(bytes32 indexed functionId, address indexed prover, bool added)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseProverUpdated(log types.Log) (*SuccinctGatewayProverUpdated, error) {
	event := new(SuccinctGatewayProverUpdated)
	if err := _SuccinctGateway.contract.UnpackLog(event, "ProverUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayRequestCallIterator is returned from FilterRequestCall and is used to iterate over the raw logs and unpacked data for RequestCall events raised by the SuccinctGateway contract.
type SuccinctGatewayRequestCallIterator struct {
	Event *SuccinctGatewayRequestCall // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayRequestCallIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayRequestCall)
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
		it.Event = new(SuccinctGatewayRequestCall)
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
func (it *SuccinctGatewayRequestCallIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayRequestCallIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayRequestCall represents a RequestCall event raised by the SuccinctGateway contract.
type SuccinctGatewayRequestCall struct {
	FunctionId    [32]byte
	Input         []byte
	EntryAddress  common.Address
	EntryCalldata []byte
	EntryGasLimit uint32
	Sender        common.Address
	FeeAmount     *big.Int
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterRequestCall is a free log retrieval operation binding the contract event 0x88632d59d3df3bee2ce2a06fbb05e1c8542c44cb2e7339bb1812500a978644d3.
//
// Solidity: event RequestCall(bytes32 indexed functionId, bytes input, address entryAddress, bytes entryCalldata, uint32 entryGasLimit, address sender, uint256 feeAmount)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterRequestCall(opts *bind.FilterOpts, functionId [][32]byte) (*SuccinctGatewayRequestCallIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "RequestCall", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayRequestCallIterator{contract: _SuccinctGateway.contract, event: "RequestCall", logs: logs, sub: sub}, nil
}

// WatchRequestCall is a free log subscription operation binding the contract event 0x88632d59d3df3bee2ce2a06fbb05e1c8542c44cb2e7339bb1812500a978644d3.
//
// Solidity: event RequestCall(bytes32 indexed functionId, bytes input, address entryAddress, bytes entryCalldata, uint32 entryGasLimit, address sender, uint256 feeAmount)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchRequestCall(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayRequestCall, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "RequestCall", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayRequestCall)
				if err := _SuccinctGateway.contract.UnpackLog(event, "RequestCall", log); err != nil {
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

// ParseRequestCall is a log parse operation binding the contract event 0x88632d59d3df3bee2ce2a06fbb05e1c8542c44cb2e7339bb1812500a978644d3.
//
// Solidity: event RequestCall(bytes32 indexed functionId, bytes input, address entryAddress, bytes entryCalldata, uint32 entryGasLimit, address sender, uint256 feeAmount)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseRequestCall(log types.Log) (*SuccinctGatewayRequestCall, error) {
	event := new(SuccinctGatewayRequestCall)
	if err := _SuccinctGateway.contract.UnpackLog(event, "RequestCall", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayRequestCallbackIterator is returned from FilterRequestCallback and is used to iterate over the raw logs and unpacked data for RequestCallback events raised by the SuccinctGateway contract.
type SuccinctGatewayRequestCallbackIterator struct {
	Event *SuccinctGatewayRequestCallback // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayRequestCallbackIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayRequestCallback)
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
		it.Event = new(SuccinctGatewayRequestCallback)
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
func (it *SuccinctGatewayRequestCallbackIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayRequestCallbackIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayRequestCallback represents a RequestCallback event raised by the SuccinctGateway contract.
type SuccinctGatewayRequestCallback struct {
	Nonce            uint32
	FunctionId       [32]byte
	Input            []byte
	Context          []byte
	CallbackAddress  common.Address
	CallbackSelector [4]byte
	CallbackGasLimit uint32
	FeeAmount        *big.Int
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterRequestCallback is a free log retrieval operation binding the contract event 0x22a09d598b323a3c65d69787dd6fd143dd8e4d2f91733c247113167df31e3e93.
//
// Solidity: event RequestCallback(uint32 indexed nonce, bytes32 indexed functionId, bytes input, bytes context, address callbackAddress, bytes4 callbackSelector, uint32 callbackGasLimit, uint256 feeAmount)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterRequestCallback(opts *bind.FilterOpts, nonce []uint32, functionId [][32]byte) (*SuccinctGatewayRequestCallbackIterator, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "RequestCallback", nonceRule, functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayRequestCallbackIterator{contract: _SuccinctGateway.contract, event: "RequestCallback", logs: logs, sub: sub}, nil
}

// WatchRequestCallback is a free log subscription operation binding the contract event 0x22a09d598b323a3c65d69787dd6fd143dd8e4d2f91733c247113167df31e3e93.
//
// Solidity: event RequestCallback(uint32 indexed nonce, bytes32 indexed functionId, bytes input, bytes context, address callbackAddress, bytes4 callbackSelector, uint32 callbackGasLimit, uint256 feeAmount)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchRequestCallback(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayRequestCallback, nonce []uint32, functionId [][32]byte) (event.Subscription, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "RequestCallback", nonceRule, functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayRequestCallback)
				if err := _SuccinctGateway.contract.UnpackLog(event, "RequestCallback", log); err != nil {
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

// ParseRequestCallback is a log parse operation binding the contract event 0x22a09d598b323a3c65d69787dd6fd143dd8e4d2f91733c247113167df31e3e93.
//
// Solidity: event RequestCallback(uint32 indexed nonce, bytes32 indexed functionId, bytes input, bytes context, address callbackAddress, bytes4 callbackSelector, uint32 callbackGasLimit, uint256 feeAmount)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseRequestCallback(log types.Log) (*SuccinctGatewayRequestCallback, error) {
	event := new(SuccinctGatewayRequestCallback)
	if err := _SuccinctGateway.contract.UnpackLog(event, "RequestCallback", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayRequestFulfilledIterator is returned from FilterRequestFulfilled and is used to iterate over the raw logs and unpacked data for RequestFulfilled events raised by the SuccinctGateway contract.
type SuccinctGatewayRequestFulfilledIterator struct {
	Event *SuccinctGatewayRequestFulfilled // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayRequestFulfilledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayRequestFulfilled)
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
		it.Event = new(SuccinctGatewayRequestFulfilled)
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
func (it *SuccinctGatewayRequestFulfilledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayRequestFulfilledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayRequestFulfilled represents a RequestFulfilled event raised by the SuccinctGateway contract.
type SuccinctGatewayRequestFulfilled struct {
	Nonce      uint32
	FunctionId [32]byte
	InputHash  [32]byte
	OutputHash [32]byte
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterRequestFulfilled is a free log retrieval operation binding the contract event 0x361a2fc76bc9f35b079dd353fd7fdd8aaf61f1a7979cf59653225692c19bbff2.
//
// Solidity: event RequestFulfilled(uint32 indexed nonce, bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterRequestFulfilled(opts *bind.FilterOpts, nonce []uint32, functionId [][32]byte) (*SuccinctGatewayRequestFulfilledIterator, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "RequestFulfilled", nonceRule, functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayRequestFulfilledIterator{contract: _SuccinctGateway.contract, event: "RequestFulfilled", logs: logs, sub: sub}, nil
}

// WatchRequestFulfilled is a free log subscription operation binding the contract event 0x361a2fc76bc9f35b079dd353fd7fdd8aaf61f1a7979cf59653225692c19bbff2.
//
// Solidity: event RequestFulfilled(uint32 indexed nonce, bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchRequestFulfilled(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayRequestFulfilled, nonce []uint32, functionId [][32]byte) (event.Subscription, error) {

	var nonceRule []interface{}
	for _, nonceItem := range nonce {
		nonceRule = append(nonceRule, nonceItem)
	}
	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "RequestFulfilled", nonceRule, functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayRequestFulfilled)
				if err := _SuccinctGateway.contract.UnpackLog(event, "RequestFulfilled", log); err != nil {
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

// ParseRequestFulfilled is a log parse operation binding the contract event 0x361a2fc76bc9f35b079dd353fd7fdd8aaf61f1a7979cf59653225692c19bbff2.
//
// Solidity: event RequestFulfilled(uint32 indexed nonce, bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseRequestFulfilled(log types.Log) (*SuccinctGatewayRequestFulfilled, error) {
	event := new(SuccinctGatewayRequestFulfilled)
	if err := _SuccinctGateway.contract.UnpackLog(event, "RequestFulfilled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewaySetFeeVaultIterator is returned from FilterSetFeeVault and is used to iterate over the raw logs and unpacked data for SetFeeVault events raised by the SuccinctGateway contract.
type SuccinctGatewaySetFeeVaultIterator struct {
	Event *SuccinctGatewaySetFeeVault // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewaySetFeeVaultIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewaySetFeeVault)
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
		it.Event = new(SuccinctGatewaySetFeeVault)
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
func (it *SuccinctGatewaySetFeeVaultIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewaySetFeeVaultIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewaySetFeeVault represents a SetFeeVault event raised by the SuccinctGateway contract.
type SuccinctGatewaySetFeeVault struct {
	OldFeeVault common.Address
	NewFeeVault common.Address
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterSetFeeVault is a free log retrieval operation binding the contract event 0xf0cca8e172b90b70922c6757d918f7a532326dfd3e9f3c5b117a616d2bb07212.
//
// Solidity: event SetFeeVault(address indexed oldFeeVault, address indexed newFeeVault)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterSetFeeVault(opts *bind.FilterOpts, oldFeeVault []common.Address, newFeeVault []common.Address) (*SuccinctGatewaySetFeeVaultIterator, error) {

	var oldFeeVaultRule []interface{}
	for _, oldFeeVaultItem := range oldFeeVault {
		oldFeeVaultRule = append(oldFeeVaultRule, oldFeeVaultItem)
	}
	var newFeeVaultRule []interface{}
	for _, newFeeVaultItem := range newFeeVault {
		newFeeVaultRule = append(newFeeVaultRule, newFeeVaultItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "SetFeeVault", oldFeeVaultRule, newFeeVaultRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewaySetFeeVaultIterator{contract: _SuccinctGateway.contract, event: "SetFeeVault", logs: logs, sub: sub}, nil
}

// WatchSetFeeVault is a free log subscription operation binding the contract event 0xf0cca8e172b90b70922c6757d918f7a532326dfd3e9f3c5b117a616d2bb07212.
//
// Solidity: event SetFeeVault(address indexed oldFeeVault, address indexed newFeeVault)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchSetFeeVault(opts *bind.WatchOpts, sink chan<- *SuccinctGatewaySetFeeVault, oldFeeVault []common.Address, newFeeVault []common.Address) (event.Subscription, error) {

	var oldFeeVaultRule []interface{}
	for _, oldFeeVaultItem := range oldFeeVault {
		oldFeeVaultRule = append(oldFeeVaultRule, oldFeeVaultItem)
	}
	var newFeeVaultRule []interface{}
	for _, newFeeVaultItem := range newFeeVault {
		newFeeVaultRule = append(newFeeVaultRule, newFeeVaultItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "SetFeeVault", oldFeeVaultRule, newFeeVaultRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewaySetFeeVault)
				if err := _SuccinctGateway.contract.UnpackLog(event, "SetFeeVault", log); err != nil {
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

// ParseSetFeeVault is a log parse operation binding the contract event 0xf0cca8e172b90b70922c6757d918f7a532326dfd3e9f3c5b117a616d2bb07212.
//
// Solidity: event SetFeeVault(address indexed oldFeeVault, address indexed newFeeVault)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseSetFeeVault(log types.Log) (*SuccinctGatewaySetFeeVault, error) {
	event := new(SuccinctGatewaySetFeeVault)
	if err := _SuccinctGateway.contract.UnpackLog(event, "SetFeeVault", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctGatewayWhitelistStatusUpdatedIterator is returned from FilterWhitelistStatusUpdated and is used to iterate over the raw logs and unpacked data for WhitelistStatusUpdated events raised by the SuccinctGateway contract.
type SuccinctGatewayWhitelistStatusUpdatedIterator struct {
	Event *SuccinctGatewayWhitelistStatusUpdated // Event containing the contract specifics and raw log

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
func (it *SuccinctGatewayWhitelistStatusUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctGatewayWhitelistStatusUpdated)
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
		it.Event = new(SuccinctGatewayWhitelistStatusUpdated)
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
func (it *SuccinctGatewayWhitelistStatusUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctGatewayWhitelistStatusUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctGatewayWhitelistStatusUpdated represents a WhitelistStatusUpdated event raised by the SuccinctGateway contract.
type SuccinctGatewayWhitelistStatusUpdated struct {
	FunctionId [32]byte
	Status     uint8
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterWhitelistStatusUpdated is a free log retrieval operation binding the contract event 0x1cfd8c214b5ba0c1306317bd132fa41ca741da227af86a75e9010708561be80e.
//
// Solidity: event WhitelistStatusUpdated(bytes32 indexed functionId, uint8 status)
func (_SuccinctGateway *SuccinctGatewayFilterer) FilterWhitelistStatusUpdated(opts *bind.FilterOpts, functionId [][32]byte) (*SuccinctGatewayWhitelistStatusUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.FilterLogs(opts, "WhitelistStatusUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctGatewayWhitelistStatusUpdatedIterator{contract: _SuccinctGateway.contract, event: "WhitelistStatusUpdated", logs: logs, sub: sub}, nil
}

// WatchWhitelistStatusUpdated is a free log subscription operation binding the contract event 0x1cfd8c214b5ba0c1306317bd132fa41ca741da227af86a75e9010708561be80e.
//
// Solidity: event WhitelistStatusUpdated(bytes32 indexed functionId, uint8 status)
func (_SuccinctGateway *SuccinctGatewayFilterer) WatchWhitelistStatusUpdated(opts *bind.WatchOpts, sink chan<- *SuccinctGatewayWhitelistStatusUpdated, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _SuccinctGateway.contract.WatchLogs(opts, "WhitelistStatusUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctGatewayWhitelistStatusUpdated)
				if err := _SuccinctGateway.contract.UnpackLog(event, "WhitelistStatusUpdated", log); err != nil {
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

// ParseWhitelistStatusUpdated is a log parse operation binding the contract event 0x1cfd8c214b5ba0c1306317bd132fa41ca741da227af86a75e9010708561be80e.
//
// Solidity: event WhitelistStatusUpdated(bytes32 indexed functionId, uint8 status)
func (_SuccinctGateway *SuccinctGatewayFilterer) ParseWhitelistStatusUpdated(log types.Log) (*SuccinctGatewayWhitelistStatusUpdated, error) {
	event := new(SuccinctGatewayWhitelistStatusUpdated)
	if err := _SuccinctGateway.contract.UnpackLog(event, "WhitelistStatusUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

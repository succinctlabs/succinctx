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
	ABI: "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"CallbackAlreadyFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"callbackAddress\",\"type\":\"address\"},{\"internalType\":\"bytes4\",\"name\":\"callbackSelector\",\"type\":\"bytes4\"}],\"name\":\"CallbackFailed\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"contextHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"ContextMismatch\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"EmptyBytecode\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"FailedDeploy\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"name\":\"FunctionAlreadyRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"inputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"InputsRootMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expected\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"actual\",\"type\":\"uint256\"}],\"name\":\"InsufficientFeeAmount\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"inputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"name\":\"InvalidProof\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expected\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"actual\",\"type\":\"uint256\"}],\"name\":\"LengthMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"actualOwner\",\"type\":\"address\"}],\"name\":\"NotFunctionOwner\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"OnlyGuardian\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"OnlyTimelock\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"OutputMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"OutputsRootMismatch\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"ProofAlreadyFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"ProofNotFulfilled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refundAccount\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"refundAmount\",\"type\":\"uint256\"}],\"name\":\"RefundFailed\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"name\":\"RequestNotFound\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"}],\"name\":\"VerificationKeysRootMismatch\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"VerifierCannotBeZero\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"AdminChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\"}],\"name\":\"BeaconUpgraded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"output\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"}],\"name\":\"CallbackFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"bytecodeHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"deployedAddress\",\"type\":\"address\"}],\"name\":\"Deployed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"FunctionOwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"FunctionRegistered\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"name\":\"FunctionVerifierUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\"}],\"name\":\"Initialized\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32[]\",\"name\":\"requestIds\",\"type\":\"bytes32[]\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"aggregateProof\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"inputsRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32[]\",\"name\":\"outputHashes\",\"type\":\"bytes32[]\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"outputsRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"verificationKeyRoot\",\"type\":\"bytes32\"}],\"name\":\"ProofBatchFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"name\":\"ProofFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"inputs\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"name\":\"ProofRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\"}],\"name\":\"RoleAdminChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleGranted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleRevoked\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"scalar\",\"type\":\"uint256\"}],\"name\":\"ScalarUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\"}],\"name\":\"Upgraded\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"AGGREGATION_FUNCTION_ID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DEFAULT_GAS_LIMIT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"GUARDIAN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"TIMELOCK_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\"}],\"name\":\"calculateFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"calculateFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_requestId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_output\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"callback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_bytecode\",\"type\":\"bytes\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"deployAndRegisterFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_bytecode\",\"type\":\"bytes\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"deployAndUpdateFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"feeVault\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_requestId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"}],\"name\":\"fulfill\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_requestIds\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes\",\"name\":\"_aggregateProof\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_inputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32[]\",\"name\":\"_outputHashes\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32\",\"name\":\"_outputsRoot\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_verificationKeyRoot\",\"type\":\"bytes32\"}],\"name\":\"fulfillBatch\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"getFunctionId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"grantRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scalar\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_feeVault\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_timelock\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_guardian\",\"type\":\"address\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"registerFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"renounceRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_input\",\"type\":\"bytes\"},{\"internalType\":\"bytes4\",\"name\":\"_callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"_gasLimit\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_refundAccount\",\"type\":\"address\"}],\"name\":\"request\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_input\",\"type\":\"bytes\"},{\"internalType\":\"bytes4\",\"name\":\"_callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"request\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"requests\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"inputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"outputHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"contextHash\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"callbackAddress\",\"type\":\"address\"},{\"internalType\":\"bytes4\",\"name\":\"callbackSelector\",\"type\":\"bytes4\"},{\"internalType\":\"bool\",\"name\":\"proofFulfilled\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"callbackFulfilled\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"revokeRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"scalar\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"updateFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scalar\",\"type\":\"uint256\"}],\"name\":\"updateScalar\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"}],\"name\":\"upgradeTo\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"upgradeToAndCall\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifierOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifiers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
	Bin: "0x60a0604052306080523480156200001557600080fd5b506200002062000026565b620000e7565b600254610100900460ff1615620000935760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60025460ff90811614620000e5576002805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b608051612ff26200011f600039600081816108b1015281816108f101528181610e6401528181610ea40152610f330152612ff26000f3fe6080604052600436106102045760003560e01c80638b4d7bc411610118578063c30d9826116100a0578063e23b04101161006f578063e23b0410146106bd578063efe1c950146106dd578063f288a2e214610713578063f45e65d814610747578063ffa1ad741461075d57600080fd5b8063c30d98261461065e578063d547741f14610673578063d6be695a14610693578063e2362c31146106aa57600080fd5b80639d866985116100e75780639d8669851461052d578063a217fddf146105f3578063affed0e014610608578063b63755e51461061e578063bd58c4bb1461063e57600080fd5b80638b4d7bc4146104a45780638bcfc3a0146104b757806391d14854146104ed5780639538f56f1461050d57600080fd5b80633bb600391161019b5780635c74ad561161016a5780635c74ad56146103e757806368ff41b114610424578063754d1d541461044457806387c5621a146104645780638ab4be9e1461048457600080fd5b80633bb6003914610352578063478222c2146103865780634f1ef286146103bf57806352d1902d146103d257600080fd5b80632f2ff15d116101d75780632f2ff15d146102d057806336568abe146102f25780633659cfe61461031257806337ea88471461033257600080fd5b806301ffc9a714610209578063178f7b401461023e578063248a9ca31461026c57806324ea54f41461029c575b600080fd5b34801561021557600080fd5b50610229610224366004612519565b610791565b60405190151581526020015b60405180910390f35b34801561024a57600080fd5b5061025e610259366004612534565b6107c8565b604051908152602001610235565b34801561027857600080fd5b5061025e610287366004612534565b600090815260cb602052604090206001015490565b3480156102a857600080fd5b5061025e7f55435dd261a4b9b3364963f7738a7a662ad9c84396d64be3365284bb7f0a504181565b3480156102dc57600080fd5b506102f06102eb366004612564565b6107fa565b005b3480156102fe57600080fd5b506102f061030d366004612564565b610824565b34801561031e57600080fd5b506102f061032d366004612590565b6108a7565b34801561033e57600080fd5b506102f061034d3660046126df565b610986565b34801561035e57600080fd5b5061025e7fcf91d3a65d6f619b1560b4409a7377da358299d073f6633a90fe3313a88b47f581565b34801561039257600080fd5b50610100546103a7906001600160a01b031681565b6040516001600160a01b039091168152602001610235565b6102f06103cd366004612782565b610e5a565b3480156103de57600080fd5b5061025e610f26565b3480156103f357600080fd5b506104076104023660046127cf565b610fd9565b604080519283526001600160a01b03909116602083015201610235565b34801561043057600080fd5b5061025e61043f366004612782565b6110b9565b34801561045057600080fd5b506102f061045f366004612828565b6111a7565b34801561047057600080fd5b506102f061047f366004612875565b6112e0565b34801561049057600080fd5b506102f061049f3660046128c4565b61144d565b61025e6104b2366004612926565b61168a565b3480156104c357600080fd5b506103a76104d2366004612534565b6001602052600090815260409020546001600160a01b031681565b3480156104f957600080fd5b50610229610508366004612564565b611860565b34801561051957600080fd5b5061025e610528366004612782565b61188b565b34801561053957600080fd5b506105a1610548366004612534565b60fe6020526000908152604090208054600182015460028301546003840154600490940154929391929091906001600160a01b03811690600160a01b810460e01b9060ff600160c01b8204811691600160c81b90041688565b6040805198895260208901979097529587019490945260608601929092526001600160a01b031660808501526001600160e01b03191660a0840152151560c0830152151560e082015261010001610235565b3480156105ff57600080fd5b5061025e600081565b34801561061457600080fd5b5061025e60fd5481565b34801561062a57600080fd5b506104076106393660046127cf565b6118be565b34801561064a57600080fd5b5061025e610659366004612782565b61198f565b34801561066a57600080fd5b5061025e611a79565b34801561067f57600080fd5b506102f061068e366004612564565b611a8c565b34801561069f57600080fd5b5061025e620f424081565b61025e6106b83660046129bc565b611ab1565b3480156106c957600080fd5b506102f06106d8366004612534565b611acf565b3480156106e957600080fd5b506103a76106f8366004612534565b6000602081905290815260409020546001600160a01b031681565b34801561071f57600080fd5b5061025e7ff66846415d2bf9eabda9e84793ff9c0ea96d87f50fc41e66aa16469c6a442f0581565b34801561075357600080fd5b5061025e60ff5481565b34801561076957600080fd5b5060408051808201825260058152640312e302e360dc1b602082015290516102359190612a89565b60006001600160e01b03198216637965db0b60e01b14806107c257506301ffc9a760e01b6001600160e01b03198316145b92915050565b600060ff546000036107de576107c2823a612ab2565b60ff546107eb833a612ab2565b6107c29190612ab2565b919050565b600082815260cb602052604090206001015461081581611b53565b61081f8383611b5d565b505050565b6001600160a01b03811633146108995760405162461bcd60e51b815260206004820152602f60248201527f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e636560448201526e103937b632b9903337b91039b2b63360891b60648201526084015b60405180910390fd5b6108a38282611be3565b5050565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036108ef5760405162461bcd60e51b815260040161089090612ac9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610938600080516020612f76833981519152546001600160a01b031690565b6001600160a01b03161461095e5760405162461bcd60e51b815260040161089090612b15565b61096781611c4a565b6040805160008082526020820190925261098391839190611c93565b50565b600086516001600160401b038111156109a1576109a16125ab565b6040519080825280602002602001820160405280156109ca578160200160208202803683370190505b509050600087516001600160401b038111156109e8576109e86125ab565b604051908082528060200260200182016040528015610a11578160200160208202803683370190505b50905060005b8851811015610b82576000898281518110610a3457610a34612b61565b602090810291909101810151600081815260fe90925260409091206004810154919250906001600160a01b0316610a8157604051630d1c383160e11b815260048101839052602401610890565b6004810154600160c01b900460ff1615610ab157604051631aea9acb60e31b815260048101839052602401610890565b8060010154858481518110610ac857610ac8612b61565b60209081029190910181019190915281546000908152808252604090819020548151634f44a2e960e11b815291516001600160a01b03909116928392639e8945d292600480830193928290030181865afa158015610b2a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b4e9190612b77565b858581518110610b6057610b60612b61565b6020026020010181815250505050508080610b7a90612b90565b915050610a17565b508451885114610bb257875185516040516355c5b3e360e11b815260048101929092526024820152604401610890565b81604051602001610bc39190612be4565b604051602081830303815290604052805190602001208614610bfc57858260405163e2920b9160e01b8152600401610890929190612bf7565b84604051602001610c0d9190612be4565b604051602081830303815290604052805190602001208414610c46578385604051637ccf42f360e01b8152600401610890929190612bf7565b80604051602001610c579190612be4565b604051602081830303815290604052805190602001208314610c9057828160405163693d503560e11b8152600401610890929190612bf7565b60005b8851811015610d27576000898281518110610cb057610cb0612b61565b602090810291909101810151600081815260fe9092526040909120600401805460ff60c01b1916600160c01b1790558751909150879083908110610cf657610cf6612b61565b602090810291909101810151600092835260fe90915260409091206002015580610d1f81612b90565b915050610c93565b507fcf91d3a65d6f619b1560b4409a7377da358299d073f6633a90fe3313a88b47f560009081526020527fdfbb683d42ec23abfd9b50088f945b2feb0772147412dcd9441f8e87a3f0ff9e546040516303784b1960e61b81526001600160a01b0390911690819063de12c64090610da6908a9089908d90600401612c10565b6020604051808303816000875af1158015610dc5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610de99190612c2f565b610e0e578087868a6040516316c7141360e31b81526004016108909493929190612c51565b7f9f5bcf5fecad905a6b02f0a6c02a52568005592a0d6c0711752b20ca854e2302898989898989604051610e4796959493929190612c7e565b60405180910390a1505050505050505050565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003610ea25760405162461bcd60e51b815260040161089090612ac9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610eeb600080516020612f76833981519152546001600160a01b031690565b6001600160a01b031614610f115760405162461bcd60e51b815260040161089090612b15565b610f1a82611c4a565b6108a382826001611c93565b6000306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610fc65760405162461bcd60e51b815260206004820152603860248201527f555550535570677261646561626c653a206d757374206e6f742062652063616c60448201527f6c6564207468726f7567682064656c656761746563616c6c00000000000000006064820152608401610890565b50600080516020612f7683398151915290565b600080610fe6338461188b565b6000818152602081905260409020549092506001600160a01b03161561102257604051635e34c78f60e01b815260048101839052602401610890565b600082815260016020526040902080546001600160a01b0319163317905561104a8483611dfe565b6000838152602081905260409081902080546001600160a01b0319166001600160a01b0384161790555190915082907f52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea906110aa90849087903390612cd3565b60405180910390a29250929050565b60006110c5338361188b565b6000818152602081905260409020549091506001600160a01b03161561110157604051635e34c78f60e01b815260048101829052602401610890565b6001600160a01b038316611128576040516302d48d1f60e61b815260040160405180910390fd5b60008181526020818152604080832080546001600160a01b03199081166001600160a01b03891617909155600190925291829020805433921682179055905182917f52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea91611199918791879190612cd3565b60405180910390a292915050565b600254610100900460ff16158080156111c75750600254600160ff909116105b806111e15750303b1580156111e1575060025460ff166001145b6112445760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610890565b6002805460ff191660011790558015611267576002805461ff0019166101001790555b60ff85905561010080546001600160a01b0319166001600160a01b0386161790556112928383611e98565b80156112d9576002805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b5050505050565b600083815260fe6020526040902060048101546001600160a01b031661131c57604051630d1c383160e11b815260048101859052602401610890565b6004810154600160c01b900460ff161561134c57604051631aea9acb60e31b815260048101859052602401610890565b6004808201805460ff60c01b1916600160c01b1790556002820184905581546000908152602081905260409081902054600184015491516303784b1960e61b81526001600160a01b0390911692839263de12c640926113af928991899101612c10565b6020604051808303816000875af11580156113ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113f29190612c2f565b61141a5760018201546040516316c7141360e31b815261089091839187908790600401612c51565b7ffddf097ddc1205e34fd4700d12ad51b32ccad4f117f7ac879a74d20b145209b48585856040516112d093929190612c10565b600083815260fe602052604090206004810154600160c81b900460ff161561148b5760405163b08540e560e01b815260048101859052602401610890565b60048101546001600160a01b03166114b957604051630d1c383160e11b815260048101859052602401610890565b815160208301206003820154146114e75783826040516389116ecd60e01b8152600401610890929190612d08565b82516020840120600282015414611515578383604051633cfc30e360e01b8152600401610890929190612d08565b6004810154600160c01b900460ff1661154457604051635ca8297160e11b815260048101859052602401610890565b60048101805460ff60c81b198116600160c81b17918290556040516000926001600160a01b0390921691600160a01b900460e01b906115899087908790602401612d21565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516115c79190612d4f565b6000604051808303816000865af19150503d8060008114611604576040519150601f19603f3d011682016040523d82523d6000602084013e611609565b606091505b50509050806116575760048281015460405163bc4a234960e01b81526001600160a01b03821692810192909252600160a01b900460e01b6001600160e01b0319166024820152604401610890565b7f4157c302cad5507e9c624680b653ae4a290e304cb0ff86a730bceda763ec878d8585856040516112d093929190612d6b565b845160208087019190912084518583012060408051610100810182528a815293840183905260009084018190526060840182905233608085018190526001600160e01b0319891660a086015260c0850182905260e0850182905290939084906116f7908890889034611f2e565b9050600060fd5483604051602001611710929190612d96565b6040516020818303038152906040528051906020012090508260fe60008381526020019081526020016000206000820151816000015560208201518160010155604082015181600201556060820151816003015560808201518160040160006101000a8154816001600160a01b0302191690836001600160a01b0316021790555060a08201518160040160146101000a81548163ffffffff021916908360e01c021790555060c08201518160040160186101000a81548160ff02191690831515021790555060e08201518160040160196101000a81548160ff0219169083151502179055509050508b60fd547f3fb5c9bd4c90dcd3781879795c37f8645d9421602f4ba57c651f3005938c7260838e8d8d88604051611833959493929190612e14565b60405180910390a360fd805490600061184b83612b90565b90915550909c9b505050505050505050505050565b600091825260cb602090815260408084206001600160a01b0393909316845291905290205460ff1690565b600082826040516020016118a0929190612e54565b60405160208183030381529060405280519060200120905092915050565b6000806118cb338461188b565b6000818152600160205260409020549092506001600160a01b031633146119275760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610890565b6119318483611dfe565b6000838152602081815260409182902080546001600160a01b0319166001600160a01b038516908117909155915191825291925083917ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b73691016110aa565b600061199b338361188b565b6000818152600160205260409020549091506001600160a01b031633146119f75760008181526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610890565b6001600160a01b038316611a1e576040516302d48d1f60e61b815260040160405180910390fd5b6000818152602081815260409182902080546001600160a01b0319166001600160a01b038716908117909155915191825282917ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369101611199565b6000611a87620f42406107c8565b905090565b600082815260cb6020526040902060010154611aa781611b53565b61081f8383611be3565b6000611ac485858585620f42403261168a565b90505b949350505050565b611af97f55435dd261a4b9b3364963f7738a7a662ad9c84396d64be3365284bb7f0a504133611860565b611b185760405163ea79172d60e01b8152336004820152602401610890565b60ff8190556040518181527f3336cd9708eaf2769a0f0dc0679f30e80f15dcd88d1921b5a16858e8b85c591a9060200160405180910390a150565b610983813361208e565b611b678282611860565b6108a357600082815260cb602090815260408083206001600160a01b03851684529091529020805460ff19166001179055611b9f3390565b6001600160a01b0316816001600160a01b0316837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45050565b611bed8282611860565b156108a357600082815260cb602090815260408083206001600160a01b0385168085529252808320805460ff1916905551339285917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a45050565b611c747ff66846415d2bf9eabda9e84793ff9c0ea96d87f50fc41e66aa16469c6a442f0533611860565b61098357604051636744392960e11b8152336004820152602401610890565b7f4910fdfa16fed3260ed0e7147f7cc6da11a60208b5b9406d12a635614ffd91435460ff1615611cc65761081f836120e7565b826001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611d20575060408051601f3d908101601f19168201909252611d1d91810190612b77565b60015b611d835760405162461bcd60e51b815260206004820152602e60248201527f45524331393637557067726164653a206e657720696d706c656d656e7461746960448201526d6f6e206973206e6f74205555505360901b6064820152608401610890565b600080516020612f768339815191528114611df25760405162461bcd60e51b815260206004820152602960248201527f45524331393637557067726164653a20756e737570706f727465642070726f786044820152681a58589b195555525160ba1b6064820152608401610890565b5061081f838383612183565b60008251600003611e22576040516321744a5960e01b815260040160405180910390fd5b818351602085016000f590506001600160a01b038116611e5557604051632081741d60e11b815260040160405180910390fd5b825160208401206040516001600160a01b0383169184917f27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f53290600090a492915050565b600254610100900460ff16611ebf5760405162461bcd60e51b815260040161089090612e78565b611ec76121ae565b611ecf6121ae565b611eda600083611b5d565b611f047ff66846415d2bf9eabda9e84793ff9c0ea96d87f50fc41e66aa16469c6a442f0583611b5d565b6108a37f55435dd261a4b9b3364963f7738a7a662ad9c84396d64be3365284bb7f0a504182611b5d565b6000611f39856107c8565b905080821015611f6657604051630961b65b60e41b81526004810182905260248101839052604401610890565b600081118015611f815750610100546001600160a01b031615155b15611fe957610100546040516333bb7f9160e01b81526001600160a01b038581166004830152909116906333bb7f919083906024016000604051808303818588803b158015611fcf57600080fd5b505af1158015611fe3573d6000803e3d6000fd5b50505050505b6000611ff58284612ec3565b90508015612085576000856001600160a01b03168260405160006040518083038185875af1925050503d806000811461204a576040519150601f19603f3d011682016040523d82523d6000602084013e61204f565b606091505b5050905080612083576040516357b9d85960e11b81526001600160a01b038716600482015260248101839052604401610890565b505b50949350505050565b6120988282611860565b6108a3576120a5816121d7565b6120b08360206121e9565b6040516020016120c1929190612ed6565b60408051601f198184030181529082905262461bcd60e51b825261089091600401612a89565b6001600160a01b0381163b6121545760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b6064820152608401610890565b600080516020612f7683398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b61218c8361238b565b6000825111806121995750805b1561081f576121a883836123cb565b50505050565b600254610100900460ff166121d55760405162461bcd60e51b815260040161089090612e78565b565b60606107c26001600160a01b03831660145b606060006121f8836002612ab2565b612203906002612f4b565b6001600160401b0381111561221a5761221a6125ab565b6040519080825280601f01601f191660200182016040528015612244576020820181803683370190505b509050600360fc1b8160008151811061225f5761225f612b61565b60200101906001600160f81b031916908160001a905350600f60fb1b8160018151811061228e5761228e612b61565b60200101906001600160f81b031916908160001a90535060006122b2846002612ab2565b6122bd906001612f4b565b90505b6001811115612335576f181899199a1a9b1b9c1cb0b131b232b360811b85600f16601081106122f1576122f1612b61565b1a60f81b82828151811061230757612307612b61565b60200101906001600160f81b031916908160001a90535060049490941c9361232e81612f5e565b90506122c0565b5083156123845760405162461bcd60e51b815260206004820181905260248201527f537472696e67733a20686578206c656e67746820696e73756666696369656e746044820152606401610890565b9392505050565b612394816120e7565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606123848383604051806060016040528060278152602001612f96602791396060600080856001600160a01b0316856040516124089190612d4f565b600060405180830381855af49150503d8060008114612443576040519150601f19603f3d011682016040523d82523d6000602084013e612448565b606091505b509150915061245986838387612463565b9695505050505050565b606083156124d25782516000036124cb576001600160a01b0385163b6124cb5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610890565b5081611ac7565b611ac783838151156124e75781518083602001fd5b8060405162461bcd60e51b81526004016108909190612a89565b80356001600160e01b0319811681146107f557600080fd5b60006020828403121561252b57600080fd5b61238482612501565b60006020828403121561254657600080fd5b5035919050565b80356001600160a01b03811681146107f557600080fd5b6000806040838503121561257757600080fd5b823591506125876020840161254d565b90509250929050565b6000602082840312156125a257600080fd5b6123848261254d565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b03811182821017156125e9576125e96125ab565b604052919050565b600082601f83011261260257600080fd5b813560206001600160401b0382111561261d5761261d6125ab565b8160051b61262c8282016125c1565b928352848101820192828101908785111561264657600080fd5b83870192505b848310156126655782358252918301919083019061264c565b979650505050505050565b600082601f83011261268157600080fd5b81356001600160401b0381111561269a5761269a6125ab565b6126ad601f8201601f19166020016125c1565b8181528460208386010111156126c257600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060008060c087890312156126f857600080fd5b86356001600160401b038082111561270f57600080fd5b61271b8a838b016125f1565b9750602089013591508082111561273157600080fd5b61273d8a838b01612670565b965060408901359550606089013591508082111561275a57600080fd5b5061276789828a016125f1565b9350506080870135915060a087013590509295509295509295565b6000806040838503121561279557600080fd5b61279e8361254d565b915060208301356001600160401b038111156127b957600080fd5b6127c585828601612670565b9150509250929050565b600080604083850312156127e257600080fd5b82356001600160401b03808211156127f957600080fd5b61280586838701612670565b9350602085013591508082111561281b57600080fd5b506127c585828601612670565b6000806000806080858703121561283e57600080fd5b8435935061284e6020860161254d565b925061285c6040860161254d565b915061286a6060860161254d565b905092959194509250565b60008060006060848603121561288a57600080fd5b833592506020840135915060408401356001600160401b038111156128ae57600080fd5b6128ba86828701612670565b9150509250925092565b6000806000606084860312156128d957600080fd5b8335925060208401356001600160401b03808211156128f757600080fd5b61290387838801612670565b9350604086013591508082111561291957600080fd5b506128ba86828701612670565b60008060008060008060c0878903121561293f57600080fd5b8635955060208701356001600160401b038082111561295d57600080fd5b6129698a838b01612670565b965061297760408a01612501565b9550606089013591508082111561298d57600080fd5b5061299a89828a01612670565b935050608087013591506129b060a0880161254d565b90509295509295509295565b600080600080608085870312156129d257600080fd5b8435935060208501356001600160401b03808211156129f057600080fd5b6129fc88838901612670565b9450612a0a60408801612501565b93506060870135915080821115612a2057600080fd5b50612a2d87828801612670565b91505092959194509250565b60005b83811015612a54578181015183820152602001612a3c565b50506000910152565b60008151808452612a75816020860160208601612a39565b601f01601f19169290920160200192915050565b6020815260006123846020830184612a5d565b634e487b7160e01b600052601160045260246000fd5b80820281158282048414176107c2576107c2612a9c565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b19195b1959d85d1958d85b1b60a21b606082015260800190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b6163746976652070726f787960a01b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b600060208284031215612b8957600080fd5b5051919050565b600060018201612ba257612ba2612a9c565b5060010190565b600081518084526020808501945080840160005b83811015612bd957815187529582019590820190600101612bbd565b509495945050505050565b6020815260006123846020830184612ba9565b828152604060208201526000611ac76040830184612ba9565b838152826020820152606060408201526000611ac46060830184612a5d565b600060208284031215612c4157600080fd5b8151801515811461238457600080fd5b60018060a01b03851681528360208201528260408201526080606082015260006124596080830184612a5d565b60c081526000612c9160c0830189612ba9565b8281036020840152612ca38189612a5d565b90508660408401528281036060840152612cbd8187612ba9565b6080840195909552505060a00152949350505050565b600060018060a01b03808616835260606020840152612cf56060840186612a5d565b9150808416604084015250949350505050565b828152604060208201526000611ac76040830184612a5d565b604081526000612d346040830185612a5d565b8281036020840152612d468185612a5d565b95945050505050565b60008251612d61818460208701612a39565b9190910192915050565b838152606060208201526000612d846060830185612a5d565b82810360408401526124598185612a5d565b6000610120820190508382528251602083015260208301516040830152604083015160608301526060830151608083015260018060a01b0360808401511660a083015263ffffffff60e01b60a08401511660c083015260c0830151151560e083015260e0830151612e0c61010084018215159052565b509392505050565b85815260a060208201526000612e2d60a0830187612a5d565b8281036040840152612e3f8187612a5d565b60608401959095525050608001529392505050565b6001600160a01b0383168152604060208201819052600090611ac790830184612a5d565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b818103818111156107c2576107c2612a9c565b7f416363657373436f6e74726f6c3a206163636f756e7420000000000000000000815260008351612f0e816017850160208801612a39565b7001034b99036b4b9b9b4b733903937b6329607d1b6017918401918201528351612f3f816028840160208801612a39565b01602801949350505050565b808201808211156107c2576107c2612a9c565b600081612f6d57612f6d612a9c565b50600019019056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220095a84c597212fc7e2eef0b50699b828153bbc7e034f1c71d8e98f57c7962a7d64736f6c63430008140033",
}

// FunctionGatewayABI is the input ABI used to generate the binding from.
// Deprecated: Use FunctionGatewayMetaData.ABI instead.
var FunctionGatewayABI = FunctionGatewayMetaData.ABI

// FunctionGatewayBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use FunctionGatewayMetaData.Bin instead.
var FunctionGatewayBin = FunctionGatewayMetaData.Bin

// DeployFunctionGateway deploys a new Ethereum contract, binding an instance of FunctionGateway to it.
func DeployFunctionGateway(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *FunctionGateway, error) {
	parsed, err := FunctionGatewayMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(FunctionGatewayBin), backend)
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

// DEFAULTADMINROLE is a free data retrieval call binding the contract method 0xa217fddf.
//
// Solidity: function DEFAULT_ADMIN_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCaller) DEFAULTADMINROLE(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "DEFAULT_ADMIN_ROLE")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// DEFAULTADMINROLE is a free data retrieval call binding the contract method 0xa217fddf.
//
// Solidity: function DEFAULT_ADMIN_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) DEFAULTADMINROLE() ([32]byte, error) {
	return _FunctionGateway.Contract.DEFAULTADMINROLE(&_FunctionGateway.CallOpts)
}

// DEFAULTADMINROLE is a free data retrieval call binding the contract method 0xa217fddf.
//
// Solidity: function DEFAULT_ADMIN_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCallerSession) DEFAULTADMINROLE() ([32]byte, error) {
	return _FunctionGateway.Contract.DEFAULTADMINROLE(&_FunctionGateway.CallOpts)
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

// GUARDIANROLE is a free data retrieval call binding the contract method 0x24ea54f4.
//
// Solidity: function GUARDIAN_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCaller) GUARDIANROLE(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "GUARDIAN_ROLE")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GUARDIANROLE is a free data retrieval call binding the contract method 0x24ea54f4.
//
// Solidity: function GUARDIAN_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) GUARDIANROLE() ([32]byte, error) {
	return _FunctionGateway.Contract.GUARDIANROLE(&_FunctionGateway.CallOpts)
}

// GUARDIANROLE is a free data retrieval call binding the contract method 0x24ea54f4.
//
// Solidity: function GUARDIAN_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCallerSession) GUARDIANROLE() ([32]byte, error) {
	return _FunctionGateway.Contract.GUARDIANROLE(&_FunctionGateway.CallOpts)
}

// TIMELOCKROLE is a free data retrieval call binding the contract method 0xf288a2e2.
//
// Solidity: function TIMELOCK_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCaller) TIMELOCKROLE(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "TIMELOCK_ROLE")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// TIMELOCKROLE is a free data retrieval call binding the contract method 0xf288a2e2.
//
// Solidity: function TIMELOCK_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) TIMELOCKROLE() ([32]byte, error) {
	return _FunctionGateway.Contract.TIMELOCKROLE(&_FunctionGateway.CallOpts)
}

// TIMELOCKROLE is a free data retrieval call binding the contract method 0xf288a2e2.
//
// Solidity: function TIMELOCK_ROLE() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCallerSession) TIMELOCKROLE() ([32]byte, error) {
	return _FunctionGateway.Contract.TIMELOCKROLE(&_FunctionGateway.CallOpts)
}

// VERSION is a free data retrieval call binding the contract method 0xffa1ad74.
//
// Solidity: function VERSION() pure returns(string)
func (_FunctionGateway *FunctionGatewayCaller) VERSION(opts *bind.CallOpts) (string, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "VERSION")

	if err != nil {
		return *new(string), err
	}

	out0 := *abi.ConvertType(out[0], new(string)).(*string)

	return out0, err

}

// VERSION is a free data retrieval call binding the contract method 0xffa1ad74.
//
// Solidity: function VERSION() pure returns(string)
func (_FunctionGateway *FunctionGatewaySession) VERSION() (string, error) {
	return _FunctionGateway.Contract.VERSION(&_FunctionGateway.CallOpts)
}

// VERSION is a free data retrieval call binding the contract method 0xffa1ad74.
//
// Solidity: function VERSION() pure returns(string)
func (_FunctionGateway *FunctionGatewayCallerSession) VERSION() (string, error) {
	return _FunctionGateway.Contract.VERSION(&_FunctionGateway.CallOpts)
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

// GetRoleAdmin is a free data retrieval call binding the contract method 0x248a9ca3.
//
// Solidity: function getRoleAdmin(bytes32 role) view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCaller) GetRoleAdmin(opts *bind.CallOpts, role [32]byte) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "getRoleAdmin", role)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetRoleAdmin is a free data retrieval call binding the contract method 0x248a9ca3.
//
// Solidity: function getRoleAdmin(bytes32 role) view returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) GetRoleAdmin(role [32]byte) ([32]byte, error) {
	return _FunctionGateway.Contract.GetRoleAdmin(&_FunctionGateway.CallOpts, role)
}

// GetRoleAdmin is a free data retrieval call binding the contract method 0x248a9ca3.
//
// Solidity: function getRoleAdmin(bytes32 role) view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCallerSession) GetRoleAdmin(role [32]byte) ([32]byte, error) {
	return _FunctionGateway.Contract.GetRoleAdmin(&_FunctionGateway.CallOpts, role)
}

// HasRole is a free data retrieval call binding the contract method 0x91d14854.
//
// Solidity: function hasRole(bytes32 role, address account) view returns(bool)
func (_FunctionGateway *FunctionGatewayCaller) HasRole(opts *bind.CallOpts, role [32]byte, account common.Address) (bool, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "hasRole", role, account)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// HasRole is a free data retrieval call binding the contract method 0x91d14854.
//
// Solidity: function hasRole(bytes32 role, address account) view returns(bool)
func (_FunctionGateway *FunctionGatewaySession) HasRole(role [32]byte, account common.Address) (bool, error) {
	return _FunctionGateway.Contract.HasRole(&_FunctionGateway.CallOpts, role, account)
}

// HasRole is a free data retrieval call binding the contract method 0x91d14854.
//
// Solidity: function hasRole(bytes32 role, address account) view returns(bool)
func (_FunctionGateway *FunctionGatewayCallerSession) HasRole(role [32]byte, account common.Address) (bool, error) {
	return _FunctionGateway.Contract.HasRole(&_FunctionGateway.CallOpts, role, account)
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

// ProxiableUUID is a free data retrieval call binding the contract method 0x52d1902d.
//
// Solidity: function proxiableUUID() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCaller) ProxiableUUID(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "proxiableUUID")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// ProxiableUUID is a free data retrieval call binding the contract method 0x52d1902d.
//
// Solidity: function proxiableUUID() view returns(bytes32)
func (_FunctionGateway *FunctionGatewaySession) ProxiableUUID() ([32]byte, error) {
	return _FunctionGateway.Contract.ProxiableUUID(&_FunctionGateway.CallOpts)
}

// ProxiableUUID is a free data retrieval call binding the contract method 0x52d1902d.
//
// Solidity: function proxiableUUID() view returns(bytes32)
func (_FunctionGateway *FunctionGatewayCallerSession) ProxiableUUID() ([32]byte, error) {
	return _FunctionGateway.Contract.ProxiableUUID(&_FunctionGateway.CallOpts)
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

// SupportsInterface is a free data retrieval call binding the contract method 0x01ffc9a7.
//
// Solidity: function supportsInterface(bytes4 interfaceId) view returns(bool)
func (_FunctionGateway *FunctionGatewayCaller) SupportsInterface(opts *bind.CallOpts, interfaceId [4]byte) (bool, error) {
	var out []interface{}
	err := _FunctionGateway.contract.Call(opts, &out, "supportsInterface", interfaceId)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// SupportsInterface is a free data retrieval call binding the contract method 0x01ffc9a7.
//
// Solidity: function supportsInterface(bytes4 interfaceId) view returns(bool)
func (_FunctionGateway *FunctionGatewaySession) SupportsInterface(interfaceId [4]byte) (bool, error) {
	return _FunctionGateway.Contract.SupportsInterface(&_FunctionGateway.CallOpts, interfaceId)
}

// SupportsInterface is a free data retrieval call binding the contract method 0x01ffc9a7.
//
// Solidity: function supportsInterface(bytes4 interfaceId) view returns(bool)
func (_FunctionGateway *FunctionGatewayCallerSession) SupportsInterface(interfaceId [4]byte) (bool, error) {
	return _FunctionGateway.Contract.SupportsInterface(&_FunctionGateway.CallOpts, interfaceId)
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

// GrantRole is a paid mutator transaction binding the contract method 0x2f2ff15d.
//
// Solidity: function grantRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewayTransactor) GrantRole(opts *bind.TransactOpts, role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "grantRole", role, account)
}

// GrantRole is a paid mutator transaction binding the contract method 0x2f2ff15d.
//
// Solidity: function grantRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewaySession) GrantRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.GrantRole(&_FunctionGateway.TransactOpts, role, account)
}

// GrantRole is a paid mutator transaction binding the contract method 0x2f2ff15d.
//
// Solidity: function grantRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) GrantRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.GrantRole(&_FunctionGateway.TransactOpts, role, account)
}

// Initialize is a paid mutator transaction binding the contract method 0x754d1d54.
//
// Solidity: function initialize(uint256 _scalar, address _feeVault, address _timelock, address _guardian) returns()
func (_FunctionGateway *FunctionGatewayTransactor) Initialize(opts *bind.TransactOpts, _scalar *big.Int, _feeVault common.Address, _timelock common.Address, _guardian common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "initialize", _scalar, _feeVault, _timelock, _guardian)
}

// Initialize is a paid mutator transaction binding the contract method 0x754d1d54.
//
// Solidity: function initialize(uint256 _scalar, address _feeVault, address _timelock, address _guardian) returns()
func (_FunctionGateway *FunctionGatewaySession) Initialize(_scalar *big.Int, _feeVault common.Address, _timelock common.Address, _guardian common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Initialize(&_FunctionGateway.TransactOpts, _scalar, _feeVault, _timelock, _guardian)
}

// Initialize is a paid mutator transaction binding the contract method 0x754d1d54.
//
// Solidity: function initialize(uint256 _scalar, address _feeVault, address _timelock, address _guardian) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) Initialize(_scalar *big.Int, _feeVault common.Address, _timelock common.Address, _guardian common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.Initialize(&_FunctionGateway.TransactOpts, _scalar, _feeVault, _timelock, _guardian)
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

// RenounceRole is a paid mutator transaction binding the contract method 0x36568abe.
//
// Solidity: function renounceRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewayTransactor) RenounceRole(opts *bind.TransactOpts, role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "renounceRole", role, account)
}

// RenounceRole is a paid mutator transaction binding the contract method 0x36568abe.
//
// Solidity: function renounceRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewaySession) RenounceRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RenounceRole(&_FunctionGateway.TransactOpts, role, account)
}

// RenounceRole is a paid mutator transaction binding the contract method 0x36568abe.
//
// Solidity: function renounceRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) RenounceRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RenounceRole(&_FunctionGateway.TransactOpts, role, account)
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

// RevokeRole is a paid mutator transaction binding the contract method 0xd547741f.
//
// Solidity: function revokeRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewayTransactor) RevokeRole(opts *bind.TransactOpts, role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "revokeRole", role, account)
}

// RevokeRole is a paid mutator transaction binding the contract method 0xd547741f.
//
// Solidity: function revokeRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewaySession) RevokeRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RevokeRole(&_FunctionGateway.TransactOpts, role, account)
}

// RevokeRole is a paid mutator transaction binding the contract method 0xd547741f.
//
// Solidity: function revokeRole(bytes32 role, address account) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) RevokeRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.RevokeRole(&_FunctionGateway.TransactOpts, role, account)
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

// UpgradeTo is a paid mutator transaction binding the contract method 0x3659cfe6.
//
// Solidity: function upgradeTo(address newImplementation) returns()
func (_FunctionGateway *FunctionGatewayTransactor) UpgradeTo(opts *bind.TransactOpts, newImplementation common.Address) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "upgradeTo", newImplementation)
}

// UpgradeTo is a paid mutator transaction binding the contract method 0x3659cfe6.
//
// Solidity: function upgradeTo(address newImplementation) returns()
func (_FunctionGateway *FunctionGatewaySession) UpgradeTo(newImplementation common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpgradeTo(&_FunctionGateway.TransactOpts, newImplementation)
}

// UpgradeTo is a paid mutator transaction binding the contract method 0x3659cfe6.
//
// Solidity: function upgradeTo(address newImplementation) returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) UpgradeTo(newImplementation common.Address) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpgradeTo(&_FunctionGateway.TransactOpts, newImplementation)
}

// UpgradeToAndCall is a paid mutator transaction binding the contract method 0x4f1ef286.
//
// Solidity: function upgradeToAndCall(address newImplementation, bytes data) payable returns()
func (_FunctionGateway *FunctionGatewayTransactor) UpgradeToAndCall(opts *bind.TransactOpts, newImplementation common.Address, data []byte) (*types.Transaction, error) {
	return _FunctionGateway.contract.Transact(opts, "upgradeToAndCall", newImplementation, data)
}

// UpgradeToAndCall is a paid mutator transaction binding the contract method 0x4f1ef286.
//
// Solidity: function upgradeToAndCall(address newImplementation, bytes data) payable returns()
func (_FunctionGateway *FunctionGatewaySession) UpgradeToAndCall(newImplementation common.Address, data []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpgradeToAndCall(&_FunctionGateway.TransactOpts, newImplementation, data)
}

// UpgradeToAndCall is a paid mutator transaction binding the contract method 0x4f1ef286.
//
// Solidity: function upgradeToAndCall(address newImplementation, bytes data) payable returns()
func (_FunctionGateway *FunctionGatewayTransactorSession) UpgradeToAndCall(newImplementation common.Address, data []byte) (*types.Transaction, error) {
	return _FunctionGateway.Contract.UpgradeToAndCall(&_FunctionGateway.TransactOpts, newImplementation, data)
}

// FunctionGatewayAdminChangedIterator is returned from FilterAdminChanged and is used to iterate over the raw logs and unpacked data for AdminChanged events raised by the FunctionGateway contract.
type FunctionGatewayAdminChangedIterator struct {
	Event *FunctionGatewayAdminChanged // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayAdminChangedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayAdminChanged)
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
		it.Event = new(FunctionGatewayAdminChanged)
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
func (it *FunctionGatewayAdminChangedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayAdminChangedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayAdminChanged represents a AdminChanged event raised by the FunctionGateway contract.
type FunctionGatewayAdminChanged struct {
	PreviousAdmin common.Address
	NewAdmin      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterAdminChanged is a free log retrieval operation binding the contract event 0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f.
//
// Solidity: event AdminChanged(address previousAdmin, address newAdmin)
func (_FunctionGateway *FunctionGatewayFilterer) FilterAdminChanged(opts *bind.FilterOpts) (*FunctionGatewayAdminChangedIterator, error) {

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "AdminChanged")
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayAdminChangedIterator{contract: _FunctionGateway.contract, event: "AdminChanged", logs: logs, sub: sub}, nil
}

// WatchAdminChanged is a free log subscription operation binding the contract event 0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f.
//
// Solidity: event AdminChanged(address previousAdmin, address newAdmin)
func (_FunctionGateway *FunctionGatewayFilterer) WatchAdminChanged(opts *bind.WatchOpts, sink chan<- *FunctionGatewayAdminChanged) (event.Subscription, error) {

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "AdminChanged")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayAdminChanged)
				if err := _FunctionGateway.contract.UnpackLog(event, "AdminChanged", log); err != nil {
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

// ParseAdminChanged is a log parse operation binding the contract event 0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f.
//
// Solidity: event AdminChanged(address previousAdmin, address newAdmin)
func (_FunctionGateway *FunctionGatewayFilterer) ParseAdminChanged(log types.Log) (*FunctionGatewayAdminChanged, error) {
	event := new(FunctionGatewayAdminChanged)
	if err := _FunctionGateway.contract.UnpackLog(event, "AdminChanged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayBeaconUpgradedIterator is returned from FilterBeaconUpgraded and is used to iterate over the raw logs and unpacked data for BeaconUpgraded events raised by the FunctionGateway contract.
type FunctionGatewayBeaconUpgradedIterator struct {
	Event *FunctionGatewayBeaconUpgraded // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayBeaconUpgradedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayBeaconUpgraded)
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
		it.Event = new(FunctionGatewayBeaconUpgraded)
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
func (it *FunctionGatewayBeaconUpgradedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayBeaconUpgradedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayBeaconUpgraded represents a BeaconUpgraded event raised by the FunctionGateway contract.
type FunctionGatewayBeaconUpgraded struct {
	Beacon common.Address
	Raw    types.Log // Blockchain specific contextual infos
}

// FilterBeaconUpgraded is a free log retrieval operation binding the contract event 0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e.
//
// Solidity: event BeaconUpgraded(address indexed beacon)
func (_FunctionGateway *FunctionGatewayFilterer) FilterBeaconUpgraded(opts *bind.FilterOpts, beacon []common.Address) (*FunctionGatewayBeaconUpgradedIterator, error) {

	var beaconRule []interface{}
	for _, beaconItem := range beacon {
		beaconRule = append(beaconRule, beaconItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "BeaconUpgraded", beaconRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayBeaconUpgradedIterator{contract: _FunctionGateway.contract, event: "BeaconUpgraded", logs: logs, sub: sub}, nil
}

// WatchBeaconUpgraded is a free log subscription operation binding the contract event 0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e.
//
// Solidity: event BeaconUpgraded(address indexed beacon)
func (_FunctionGateway *FunctionGatewayFilterer) WatchBeaconUpgraded(opts *bind.WatchOpts, sink chan<- *FunctionGatewayBeaconUpgraded, beacon []common.Address) (event.Subscription, error) {

	var beaconRule []interface{}
	for _, beaconItem := range beacon {
		beaconRule = append(beaconRule, beaconItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "BeaconUpgraded", beaconRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayBeaconUpgraded)
				if err := _FunctionGateway.contract.UnpackLog(event, "BeaconUpgraded", log); err != nil {
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

// ParseBeaconUpgraded is a log parse operation binding the contract event 0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e.
//
// Solidity: event BeaconUpgraded(address indexed beacon)
func (_FunctionGateway *FunctionGatewayFilterer) ParseBeaconUpgraded(log types.Log) (*FunctionGatewayBeaconUpgraded, error) {
	event := new(FunctionGatewayBeaconUpgraded)
	if err := _FunctionGateway.contract.UnpackLog(event, "BeaconUpgraded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
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

// FunctionGatewayInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the FunctionGateway contract.
type FunctionGatewayInitializedIterator struct {
	Event *FunctionGatewayInitialized // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayInitialized)
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
		it.Event = new(FunctionGatewayInitialized)
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
func (it *FunctionGatewayInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayInitialized represents a Initialized event raised by the FunctionGateway contract.
type FunctionGatewayInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_FunctionGateway *FunctionGatewayFilterer) FilterInitialized(opts *bind.FilterOpts) (*FunctionGatewayInitializedIterator, error) {

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayInitializedIterator{contract: _FunctionGateway.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_FunctionGateway *FunctionGatewayFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *FunctionGatewayInitialized) (event.Subscription, error) {

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayInitialized)
				if err := _FunctionGateway.contract.UnpackLog(event, "Initialized", log); err != nil {
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
func (_FunctionGateway *FunctionGatewayFilterer) ParseInitialized(log types.Log) (*FunctionGatewayInitialized, error) {
	event := new(FunctionGatewayInitialized)
	if err := _FunctionGateway.contract.UnpackLog(event, "Initialized", log); err != nil {
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

// FunctionGatewayRoleAdminChangedIterator is returned from FilterRoleAdminChanged and is used to iterate over the raw logs and unpacked data for RoleAdminChanged events raised by the FunctionGateway contract.
type FunctionGatewayRoleAdminChangedIterator struct {
	Event *FunctionGatewayRoleAdminChanged // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayRoleAdminChangedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayRoleAdminChanged)
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
		it.Event = new(FunctionGatewayRoleAdminChanged)
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
func (it *FunctionGatewayRoleAdminChangedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayRoleAdminChangedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayRoleAdminChanged represents a RoleAdminChanged event raised by the FunctionGateway contract.
type FunctionGatewayRoleAdminChanged struct {
	Role              [32]byte
	PreviousAdminRole [32]byte
	NewAdminRole      [32]byte
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterRoleAdminChanged is a free log retrieval operation binding the contract event 0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff.
//
// Solidity: event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole)
func (_FunctionGateway *FunctionGatewayFilterer) FilterRoleAdminChanged(opts *bind.FilterOpts, role [][32]byte, previousAdminRole [][32]byte, newAdminRole [][32]byte) (*FunctionGatewayRoleAdminChangedIterator, error) {

	var roleRule []interface{}
	for _, roleItem := range role {
		roleRule = append(roleRule, roleItem)
	}
	var previousAdminRoleRule []interface{}
	for _, previousAdminRoleItem := range previousAdminRole {
		previousAdminRoleRule = append(previousAdminRoleRule, previousAdminRoleItem)
	}
	var newAdminRoleRule []interface{}
	for _, newAdminRoleItem := range newAdminRole {
		newAdminRoleRule = append(newAdminRoleRule, newAdminRoleItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "RoleAdminChanged", roleRule, previousAdminRoleRule, newAdminRoleRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayRoleAdminChangedIterator{contract: _FunctionGateway.contract, event: "RoleAdminChanged", logs: logs, sub: sub}, nil
}

// WatchRoleAdminChanged is a free log subscription operation binding the contract event 0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff.
//
// Solidity: event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole)
func (_FunctionGateway *FunctionGatewayFilterer) WatchRoleAdminChanged(opts *bind.WatchOpts, sink chan<- *FunctionGatewayRoleAdminChanged, role [][32]byte, previousAdminRole [][32]byte, newAdminRole [][32]byte) (event.Subscription, error) {

	var roleRule []interface{}
	for _, roleItem := range role {
		roleRule = append(roleRule, roleItem)
	}
	var previousAdminRoleRule []interface{}
	for _, previousAdminRoleItem := range previousAdminRole {
		previousAdminRoleRule = append(previousAdminRoleRule, previousAdminRoleItem)
	}
	var newAdminRoleRule []interface{}
	for _, newAdminRoleItem := range newAdminRole {
		newAdminRoleRule = append(newAdminRoleRule, newAdminRoleItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "RoleAdminChanged", roleRule, previousAdminRoleRule, newAdminRoleRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayRoleAdminChanged)
				if err := _FunctionGateway.contract.UnpackLog(event, "RoleAdminChanged", log); err != nil {
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

// ParseRoleAdminChanged is a log parse operation binding the contract event 0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff.
//
// Solidity: event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole)
func (_FunctionGateway *FunctionGatewayFilterer) ParseRoleAdminChanged(log types.Log) (*FunctionGatewayRoleAdminChanged, error) {
	event := new(FunctionGatewayRoleAdminChanged)
	if err := _FunctionGateway.contract.UnpackLog(event, "RoleAdminChanged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayRoleGrantedIterator is returned from FilterRoleGranted and is used to iterate over the raw logs and unpacked data for RoleGranted events raised by the FunctionGateway contract.
type FunctionGatewayRoleGrantedIterator struct {
	Event *FunctionGatewayRoleGranted // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayRoleGrantedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayRoleGranted)
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
		it.Event = new(FunctionGatewayRoleGranted)
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
func (it *FunctionGatewayRoleGrantedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayRoleGrantedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayRoleGranted represents a RoleGranted event raised by the FunctionGateway contract.
type FunctionGatewayRoleGranted struct {
	Role    [32]byte
	Account common.Address
	Sender  common.Address
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterRoleGranted is a free log retrieval operation binding the contract event 0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d.
//
// Solidity: event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)
func (_FunctionGateway *FunctionGatewayFilterer) FilterRoleGranted(opts *bind.FilterOpts, role [][32]byte, account []common.Address, sender []common.Address) (*FunctionGatewayRoleGrantedIterator, error) {

	var roleRule []interface{}
	for _, roleItem := range role {
		roleRule = append(roleRule, roleItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var senderRule []interface{}
	for _, senderItem := range sender {
		senderRule = append(senderRule, senderItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "RoleGranted", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayRoleGrantedIterator{contract: _FunctionGateway.contract, event: "RoleGranted", logs: logs, sub: sub}, nil
}

// WatchRoleGranted is a free log subscription operation binding the contract event 0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d.
//
// Solidity: event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)
func (_FunctionGateway *FunctionGatewayFilterer) WatchRoleGranted(opts *bind.WatchOpts, sink chan<- *FunctionGatewayRoleGranted, role [][32]byte, account []common.Address, sender []common.Address) (event.Subscription, error) {

	var roleRule []interface{}
	for _, roleItem := range role {
		roleRule = append(roleRule, roleItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var senderRule []interface{}
	for _, senderItem := range sender {
		senderRule = append(senderRule, senderItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "RoleGranted", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayRoleGranted)
				if err := _FunctionGateway.contract.UnpackLog(event, "RoleGranted", log); err != nil {
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

// ParseRoleGranted is a log parse operation binding the contract event 0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d.
//
// Solidity: event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)
func (_FunctionGateway *FunctionGatewayFilterer) ParseRoleGranted(log types.Log) (*FunctionGatewayRoleGranted, error) {
	event := new(FunctionGatewayRoleGranted)
	if err := _FunctionGateway.contract.UnpackLog(event, "RoleGranted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionGatewayRoleRevokedIterator is returned from FilterRoleRevoked and is used to iterate over the raw logs and unpacked data for RoleRevoked events raised by the FunctionGateway contract.
type FunctionGatewayRoleRevokedIterator struct {
	Event *FunctionGatewayRoleRevoked // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayRoleRevokedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayRoleRevoked)
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
		it.Event = new(FunctionGatewayRoleRevoked)
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
func (it *FunctionGatewayRoleRevokedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayRoleRevokedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayRoleRevoked represents a RoleRevoked event raised by the FunctionGateway contract.
type FunctionGatewayRoleRevoked struct {
	Role    [32]byte
	Account common.Address
	Sender  common.Address
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterRoleRevoked is a free log retrieval operation binding the contract event 0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b.
//
// Solidity: event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)
func (_FunctionGateway *FunctionGatewayFilterer) FilterRoleRevoked(opts *bind.FilterOpts, role [][32]byte, account []common.Address, sender []common.Address) (*FunctionGatewayRoleRevokedIterator, error) {

	var roleRule []interface{}
	for _, roleItem := range role {
		roleRule = append(roleRule, roleItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var senderRule []interface{}
	for _, senderItem := range sender {
		senderRule = append(senderRule, senderItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "RoleRevoked", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayRoleRevokedIterator{contract: _FunctionGateway.contract, event: "RoleRevoked", logs: logs, sub: sub}, nil
}

// WatchRoleRevoked is a free log subscription operation binding the contract event 0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b.
//
// Solidity: event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)
func (_FunctionGateway *FunctionGatewayFilterer) WatchRoleRevoked(opts *bind.WatchOpts, sink chan<- *FunctionGatewayRoleRevoked, role [][32]byte, account []common.Address, sender []common.Address) (event.Subscription, error) {

	var roleRule []interface{}
	for _, roleItem := range role {
		roleRule = append(roleRule, roleItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var senderRule []interface{}
	for _, senderItem := range sender {
		senderRule = append(senderRule, senderItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "RoleRevoked", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayRoleRevoked)
				if err := _FunctionGateway.contract.UnpackLog(event, "RoleRevoked", log); err != nil {
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

// ParseRoleRevoked is a log parse operation binding the contract event 0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b.
//
// Solidity: event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)
func (_FunctionGateway *FunctionGatewayFilterer) ParseRoleRevoked(log types.Log) (*FunctionGatewayRoleRevoked, error) {
	event := new(FunctionGatewayRoleRevoked)
	if err := _FunctionGateway.contract.UnpackLog(event, "RoleRevoked", log); err != nil {
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

// FunctionGatewayUpgradedIterator is returned from FilterUpgraded and is used to iterate over the raw logs and unpacked data for Upgraded events raised by the FunctionGateway contract.
type FunctionGatewayUpgradedIterator struct {
	Event *FunctionGatewayUpgraded // Event containing the contract specifics and raw log

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
func (it *FunctionGatewayUpgradedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionGatewayUpgraded)
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
		it.Event = new(FunctionGatewayUpgraded)
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
func (it *FunctionGatewayUpgradedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionGatewayUpgradedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionGatewayUpgraded represents a Upgraded event raised by the FunctionGateway contract.
type FunctionGatewayUpgraded struct {
	Implementation common.Address
	Raw            types.Log // Blockchain specific contextual infos
}

// FilterUpgraded is a free log retrieval operation binding the contract event 0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b.
//
// Solidity: event Upgraded(address indexed implementation)
func (_FunctionGateway *FunctionGatewayFilterer) FilterUpgraded(opts *bind.FilterOpts, implementation []common.Address) (*FunctionGatewayUpgradedIterator, error) {

	var implementationRule []interface{}
	for _, implementationItem := range implementation {
		implementationRule = append(implementationRule, implementationItem)
	}

	logs, sub, err := _FunctionGateway.contract.FilterLogs(opts, "Upgraded", implementationRule)
	if err != nil {
		return nil, err
	}
	return &FunctionGatewayUpgradedIterator{contract: _FunctionGateway.contract, event: "Upgraded", logs: logs, sub: sub}, nil
}

// WatchUpgraded is a free log subscription operation binding the contract event 0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b.
//
// Solidity: event Upgraded(address indexed implementation)
func (_FunctionGateway *FunctionGatewayFilterer) WatchUpgraded(opts *bind.WatchOpts, sink chan<- *FunctionGatewayUpgraded, implementation []common.Address) (event.Subscription, error) {

	var implementationRule []interface{}
	for _, implementationItem := range implementation {
		implementationRule = append(implementationRule, implementationItem)
	}

	logs, sub, err := _FunctionGateway.contract.WatchLogs(opts, "Upgraded", implementationRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionGatewayUpgraded)
				if err := _FunctionGateway.contract.UnpackLog(event, "Upgraded", log); err != nil {
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

// ParseUpgraded is a log parse operation binding the contract event 0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b.
//
// Solidity: event Upgraded(address indexed implementation)
func (_FunctionGateway *FunctionGatewayFilterer) ParseUpgraded(log types.Log) (*FunctionGatewayUpgraded, error) {
	event := new(FunctionGatewayUpgraded)
	if err := _FunctionGateway.contract.UnpackLog(event, "Upgraded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

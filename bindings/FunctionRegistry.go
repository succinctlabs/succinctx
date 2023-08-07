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

// FunctionRegistryMetaData contains all meta data concerning the FunctionRegistry contract.
var FunctionRegistryMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"proofId\",\"type\":\"bytes32\"}],\"name\":\"FunctionAlreadyRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"proofId\",\"type\":\"bytes32\"}],\"name\":\"FunctionNotRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"actualOwner\",\"type\":\"address\"}],\"name\":\"NotFunctionOwner\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"registerFunction\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"updateFunctionOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"}],\"name\":\"updateFunctionVerifier\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifierOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifiers\",\"outputs\":[{\"internalType\":\"contractIFunctionVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
	Bin: "0x608060405234801561001057600080fd5b506103d4806100206000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c80636f652c951461005c5780638bcfc3a014610071578063e7ddf4c6146100b6578063efe1c950146100c9578063f7da7486146100f2575b600080fd5b61006f61006a36600461031d565b610105565b005b61009a61007f366004610349565b6001602052600090815260409020546001600160a01b031681565b6040516001600160a01b03909116815260200160405180910390f35b61006f6100c436600461031d565b6101c9565b61009a6100d7366004610349565b6000602081905290815260409020546001600160a01b031681565b61006f610100366004610362565b610288565b6000828152602081905260409020546001600160a01b03166101425760405163632e273160e11b8152600481018390526024015b60405180910390fd5b6000828152600160205260409020546001600160a01b0316331461019b5760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610139565b60009182526020829052604090912080546001600160a01b0319166001600160a01b03909216919091179055565b6000828152602081905260409020546001600160a01b03166102015760405163632e273160e11b815260048101839052602401610139565b6000828152600160205260409020546001600160a01b0316331461025a5760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b039091166024820152604401610139565b60009182526001602052604090912080546001600160a01b0319166001600160a01b03909216919091179055565b6000838152602081905260409020546001600160a01b0316156102c157604051635e34c78f60e01b815260048101849052602401610139565b60009283526020838152604080852080546001600160a01b03199081166001600160a01b0396871617909155600190925290932080549093169116179055565b80356001600160a01b038116811461031857600080fd5b919050565b6000806040838503121561033057600080fd5b8235915061034060208401610301565b90509250929050565b60006020828403121561035b57600080fd5b5035919050565b60008060006060848603121561037757600080fd5b8335925061038760208501610301565b915061039560408501610301565b9050925092509256fea2646970667358221220807e761799286a9b05188ba917cb7cf2d319dd47ce4e5bea1b1f75e00e9fab2d64736f6c63430008140033",
}

// FunctionRegistryABI is the input ABI used to generate the binding from.
// Deprecated: Use FunctionRegistryMetaData.ABI instead.
var FunctionRegistryABI = FunctionRegistryMetaData.ABI

// FunctionRegistryBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use FunctionRegistryMetaData.Bin instead.
var FunctionRegistryBin = FunctionRegistryMetaData.Bin

// DeployFunctionRegistry deploys a new Ethereum contract, binding an instance of FunctionRegistry to it.
func DeployFunctionRegistry(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *FunctionRegistry, error) {
	parsed, err := FunctionRegistryMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(FunctionRegistryBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &FunctionRegistry{FunctionRegistryCaller: FunctionRegistryCaller{contract: contract}, FunctionRegistryTransactor: FunctionRegistryTransactor{contract: contract}, FunctionRegistryFilterer: FunctionRegistryFilterer{contract: contract}}, nil
}

// FunctionRegistry is an auto generated Go binding around an Ethereum contract.
type FunctionRegistry struct {
	FunctionRegistryCaller     // Read-only binding to the contract
	FunctionRegistryTransactor // Write-only binding to the contract
	FunctionRegistryFilterer   // Log filterer for contract events
}

// FunctionRegistryCaller is an auto generated read-only Go binding around an Ethereum contract.
type FunctionRegistryCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FunctionRegistryTransactor is an auto generated write-only Go binding around an Ethereum contract.
type FunctionRegistryTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FunctionRegistryFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type FunctionRegistryFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FunctionRegistrySession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type FunctionRegistrySession struct {
	Contract     *FunctionRegistry // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// FunctionRegistryCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type FunctionRegistryCallerSession struct {
	Contract *FunctionRegistryCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts           // Call options to use throughout this session
}

// FunctionRegistryTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type FunctionRegistryTransactorSession struct {
	Contract     *FunctionRegistryTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts           // Transaction auth options to use throughout this session
}

// FunctionRegistryRaw is an auto generated low-level Go binding around an Ethereum contract.
type FunctionRegistryRaw struct {
	Contract *FunctionRegistry // Generic contract binding to access the raw methods on
}

// FunctionRegistryCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type FunctionRegistryCallerRaw struct {
	Contract *FunctionRegistryCaller // Generic read-only contract binding to access the raw methods on
}

// FunctionRegistryTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type FunctionRegistryTransactorRaw struct {
	Contract *FunctionRegistryTransactor // Generic write-only contract binding to access the raw methods on
}

// NewFunctionRegistry creates a new instance of FunctionRegistry, bound to a specific deployed contract.
func NewFunctionRegistry(address common.Address, backend bind.ContractBackend) (*FunctionRegistry, error) {
	contract, err := bindFunctionRegistry(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistry{FunctionRegistryCaller: FunctionRegistryCaller{contract: contract}, FunctionRegistryTransactor: FunctionRegistryTransactor{contract: contract}, FunctionRegistryFilterer: FunctionRegistryFilterer{contract: contract}}, nil
}

// NewFunctionRegistryCaller creates a new read-only instance of FunctionRegistry, bound to a specific deployed contract.
func NewFunctionRegistryCaller(address common.Address, caller bind.ContractCaller) (*FunctionRegistryCaller, error) {
	contract, err := bindFunctionRegistry(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryCaller{contract: contract}, nil
}

// NewFunctionRegistryTransactor creates a new write-only instance of FunctionRegistry, bound to a specific deployed contract.
func NewFunctionRegistryTransactor(address common.Address, transactor bind.ContractTransactor) (*FunctionRegistryTransactor, error) {
	contract, err := bindFunctionRegistry(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryTransactor{contract: contract}, nil
}

// NewFunctionRegistryFilterer creates a new log filterer instance of FunctionRegistry, bound to a specific deployed contract.
func NewFunctionRegistryFilterer(address common.Address, filterer bind.ContractFilterer) (*FunctionRegistryFilterer, error) {
	contract, err := bindFunctionRegistry(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryFilterer{contract: contract}, nil
}

// bindFunctionRegistry binds a generic wrapper to an already deployed contract.
func bindFunctionRegistry(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := FunctionRegistryMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_FunctionRegistry *FunctionRegistryRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _FunctionRegistry.Contract.FunctionRegistryCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_FunctionRegistry *FunctionRegistryRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.FunctionRegistryTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_FunctionRegistry *FunctionRegistryRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.FunctionRegistryTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_FunctionRegistry *FunctionRegistryCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _FunctionRegistry.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_FunctionRegistry *FunctionRegistryTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_FunctionRegistry *FunctionRegistryTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.contract.Transact(opts, method, params...)
}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_FunctionRegistry *FunctionRegistryCaller) VerifierOwners(opts *bind.CallOpts, arg0 [32]byte) (common.Address, error) {
	var out []interface{}
	err := _FunctionRegistry.contract.Call(opts, &out, "verifierOwners", arg0)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_FunctionRegistry *FunctionRegistrySession) VerifierOwners(arg0 [32]byte) (common.Address, error) {
	return _FunctionRegistry.Contract.VerifierOwners(&_FunctionRegistry.CallOpts, arg0)
}

// VerifierOwners is a free data retrieval call binding the contract method 0x8bcfc3a0.
//
// Solidity: function verifierOwners(bytes32 ) view returns(address)
func (_FunctionRegistry *FunctionRegistryCallerSession) VerifierOwners(arg0 [32]byte) (common.Address, error) {
	return _FunctionRegistry.Contract.VerifierOwners(&_FunctionRegistry.CallOpts, arg0)
}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_FunctionRegistry *FunctionRegistryCaller) Verifiers(opts *bind.CallOpts, arg0 [32]byte) (common.Address, error) {
	var out []interface{}
	err := _FunctionRegistry.contract.Call(opts, &out, "verifiers", arg0)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_FunctionRegistry *FunctionRegistrySession) Verifiers(arg0 [32]byte) (common.Address, error) {
	return _FunctionRegistry.Contract.Verifiers(&_FunctionRegistry.CallOpts, arg0)
}

// Verifiers is a free data retrieval call binding the contract method 0xefe1c950.
//
// Solidity: function verifiers(bytes32 ) view returns(address)
func (_FunctionRegistry *FunctionRegistryCallerSession) Verifiers(arg0 [32]byte) (common.Address, error) {
	return _FunctionRegistry.Contract.Verifiers(&_FunctionRegistry.CallOpts, arg0)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0xf7da7486.
//
// Solidity: function registerFunction(bytes32 _functionId, address _verifier, address _owner) returns()
func (_FunctionRegistry *FunctionRegistryTransactor) RegisterFunction(opts *bind.TransactOpts, _functionId [32]byte, _verifier common.Address, _owner common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "registerFunction", _functionId, _verifier, _owner)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0xf7da7486.
//
// Solidity: function registerFunction(bytes32 _functionId, address _verifier, address _owner) returns()
func (_FunctionRegistry *FunctionRegistrySession) RegisterFunction(_functionId [32]byte, _verifier common.Address, _owner common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.RegisterFunction(&_FunctionRegistry.TransactOpts, _functionId, _verifier, _owner)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0xf7da7486.
//
// Solidity: function registerFunction(bytes32 _functionId, address _verifier, address _owner) returns()
func (_FunctionRegistry *FunctionRegistryTransactorSession) RegisterFunction(_functionId [32]byte, _verifier common.Address, _owner common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.RegisterFunction(&_FunctionRegistry.TransactOpts, _functionId, _verifier, _owner)
}

// UpdateFunctionOwner is a paid mutator transaction binding the contract method 0xe7ddf4c6.
//
// Solidity: function updateFunctionOwner(bytes32 _functionId, address _owner) returns()
func (_FunctionRegistry *FunctionRegistryTransactor) UpdateFunctionOwner(opts *bind.TransactOpts, _functionId [32]byte, _owner common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "updateFunctionOwner", _functionId, _owner)
}

// UpdateFunctionOwner is a paid mutator transaction binding the contract method 0xe7ddf4c6.
//
// Solidity: function updateFunctionOwner(bytes32 _functionId, address _owner) returns()
func (_FunctionRegistry *FunctionRegistrySession) UpdateFunctionOwner(_functionId [32]byte, _owner common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.UpdateFunctionOwner(&_FunctionRegistry.TransactOpts, _functionId, _owner)
}

// UpdateFunctionOwner is a paid mutator transaction binding the contract method 0xe7ddf4c6.
//
// Solidity: function updateFunctionOwner(bytes32 _functionId, address _owner) returns()
func (_FunctionRegistry *FunctionRegistryTransactorSession) UpdateFunctionOwner(_functionId [32]byte, _owner common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.UpdateFunctionOwner(&_FunctionRegistry.TransactOpts, _functionId, _owner)
}

// UpdateFunctionVerifier is a paid mutator transaction binding the contract method 0x6f652c95.
//
// Solidity: function updateFunctionVerifier(bytes32 _functionId, address _verifier) returns()
func (_FunctionRegistry *FunctionRegistryTransactor) UpdateFunctionVerifier(opts *bind.TransactOpts, _functionId [32]byte, _verifier common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "updateFunctionVerifier", _functionId, _verifier)
}

// UpdateFunctionVerifier is a paid mutator transaction binding the contract method 0x6f652c95.
//
// Solidity: function updateFunctionVerifier(bytes32 _functionId, address _verifier) returns()
func (_FunctionRegistry *FunctionRegistrySession) UpdateFunctionVerifier(_functionId [32]byte, _verifier common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.UpdateFunctionVerifier(&_FunctionRegistry.TransactOpts, _functionId, _verifier)
}

// UpdateFunctionVerifier is a paid mutator transaction binding the contract method 0x6f652c95.
//
// Solidity: function updateFunctionVerifier(bytes32 _functionId, address _verifier) returns()
func (_FunctionRegistry *FunctionRegistryTransactorSession) UpdateFunctionVerifier(_functionId [32]byte, _verifier common.Address) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.UpdateFunctionVerifier(&_FunctionRegistry.TransactOpts, _functionId, _verifier)
}

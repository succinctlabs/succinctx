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
	ABI: "[{\"inputs\":[],\"name\":\"EmptyBytecode\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"FailedDeploy\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"name\":\"FunctionAlreadyRegistered\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"actualOwner\",\"type\":\"address\"}],\"name\":\"NotFunctionOwner\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"VerifierCannotBeZero\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"bytecodeHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"deployedAddress\",\"type\":\"address\"}],\"name\":\"Deployed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"FunctionOwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"FunctionRegistered\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"name\":\"FunctionVerifierUpdated\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_bytecode\",\"type\":\"bytes\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"deployAndRegisterFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_bytecode\",\"type\":\"bytes\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"deployAndUpdateFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"getFunctionId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"registerFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"}],\"name\":\"updateFunction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"functionId\",\"type\":\"bytes32\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifierOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"verifiers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
	Bin: "0x608060405234801561001057600080fd5b5061084c806100206000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c80639538f56f1161005b5780639538f56f14610119578063b63755e51461012c578063bd58c4bb1461013f578063efe1c9501461015257600080fd5b80635c74ad561461008257806368ff41b1146100b75780638bcfc3a0146100d8575b600080fd5b610095610090366004610689565b61017b565b604080519283526001600160a01b039091166020830152015b60405180910390f35b6100ca6100c5366004610704565b610260565b6040519081526020016100ae565b6101016100e6366004610756565b6001602052600090815260409020546001600160a01b031681565b6040516001600160a01b0390911681526020016100ae565b6100ca610127366004610704565b61034e565b61009561013a366004610689565b610381565b6100ca61014d366004610704565b610452565b610101610160366004610756565b6000602081905290815260409020546001600160a01b031681565b600080610188338461034e565b6000818152602081905260409020549092506001600160a01b0316156101c957604051635e34c78f60e01b8152600481018390526024015b60405180910390fd5b600082815260016020526040902080546001600160a01b031916331790556101f1848361053c565b6000838152602081905260409081902080546001600160a01b0319166001600160a01b0384161790555190915082907f52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea90610251908490879033906107b5565b60405180910390a29250929050565b600061026c338361034e565b6000818152602081905260409020549091506001600160a01b0316156102a857604051635e34c78f60e01b8152600481018290526024016101c0565b6001600160a01b0383166102cf576040516302d48d1f60e61b815260040160405180910390fd5b60008181526020818152604080832080546001600160a01b03199081166001600160a01b03891617909155600190925291829020805433921682179055905182917f52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea916103409187918791906107b5565b60405180910390a292915050565b600082826040516020016103639291906107ea565b60405160208183030381529060405280519060200120905092915050565b60008061038e338461034e565b6000818152600160205260409020549092506001600160a01b031633146103ea5760008281526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b0390911660248201526044016101c0565b6103f4848361053c565b6000838152602081815260409182902080546001600160a01b0319166001600160a01b038516908117909155915191825291925083917ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369101610251565b600061045e338361034e565b6000818152600160205260409020549091506001600160a01b031633146104ba5760008181526001602052604090819020549051633368f56b60e11b81523360048201526001600160a01b0390911660248201526044016101c0565b6001600160a01b0383166104e1576040516302d48d1f60e61b815260040160405180910390fd5b6000818152602081815260409182902080546001600160a01b0319166001600160a01b038716908117909155915191825282917ffc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b7369101610340565b60008251600003610560576040516321744a5960e01b815260040160405180910390fd5b818351602085016000f590506001600160a01b03811661059357604051632081741d60e11b815260040160405180910390fd5b825160208401206040516001600160a01b0383169184917f27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f53290600090a492915050565b634e487b7160e01b600052604160045260246000fd5b600067ffffffffffffffff80841115610607576106076105d6565b604051601f8501601f19908116603f0116810190828211818310171561062f5761062f6105d6565b8160405280935085815286868601111561064857600080fd5b858560208301376000602087830101525050509392505050565b600082601f83011261067357600080fd5b610682838335602085016105ec565b9392505050565b6000806040838503121561069c57600080fd5b823567ffffffffffffffff808211156106b457600080fd5b818501915085601f8301126106c857600080fd5b6106d7868335602085016105ec565b935060208501359150808211156106ed57600080fd5b506106fa85828601610662565b9150509250929050565b6000806040838503121561071757600080fd5b82356001600160a01b038116811461072e57600080fd5b9150602083013567ffffffffffffffff81111561074a57600080fd5b6106fa85828601610662565b60006020828403121561076857600080fd5b5035919050565b6000815180845260005b8181101561079557602081850181015186830182015201610779565b506000602082860101526020601f19601f83011685010191505092915050565b600060018060a01b038086168352606060208401526107d7606084018661076f565b9150808416604084015250949350505050565b6001600160a01b038316815260406020820181905260009061080e9083018461076f565b94935050505056fea2646970667358221220fd7ff5954385b1bf9be27570b28accdece510982b131808dbe1bec973842ec6664736f6c63430008130033",
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

// GetFunctionId is a free data retrieval call binding the contract method 0x9538f56f.
//
// Solidity: function getFunctionId(address _owner, string _name) pure returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistryCaller) GetFunctionId(opts *bind.CallOpts, _owner common.Address, _name string) ([32]byte, error) {
	var out []interface{}
	err := _FunctionRegistry.contract.Call(opts, &out, "getFunctionId", _owner, _name)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetFunctionId is a free data retrieval call binding the contract method 0x9538f56f.
//
// Solidity: function getFunctionId(address _owner, string _name) pure returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistrySession) GetFunctionId(_owner common.Address, _name string) ([32]byte, error) {
	return _FunctionRegistry.Contract.GetFunctionId(&_FunctionRegistry.CallOpts, _owner, _name)
}

// GetFunctionId is a free data retrieval call binding the contract method 0x9538f56f.
//
// Solidity: function getFunctionId(address _owner, string _name) pure returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistryCallerSession) GetFunctionId(_owner common.Address, _name string) ([32]byte, error) {
	return _FunctionRegistry.Contract.GetFunctionId(&_FunctionRegistry.CallOpts, _owner, _name)
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

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x5c74ad56.
//
// Solidity: function deployAndRegisterFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionRegistry *FunctionRegistryTransactor) DeployAndRegisterFunction(opts *bind.TransactOpts, _bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "deployAndRegisterFunction", _bytecode, _name)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x5c74ad56.
//
// Solidity: function deployAndRegisterFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionRegistry *FunctionRegistrySession) DeployAndRegisterFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.DeployAndRegisterFunction(&_FunctionRegistry.TransactOpts, _bytecode, _name)
}

// DeployAndRegisterFunction is a paid mutator transaction binding the contract method 0x5c74ad56.
//
// Solidity: function deployAndRegisterFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionRegistry *FunctionRegistryTransactorSession) DeployAndRegisterFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.DeployAndRegisterFunction(&_FunctionRegistry.TransactOpts, _bytecode, _name)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xb63755e5.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionRegistry *FunctionRegistryTransactor) DeployAndUpdateFunction(opts *bind.TransactOpts, _bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "deployAndUpdateFunction", _bytecode, _name)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xb63755e5.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionRegistry *FunctionRegistrySession) DeployAndUpdateFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.DeployAndUpdateFunction(&_FunctionRegistry.TransactOpts, _bytecode, _name)
}

// DeployAndUpdateFunction is a paid mutator transaction binding the contract method 0xb63755e5.
//
// Solidity: function deployAndUpdateFunction(bytes _bytecode, string _name) returns(bytes32 functionId, address verifier)
func (_FunctionRegistry *FunctionRegistryTransactorSession) DeployAndUpdateFunction(_bytecode []byte, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.DeployAndUpdateFunction(&_FunctionRegistry.TransactOpts, _bytecode, _name)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x68ff41b1.
//
// Solidity: function registerFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistryTransactor) RegisterFunction(opts *bind.TransactOpts, _verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "registerFunction", _verifier, _name)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x68ff41b1.
//
// Solidity: function registerFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistrySession) RegisterFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.RegisterFunction(&_FunctionRegistry.TransactOpts, _verifier, _name)
}

// RegisterFunction is a paid mutator transaction binding the contract method 0x68ff41b1.
//
// Solidity: function registerFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistryTransactorSession) RegisterFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.RegisterFunction(&_FunctionRegistry.TransactOpts, _verifier, _name)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xbd58c4bb.
//
// Solidity: function updateFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistryTransactor) UpdateFunction(opts *bind.TransactOpts, _verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.contract.Transact(opts, "updateFunction", _verifier, _name)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xbd58c4bb.
//
// Solidity: function updateFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistrySession) UpdateFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.UpdateFunction(&_FunctionRegistry.TransactOpts, _verifier, _name)
}

// UpdateFunction is a paid mutator transaction binding the contract method 0xbd58c4bb.
//
// Solidity: function updateFunction(address _verifier, string _name) returns(bytes32 functionId)
func (_FunctionRegistry *FunctionRegistryTransactorSession) UpdateFunction(_verifier common.Address, _name string) (*types.Transaction, error) {
	return _FunctionRegistry.Contract.UpdateFunction(&_FunctionRegistry.TransactOpts, _verifier, _name)
}

// FunctionRegistryDeployedIterator is returned from FilterDeployed and is used to iterate over the raw logs and unpacked data for Deployed events raised by the FunctionRegistry contract.
type FunctionRegistryDeployedIterator struct {
	Event *FunctionRegistryDeployed // Event containing the contract specifics and raw log

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
func (it *FunctionRegistryDeployedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionRegistryDeployed)
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
		it.Event = new(FunctionRegistryDeployed)
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
func (it *FunctionRegistryDeployedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionRegistryDeployedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionRegistryDeployed represents a Deployed event raised by the FunctionRegistry contract.
type FunctionRegistryDeployed struct {
	BytecodeHash    [32]byte
	Salt            [32]byte
	DeployedAddress common.Address
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterDeployed is a free log retrieval operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_FunctionRegistry *FunctionRegistryFilterer) FilterDeployed(opts *bind.FilterOpts, bytecodeHash [][32]byte, salt [][32]byte, deployedAddress []common.Address) (*FunctionRegistryDeployedIterator, error) {

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

	logs, sub, err := _FunctionRegistry.contract.FilterLogs(opts, "Deployed", bytecodeHashRule, saltRule, deployedAddressRule)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryDeployedIterator{contract: _FunctionRegistry.contract, event: "Deployed", logs: logs, sub: sub}, nil
}

// WatchDeployed is a free log subscription operation binding the contract event 0x27b8e3132afa95254770e1c1d214eafde52bc47d1b6e1f5dfcbb380c3ca3f532.
//
// Solidity: event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress)
func (_FunctionRegistry *FunctionRegistryFilterer) WatchDeployed(opts *bind.WatchOpts, sink chan<- *FunctionRegistryDeployed, bytecodeHash [][32]byte, salt [][32]byte, deployedAddress []common.Address) (event.Subscription, error) {

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

	logs, sub, err := _FunctionRegistry.contract.WatchLogs(opts, "Deployed", bytecodeHashRule, saltRule, deployedAddressRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionRegistryDeployed)
				if err := _FunctionRegistry.contract.UnpackLog(event, "Deployed", log); err != nil {
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
func (_FunctionRegistry *FunctionRegistryFilterer) ParseDeployed(log types.Log) (*FunctionRegistryDeployed, error) {
	event := new(FunctionRegistryDeployed)
	if err := _FunctionRegistry.contract.UnpackLog(event, "Deployed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionRegistryFunctionOwnerUpdatedIterator is returned from FilterFunctionOwnerUpdated and is used to iterate over the raw logs and unpacked data for FunctionOwnerUpdated events raised by the FunctionRegistry contract.
type FunctionRegistryFunctionOwnerUpdatedIterator struct {
	Event *FunctionRegistryFunctionOwnerUpdated // Event containing the contract specifics and raw log

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
func (it *FunctionRegistryFunctionOwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionRegistryFunctionOwnerUpdated)
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
		it.Event = new(FunctionRegistryFunctionOwnerUpdated)
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
func (it *FunctionRegistryFunctionOwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionRegistryFunctionOwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionRegistryFunctionOwnerUpdated represents a FunctionOwnerUpdated event raised by the FunctionRegistry contract.
type FunctionRegistryFunctionOwnerUpdated struct {
	FunctionId [32]byte
	Owner      common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionOwnerUpdated is a free log retrieval operation binding the contract event 0x376b0a13fca0286b5c7c73288ea980eb9d131fc8b996f7a46a49e0f90269aadf.
//
// Solidity: event FunctionOwnerUpdated(bytes32 indexed functionId, address owner)
func (_FunctionRegistry *FunctionRegistryFilterer) FilterFunctionOwnerUpdated(opts *bind.FilterOpts, functionId [][32]byte) (*FunctionRegistryFunctionOwnerUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionRegistry.contract.FilterLogs(opts, "FunctionOwnerUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryFunctionOwnerUpdatedIterator{contract: _FunctionRegistry.contract, event: "FunctionOwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchFunctionOwnerUpdated is a free log subscription operation binding the contract event 0x376b0a13fca0286b5c7c73288ea980eb9d131fc8b996f7a46a49e0f90269aadf.
//
// Solidity: event FunctionOwnerUpdated(bytes32 indexed functionId, address owner)
func (_FunctionRegistry *FunctionRegistryFilterer) WatchFunctionOwnerUpdated(opts *bind.WatchOpts, sink chan<- *FunctionRegistryFunctionOwnerUpdated, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionRegistry.contract.WatchLogs(opts, "FunctionOwnerUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionRegistryFunctionOwnerUpdated)
				if err := _FunctionRegistry.contract.UnpackLog(event, "FunctionOwnerUpdated", log); err != nil {
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
func (_FunctionRegistry *FunctionRegistryFilterer) ParseFunctionOwnerUpdated(log types.Log) (*FunctionRegistryFunctionOwnerUpdated, error) {
	event := new(FunctionRegistryFunctionOwnerUpdated)
	if err := _FunctionRegistry.contract.UnpackLog(event, "FunctionOwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionRegistryFunctionRegisteredIterator is returned from FilterFunctionRegistered and is used to iterate over the raw logs and unpacked data for FunctionRegistered events raised by the FunctionRegistry contract.
type FunctionRegistryFunctionRegisteredIterator struct {
	Event *FunctionRegistryFunctionRegistered // Event containing the contract specifics and raw log

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
func (it *FunctionRegistryFunctionRegisteredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionRegistryFunctionRegistered)
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
		it.Event = new(FunctionRegistryFunctionRegistered)
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
func (it *FunctionRegistryFunctionRegisteredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionRegistryFunctionRegisteredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionRegistryFunctionRegistered represents a FunctionRegistered event raised by the FunctionRegistry contract.
type FunctionRegistryFunctionRegistered struct {
	FunctionId [32]byte
	Verifier   common.Address
	Name       string
	Owner      common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionRegistered is a free log retrieval operation binding the contract event 0x52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, string name, address owner)
func (_FunctionRegistry *FunctionRegistryFilterer) FilterFunctionRegistered(opts *bind.FilterOpts, functionId [][32]byte) (*FunctionRegistryFunctionRegisteredIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionRegistry.contract.FilterLogs(opts, "FunctionRegistered", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryFunctionRegisteredIterator{contract: _FunctionRegistry.contract, event: "FunctionRegistered", logs: logs, sub: sub}, nil
}

// WatchFunctionRegistered is a free log subscription operation binding the contract event 0x52664851d3d2a6452a5b4ce529443a2e880f03048598d72fbc426d7402956dea.
//
// Solidity: event FunctionRegistered(bytes32 indexed functionId, address verifier, string name, address owner)
func (_FunctionRegistry *FunctionRegistryFilterer) WatchFunctionRegistered(opts *bind.WatchOpts, sink chan<- *FunctionRegistryFunctionRegistered, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionRegistry.contract.WatchLogs(opts, "FunctionRegistered", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionRegistryFunctionRegistered)
				if err := _FunctionRegistry.contract.UnpackLog(event, "FunctionRegistered", log); err != nil {
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
func (_FunctionRegistry *FunctionRegistryFilterer) ParseFunctionRegistered(log types.Log) (*FunctionRegistryFunctionRegistered, error) {
	event := new(FunctionRegistryFunctionRegistered)
	if err := _FunctionRegistry.contract.UnpackLog(event, "FunctionRegistered", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FunctionRegistryFunctionVerifierUpdatedIterator is returned from FilterFunctionVerifierUpdated and is used to iterate over the raw logs and unpacked data for FunctionVerifierUpdated events raised by the FunctionRegistry contract.
type FunctionRegistryFunctionVerifierUpdatedIterator struct {
	Event *FunctionRegistryFunctionVerifierUpdated // Event containing the contract specifics and raw log

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
func (it *FunctionRegistryFunctionVerifierUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FunctionRegistryFunctionVerifierUpdated)
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
		it.Event = new(FunctionRegistryFunctionVerifierUpdated)
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
func (it *FunctionRegistryFunctionVerifierUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FunctionRegistryFunctionVerifierUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FunctionRegistryFunctionVerifierUpdated represents a FunctionVerifierUpdated event raised by the FunctionRegistry contract.
type FunctionRegistryFunctionVerifierUpdated struct {
	FunctionId [32]byte
	Verifier   common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterFunctionVerifierUpdated is a free log retrieval operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_FunctionRegistry *FunctionRegistryFilterer) FilterFunctionVerifierUpdated(opts *bind.FilterOpts, functionId [][32]byte) (*FunctionRegistryFunctionVerifierUpdatedIterator, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionRegistry.contract.FilterLogs(opts, "FunctionVerifierUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return &FunctionRegistryFunctionVerifierUpdatedIterator{contract: _FunctionRegistry.contract, event: "FunctionVerifierUpdated", logs: logs, sub: sub}, nil
}

// WatchFunctionVerifierUpdated is a free log subscription operation binding the contract event 0xfc14566d4fed0acece30e4fd5b3f5f6dadee9c5ecb852fdaf9c13999c733b736.
//
// Solidity: event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier)
func (_FunctionRegistry *FunctionRegistryFilterer) WatchFunctionVerifierUpdated(opts *bind.WatchOpts, sink chan<- *FunctionRegistryFunctionVerifierUpdated, functionId [][32]byte) (event.Subscription, error) {

	var functionIdRule []interface{}
	for _, functionIdItem := range functionId {
		functionIdRule = append(functionIdRule, functionIdItem)
	}

	logs, sub, err := _FunctionRegistry.contract.WatchLogs(opts, "FunctionVerifierUpdated", functionIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FunctionRegistryFunctionVerifierUpdated)
				if err := _FunctionRegistry.contract.UnpackLog(event, "FunctionVerifierUpdated", log); err != nil {
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
func (_FunctionRegistry *FunctionRegistryFilterer) ParseFunctionVerifierUpdated(log types.Log) (*FunctionRegistryFunctionVerifierUpdated, error) {
	event := new(FunctionRegistryFunctionVerifierUpdated)
	if err := _FunctionRegistry.contract.UnpackLog(event, "FunctionVerifierUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

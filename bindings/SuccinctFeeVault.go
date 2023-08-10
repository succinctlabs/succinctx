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

// SuccinctFeeVaultMetaData contains all meta data concerning the SuccinctFeeVault contract.
var SuccinctFeeVaultMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"FailedToSendNative\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"InsufficentAllowance\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"InsufficientBalance\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"InvalidAccount\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"InvalidToken\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"OnlyDeductor\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"Collected\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"Deducted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"Received\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_deductor\",\"type\":\"address\"}],\"name\":\"addDeductor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"allowedDeductors\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"balances\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"collect\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"collectNative\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"deduct\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"deductNative\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"}],\"name\":\"depositNative\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_deductor\",\"type\":\"address\"}],\"name\":\"removeDeductor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
	Bin: "0x608060405234801561001057600080fd5b5060405162000f9138038062000f9183398101604081905261003191610173565b61003a33610049565b61004381610099565b506101a3565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6100a1610117565b6001600160a01b03811661010b5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b61011481610049565b50565b6000546001600160a01b031633146101715760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610102565b565b60006020828403121561018557600080fd5b81516001600160a01b038116811461019c57600080fd5b9392505050565b610dde80620001b36000396000f3fe6080604052600436106100c25760003560e01c8063929c51791161007f578063cf6ee7a011610059578063cf6ee7a014610204578063e5f8699d14610224578063ee3985b314610244578063f2fde38b1461028457600080fd5b8063929c51791461017e578063c23f001f1461019e578063c8fea2fb146101e457600080fd5b806333bb7f91146100c757806339e0273c146100dc578063715018a6146100fc5780638340f549146101115780638da5cb5b1461013157806391ab27341461015e575b600080fd5b6100da6100d5366004610c70565b6102a4565b005b3480156100e857600080fd5b506100da6100f7366004610c70565b61036b565b34801561010857600080fd5b506100da610397565b34801561011d57600080fd5b506100da61012c366004610c92565b6103ab565b34801561013d57600080fd5b506000546040516001600160a01b0390911681526020015b60405180910390f35b34801561016a57600080fd5b506100da610179366004610cce565b6105c1565b34801561018a57600080fd5b506100da610199366004610c92565b6106b2565b3480156101aa57600080fd5b506101d66101b9366004610cf8565b600160209081526000928352604080842090915290825290205481565b604051908152602001610155565b3480156101f057600080fd5b506100da6101ff366004610c92565b610823565b34801561021057600080fd5b506100da61021f366004610cce565b6109b1565b34801561023057600080fd5b506100da61023f366004610c70565b610b08565b34801561025057600080fd5b5061027461025f366004610c70565b60026020526000908152604090205460ff1681565b6040519015158152602001610155565b34801561029057600080fd5b506100da61029f366004610c70565b610b31565b6001600160a01b0381166102db576040516325abcd9160e11b81526001600160a01b03821660048201526024015b60405180910390fd5b6001600160a01b03811660009081527fa6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb49602052604081208054349290610322908490610d41565b90915550506040513481526000906001600160a01b038316907f8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb9060200160405180910390a350565b610373610baa565b6001600160a01b03166000908152600260205260409020805460ff19166001179055565b61039f610baa565b6103a96000610c04565b565b6001600160a01b0383166103dd576040516325abcd9160e11b81526001600160a01b03841660048201526024016102d2565b6001600160a01b03821661040f5760405163961c9a4f60e01b81526001600160a01b03831660048201526024016102d2565b604051636eb1769f60e11b815233600482015230602482015282906000906001600160a01b0383169063dd62ed3e90604401602060405180830381865afa15801561045e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104829190610d5a565b9050828110156104b75760405163a20569a160e01b81526001600160a01b0385166004820152602481018490526044016102d2565b6040516323b872dd60e01b8152336004820152306024820152604481018490526001600160a01b038316906323b872dd906064016020604051808303816000875af115801561050a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061052e9190610d73565b506001600160a01b03808516600090815260016020908152604080832093891683529290529081208054859290610566908490610d41565b92505081905550836001600160a01b0316856001600160a01b03167f8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb856040516105b291815260200190565b60405180910390a35050505050565b6105c9610baa565b804710156105f457604051633db7aa8160e21b815260006004820152602481018290526044016102d2565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114610641576040519150601f19603f3d011682016040523d82523d6000602084013e610646565b606091505b505090508061066b5760405163d23dc39360e01b8152600481018390526024016102d2565b6040518281526000906001600160a01b038516907f484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd906020015b60405180910390a3505050565b3360009081526002602052604090205460ff166106e45760405163f9434e0160e01b81523360048201526024016102d2565b6001600160a01b038316610716576040516325abcd9160e11b81526001600160a01b03841660048201526024016102d2565b6001600160a01b0382166107485760405163961c9a4f60e01b81526001600160a01b03831660048201526024016102d2565b6001600160a01b038083166000908152600160209081526040808320938716835292905220548111156107a057604051633db7aa8160e21b81526001600160a01b0383166004820152602481018290526044016102d2565b6001600160a01b038083166000908152600160209081526040808320938716835292905290812080548392906107d7908490610d95565b92505081905550816001600160a01b0316836001600160a01b03167f14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e45836040516106a591815260200190565b61082b610baa565b6001600160a01b03821661085d5760405163961c9a4f60e01b81526001600160a01b03831660048201526024016102d2565b6040516370a0823160e01b815230600482015281906001600160a01b038416906370a0823190602401602060405180830381865afa1580156108a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108c79190610d5a565b10156108f857604051633db7aa8160e21b81526001600160a01b0383166004820152602481018290526044016102d2565b60405163a9059cbb60e01b81526001600160a01b0384811660048301526024820183905283169063a9059cbb906044016020604051808303816000875af1158015610947573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061096b9190610d73565b50816001600160a01b0316836001600160a01b03167f484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd836040516106a591815260200190565b3360009081526002602052604090205460ff166109e35760405163f9434e0160e01b81523360048201526024016102d2565b6001600160a01b038216610a15576040516325abcd9160e11b81526001600160a01b03831660048201526024016102d2565b6001600160a01b03821660009081527fa6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb496020526040902054811115610a7757604051633db7aa8160e21b815260006004820152602481018290526044016102d2565b6001600160a01b03821660009081527fa6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb49602052604081208054839290610abe908490610d95565b90915550506040518181526000906001600160a01b038416907f14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e459060200160405180910390a35050565b610b10610baa565b6001600160a01b03166000908152600260205260409020805460ff19169055565b610b39610baa565b6001600160a01b038116610b9e5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016102d2565b610ba781610c04565b50565b6000546001600160a01b031633146103a95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016102d2565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b80356001600160a01b0381168114610c6b57600080fd5b919050565b600060208284031215610c8257600080fd5b610c8b82610c54565b9392505050565b600080600060608486031215610ca757600080fd5b610cb084610c54565b9250610cbe60208501610c54565b9150604084013590509250925092565b60008060408385031215610ce157600080fd5b610cea83610c54565b946020939093013593505050565b60008060408385031215610d0b57600080fd5b610d1483610c54565b9150610d2260208401610c54565b90509250929050565b634e487b7160e01b600052601160045260246000fd5b80820180821115610d5457610d54610d2b565b92915050565b600060208284031215610d6c57600080fd5b5051919050565b600060208284031215610d8557600080fd5b81518015158114610c8b57600080fd5b81810381811115610d5457610d54610d2b56fea26469706673582212204148dfc9812cd97c949601a0ba1bfbc447d187805d8871f70b16c3baa462d28864736f6c63430008140033",
}

// SuccinctFeeVaultABI is the input ABI used to generate the binding from.
// Deprecated: Use SuccinctFeeVaultMetaData.ABI instead.
var SuccinctFeeVaultABI = SuccinctFeeVaultMetaData.ABI

// SuccinctFeeVaultBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use SuccinctFeeVaultMetaData.Bin instead.
var SuccinctFeeVaultBin = SuccinctFeeVaultMetaData.Bin

// DeploySuccinctFeeVault deploys a new Ethereum contract, binding an instance of SuccinctFeeVault to it.
func DeploySuccinctFeeVault(auth *bind.TransactOpts, backend bind.ContractBackend, _owner common.Address) (common.Address, *types.Transaction, *SuccinctFeeVault, error) {
	parsed, err := SuccinctFeeVaultMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(SuccinctFeeVaultBin), backend, _owner)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &SuccinctFeeVault{SuccinctFeeVaultCaller: SuccinctFeeVaultCaller{contract: contract}, SuccinctFeeVaultTransactor: SuccinctFeeVaultTransactor{contract: contract}, SuccinctFeeVaultFilterer: SuccinctFeeVaultFilterer{contract: contract}}, nil
}

// SuccinctFeeVault is an auto generated Go binding around an Ethereum contract.
type SuccinctFeeVault struct {
	SuccinctFeeVaultCaller     // Read-only binding to the contract
	SuccinctFeeVaultTransactor // Write-only binding to the contract
	SuccinctFeeVaultFilterer   // Log filterer for contract events
}

// SuccinctFeeVaultCaller is an auto generated read-only Go binding around an Ethereum contract.
type SuccinctFeeVaultCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SuccinctFeeVaultTransactor is an auto generated write-only Go binding around an Ethereum contract.
type SuccinctFeeVaultTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SuccinctFeeVaultFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type SuccinctFeeVaultFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SuccinctFeeVaultSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type SuccinctFeeVaultSession struct {
	Contract     *SuccinctFeeVault // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// SuccinctFeeVaultCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type SuccinctFeeVaultCallerSession struct {
	Contract *SuccinctFeeVaultCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts           // Call options to use throughout this session
}

// SuccinctFeeVaultTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type SuccinctFeeVaultTransactorSession struct {
	Contract     *SuccinctFeeVaultTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts           // Transaction auth options to use throughout this session
}

// SuccinctFeeVaultRaw is an auto generated low-level Go binding around an Ethereum contract.
type SuccinctFeeVaultRaw struct {
	Contract *SuccinctFeeVault // Generic contract binding to access the raw methods on
}

// SuccinctFeeVaultCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type SuccinctFeeVaultCallerRaw struct {
	Contract *SuccinctFeeVaultCaller // Generic read-only contract binding to access the raw methods on
}

// SuccinctFeeVaultTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type SuccinctFeeVaultTransactorRaw struct {
	Contract *SuccinctFeeVaultTransactor // Generic write-only contract binding to access the raw methods on
}

// NewSuccinctFeeVault creates a new instance of SuccinctFeeVault, bound to a specific deployed contract.
func NewSuccinctFeeVault(address common.Address, backend bind.ContractBackend) (*SuccinctFeeVault, error) {
	contract, err := bindSuccinctFeeVault(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVault{SuccinctFeeVaultCaller: SuccinctFeeVaultCaller{contract: contract}, SuccinctFeeVaultTransactor: SuccinctFeeVaultTransactor{contract: contract}, SuccinctFeeVaultFilterer: SuccinctFeeVaultFilterer{contract: contract}}, nil
}

// NewSuccinctFeeVaultCaller creates a new read-only instance of SuccinctFeeVault, bound to a specific deployed contract.
func NewSuccinctFeeVaultCaller(address common.Address, caller bind.ContractCaller) (*SuccinctFeeVaultCaller, error) {
	contract, err := bindSuccinctFeeVault(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultCaller{contract: contract}, nil
}

// NewSuccinctFeeVaultTransactor creates a new write-only instance of SuccinctFeeVault, bound to a specific deployed contract.
func NewSuccinctFeeVaultTransactor(address common.Address, transactor bind.ContractTransactor) (*SuccinctFeeVaultTransactor, error) {
	contract, err := bindSuccinctFeeVault(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultTransactor{contract: contract}, nil
}

// NewSuccinctFeeVaultFilterer creates a new log filterer instance of SuccinctFeeVault, bound to a specific deployed contract.
func NewSuccinctFeeVaultFilterer(address common.Address, filterer bind.ContractFilterer) (*SuccinctFeeVaultFilterer, error) {
	contract, err := bindSuccinctFeeVault(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultFilterer{contract: contract}, nil
}

// bindSuccinctFeeVault binds a generic wrapper to an already deployed contract.
func bindSuccinctFeeVault(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := SuccinctFeeVaultMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SuccinctFeeVault *SuccinctFeeVaultRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SuccinctFeeVault.Contract.SuccinctFeeVaultCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SuccinctFeeVault *SuccinctFeeVaultRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.SuccinctFeeVaultTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SuccinctFeeVault *SuccinctFeeVaultRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.SuccinctFeeVaultTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SuccinctFeeVault *SuccinctFeeVaultCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SuccinctFeeVault.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.contract.Transact(opts, method, params...)
}

// AllowedDeductors is a free data retrieval call binding the contract method 0xee3985b3.
//
// Solidity: function allowedDeductors(address ) view returns(bool)
func (_SuccinctFeeVault *SuccinctFeeVaultCaller) AllowedDeductors(opts *bind.CallOpts, arg0 common.Address) (bool, error) {
	var out []interface{}
	err := _SuccinctFeeVault.contract.Call(opts, &out, "allowedDeductors", arg0)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// AllowedDeductors is a free data retrieval call binding the contract method 0xee3985b3.
//
// Solidity: function allowedDeductors(address ) view returns(bool)
func (_SuccinctFeeVault *SuccinctFeeVaultSession) AllowedDeductors(arg0 common.Address) (bool, error) {
	return _SuccinctFeeVault.Contract.AllowedDeductors(&_SuccinctFeeVault.CallOpts, arg0)
}

// AllowedDeductors is a free data retrieval call binding the contract method 0xee3985b3.
//
// Solidity: function allowedDeductors(address ) view returns(bool)
func (_SuccinctFeeVault *SuccinctFeeVaultCallerSession) AllowedDeductors(arg0 common.Address) (bool, error) {
	return _SuccinctFeeVault.Contract.AllowedDeductors(&_SuccinctFeeVault.CallOpts, arg0)
}

// Balances is a free data retrieval call binding the contract method 0xc23f001f.
//
// Solidity: function balances(address , address ) view returns(uint256)
func (_SuccinctFeeVault *SuccinctFeeVaultCaller) Balances(opts *bind.CallOpts, arg0 common.Address, arg1 common.Address) (*big.Int, error) {
	var out []interface{}
	err := _SuccinctFeeVault.contract.Call(opts, &out, "balances", arg0, arg1)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Balances is a free data retrieval call binding the contract method 0xc23f001f.
//
// Solidity: function balances(address , address ) view returns(uint256)
func (_SuccinctFeeVault *SuccinctFeeVaultSession) Balances(arg0 common.Address, arg1 common.Address) (*big.Int, error) {
	return _SuccinctFeeVault.Contract.Balances(&_SuccinctFeeVault.CallOpts, arg0, arg1)
}

// Balances is a free data retrieval call binding the contract method 0xc23f001f.
//
// Solidity: function balances(address , address ) view returns(uint256)
func (_SuccinctFeeVault *SuccinctFeeVaultCallerSession) Balances(arg0 common.Address, arg1 common.Address) (*big.Int, error) {
	return _SuccinctFeeVault.Contract.Balances(&_SuccinctFeeVault.CallOpts, arg0, arg1)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_SuccinctFeeVault *SuccinctFeeVaultCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _SuccinctFeeVault.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_SuccinctFeeVault *SuccinctFeeVaultSession) Owner() (common.Address, error) {
	return _SuccinctFeeVault.Contract.Owner(&_SuccinctFeeVault.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_SuccinctFeeVault *SuccinctFeeVaultCallerSession) Owner() (common.Address, error) {
	return _SuccinctFeeVault.Contract.Owner(&_SuccinctFeeVault.CallOpts)
}

// AddDeductor is a paid mutator transaction binding the contract method 0x39e0273c.
//
// Solidity: function addDeductor(address _deductor) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) AddDeductor(opts *bind.TransactOpts, _deductor common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "addDeductor", _deductor)
}

// AddDeductor is a paid mutator transaction binding the contract method 0x39e0273c.
//
// Solidity: function addDeductor(address _deductor) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) AddDeductor(_deductor common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.AddDeductor(&_SuccinctFeeVault.TransactOpts, _deductor)
}

// AddDeductor is a paid mutator transaction binding the contract method 0x39e0273c.
//
// Solidity: function addDeductor(address _deductor) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) AddDeductor(_deductor common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.AddDeductor(&_SuccinctFeeVault.TransactOpts, _deductor)
}

// Collect is a paid mutator transaction binding the contract method 0xc8fea2fb.
//
// Solidity: function collect(address _to, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) Collect(opts *bind.TransactOpts, _to common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "collect", _to, _token, _amount)
}

// Collect is a paid mutator transaction binding the contract method 0xc8fea2fb.
//
// Solidity: function collect(address _to, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) Collect(_to common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Collect(&_SuccinctFeeVault.TransactOpts, _to, _token, _amount)
}

// Collect is a paid mutator transaction binding the contract method 0xc8fea2fb.
//
// Solidity: function collect(address _to, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) Collect(_to common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Collect(&_SuccinctFeeVault.TransactOpts, _to, _token, _amount)
}

// CollectNative is a paid mutator transaction binding the contract method 0x91ab2734.
//
// Solidity: function collectNative(address _to, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) CollectNative(opts *bind.TransactOpts, _to common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "collectNative", _to, _amount)
}

// CollectNative is a paid mutator transaction binding the contract method 0x91ab2734.
//
// Solidity: function collectNative(address _to, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) CollectNative(_to common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.CollectNative(&_SuccinctFeeVault.TransactOpts, _to, _amount)
}

// CollectNative is a paid mutator transaction binding the contract method 0x91ab2734.
//
// Solidity: function collectNative(address _to, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) CollectNative(_to common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.CollectNative(&_SuccinctFeeVault.TransactOpts, _to, _amount)
}

// Deduct is a paid mutator transaction binding the contract method 0x929c5179.
//
// Solidity: function deduct(address _account, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) Deduct(opts *bind.TransactOpts, _account common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "deduct", _account, _token, _amount)
}

// Deduct is a paid mutator transaction binding the contract method 0x929c5179.
//
// Solidity: function deduct(address _account, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) Deduct(_account common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Deduct(&_SuccinctFeeVault.TransactOpts, _account, _token, _amount)
}

// Deduct is a paid mutator transaction binding the contract method 0x929c5179.
//
// Solidity: function deduct(address _account, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) Deduct(_account common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Deduct(&_SuccinctFeeVault.TransactOpts, _account, _token, _amount)
}

// DeductNative is a paid mutator transaction binding the contract method 0xcf6ee7a0.
//
// Solidity: function deductNative(address _account, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) DeductNative(opts *bind.TransactOpts, _account common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "deductNative", _account, _amount)
}

// DeductNative is a paid mutator transaction binding the contract method 0xcf6ee7a0.
//
// Solidity: function deductNative(address _account, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) DeductNative(_account common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.DeductNative(&_SuccinctFeeVault.TransactOpts, _account, _amount)
}

// DeductNative is a paid mutator transaction binding the contract method 0xcf6ee7a0.
//
// Solidity: function deductNative(address _account, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) DeductNative(_account common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.DeductNative(&_SuccinctFeeVault.TransactOpts, _account, _amount)
}

// Deposit is a paid mutator transaction binding the contract method 0x8340f549.
//
// Solidity: function deposit(address _account, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) Deposit(opts *bind.TransactOpts, _account common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "deposit", _account, _token, _amount)
}

// Deposit is a paid mutator transaction binding the contract method 0x8340f549.
//
// Solidity: function deposit(address _account, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) Deposit(_account common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Deposit(&_SuccinctFeeVault.TransactOpts, _account, _token, _amount)
}

// Deposit is a paid mutator transaction binding the contract method 0x8340f549.
//
// Solidity: function deposit(address _account, address _token, uint256 _amount) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) Deposit(_account common.Address, _token common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Deposit(&_SuccinctFeeVault.TransactOpts, _account, _token, _amount)
}

// DepositNative is a paid mutator transaction binding the contract method 0x33bb7f91.
//
// Solidity: function depositNative(address _account) payable returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) DepositNative(opts *bind.TransactOpts, _account common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "depositNative", _account)
}

// DepositNative is a paid mutator transaction binding the contract method 0x33bb7f91.
//
// Solidity: function depositNative(address _account) payable returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) DepositNative(_account common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.DepositNative(&_SuccinctFeeVault.TransactOpts, _account)
}

// DepositNative is a paid mutator transaction binding the contract method 0x33bb7f91.
//
// Solidity: function depositNative(address _account) payable returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) DepositNative(_account common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.DepositNative(&_SuccinctFeeVault.TransactOpts, _account)
}

// RemoveDeductor is a paid mutator transaction binding the contract method 0xe5f8699d.
//
// Solidity: function removeDeductor(address _deductor) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) RemoveDeductor(opts *bind.TransactOpts, _deductor common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "removeDeductor", _deductor)
}

// RemoveDeductor is a paid mutator transaction binding the contract method 0xe5f8699d.
//
// Solidity: function removeDeductor(address _deductor) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) RemoveDeductor(_deductor common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.RemoveDeductor(&_SuccinctFeeVault.TransactOpts, _deductor)
}

// RemoveDeductor is a paid mutator transaction binding the contract method 0xe5f8699d.
//
// Solidity: function removeDeductor(address _deductor) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) RemoveDeductor(_deductor common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.RemoveDeductor(&_SuccinctFeeVault.TransactOpts, _deductor)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) RenounceOwnership() (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.RenounceOwnership(&_SuccinctFeeVault.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.RenounceOwnership(&_SuccinctFeeVault.TransactOpts)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.TransferOwnership(&_SuccinctFeeVault.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.TransferOwnership(&_SuccinctFeeVault.TransactOpts, newOwner)
}

// SuccinctFeeVaultCollectedIterator is returned from FilterCollected and is used to iterate over the raw logs and unpacked data for Collected events raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultCollectedIterator struct {
	Event *SuccinctFeeVaultCollected // Event containing the contract specifics and raw log

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
func (it *SuccinctFeeVaultCollectedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctFeeVaultCollected)
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
		it.Event = new(SuccinctFeeVaultCollected)
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
func (it *SuccinctFeeVaultCollectedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctFeeVaultCollectedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctFeeVaultCollected represents a Collected event raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultCollected struct {
	To     common.Address
	Token  common.Address
	Amount *big.Int
	Raw    types.Log // Blockchain specific contextual infos
}

// FilterCollected is a free log retrieval operation binding the contract event 0x484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd.
//
// Solidity: event Collected(address indexed to, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) FilterCollected(opts *bind.FilterOpts, to []common.Address, token []common.Address) (*SuccinctFeeVaultCollectedIterator, error) {

	var toRule []interface{}
	for _, toItem := range to {
		toRule = append(toRule, toItem)
	}
	var tokenRule []interface{}
	for _, tokenItem := range token {
		tokenRule = append(tokenRule, tokenItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.FilterLogs(opts, "Collected", toRule, tokenRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultCollectedIterator{contract: _SuccinctFeeVault.contract, event: "Collected", logs: logs, sub: sub}, nil
}

// WatchCollected is a free log subscription operation binding the contract event 0x484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd.
//
// Solidity: event Collected(address indexed to, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) WatchCollected(opts *bind.WatchOpts, sink chan<- *SuccinctFeeVaultCollected, to []common.Address, token []common.Address) (event.Subscription, error) {

	var toRule []interface{}
	for _, toItem := range to {
		toRule = append(toRule, toItem)
	}
	var tokenRule []interface{}
	for _, tokenItem := range token {
		tokenRule = append(tokenRule, tokenItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.WatchLogs(opts, "Collected", toRule, tokenRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctFeeVaultCollected)
				if err := _SuccinctFeeVault.contract.UnpackLog(event, "Collected", log); err != nil {
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

// ParseCollected is a log parse operation binding the contract event 0x484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd.
//
// Solidity: event Collected(address indexed to, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) ParseCollected(log types.Log) (*SuccinctFeeVaultCollected, error) {
	event := new(SuccinctFeeVaultCollected)
	if err := _SuccinctFeeVault.contract.UnpackLog(event, "Collected", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctFeeVaultDeductedIterator is returned from FilterDeducted and is used to iterate over the raw logs and unpacked data for Deducted events raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultDeductedIterator struct {
	Event *SuccinctFeeVaultDeducted // Event containing the contract specifics and raw log

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
func (it *SuccinctFeeVaultDeductedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctFeeVaultDeducted)
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
		it.Event = new(SuccinctFeeVaultDeducted)
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
func (it *SuccinctFeeVaultDeductedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctFeeVaultDeductedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctFeeVaultDeducted represents a Deducted event raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultDeducted struct {
	Account common.Address
	Token   common.Address
	Amount  *big.Int
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterDeducted is a free log retrieval operation binding the contract event 0x14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e45.
//
// Solidity: event Deducted(address indexed account, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) FilterDeducted(opts *bind.FilterOpts, account []common.Address, token []common.Address) (*SuccinctFeeVaultDeductedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var tokenRule []interface{}
	for _, tokenItem := range token {
		tokenRule = append(tokenRule, tokenItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.FilterLogs(opts, "Deducted", accountRule, tokenRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultDeductedIterator{contract: _SuccinctFeeVault.contract, event: "Deducted", logs: logs, sub: sub}, nil
}

// WatchDeducted is a free log subscription operation binding the contract event 0x14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e45.
//
// Solidity: event Deducted(address indexed account, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) WatchDeducted(opts *bind.WatchOpts, sink chan<- *SuccinctFeeVaultDeducted, account []common.Address, token []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var tokenRule []interface{}
	for _, tokenItem := range token {
		tokenRule = append(tokenRule, tokenItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.WatchLogs(opts, "Deducted", accountRule, tokenRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctFeeVaultDeducted)
				if err := _SuccinctFeeVault.contract.UnpackLog(event, "Deducted", log); err != nil {
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

// ParseDeducted is a log parse operation binding the contract event 0x14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e45.
//
// Solidity: event Deducted(address indexed account, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) ParseDeducted(log types.Log) (*SuccinctFeeVaultDeducted, error) {
	event := new(SuccinctFeeVaultDeducted)
	if err := _SuccinctFeeVault.contract.UnpackLog(event, "Deducted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctFeeVaultOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultOwnershipTransferredIterator struct {
	Event *SuccinctFeeVaultOwnershipTransferred // Event containing the contract specifics and raw log

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
func (it *SuccinctFeeVaultOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctFeeVaultOwnershipTransferred)
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
		it.Event = new(SuccinctFeeVaultOwnershipTransferred)
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
func (it *SuccinctFeeVaultOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctFeeVaultOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctFeeVaultOwnershipTransferred represents a OwnershipTransferred event raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*SuccinctFeeVaultOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultOwnershipTransferredIterator{contract: _SuccinctFeeVault.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *SuccinctFeeVaultOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctFeeVaultOwnershipTransferred)
				if err := _SuccinctFeeVault.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
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
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) ParseOwnershipTransferred(log types.Log) (*SuccinctFeeVaultOwnershipTransferred, error) {
	event := new(SuccinctFeeVaultOwnershipTransferred)
	if err := _SuccinctFeeVault.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// SuccinctFeeVaultReceivedIterator is returned from FilterReceived and is used to iterate over the raw logs and unpacked data for Received events raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultReceivedIterator struct {
	Event *SuccinctFeeVaultReceived // Event containing the contract specifics and raw log

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
func (it *SuccinctFeeVaultReceivedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctFeeVaultReceived)
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
		it.Event = new(SuccinctFeeVaultReceived)
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
func (it *SuccinctFeeVaultReceivedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctFeeVaultReceivedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctFeeVaultReceived represents a Received event raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultReceived struct {
	Account common.Address
	Token   common.Address
	Amount  *big.Int
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterReceived is a free log retrieval operation binding the contract event 0x8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb.
//
// Solidity: event Received(address indexed account, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) FilterReceived(opts *bind.FilterOpts, account []common.Address, token []common.Address) (*SuccinctFeeVaultReceivedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var tokenRule []interface{}
	for _, tokenItem := range token {
		tokenRule = append(tokenRule, tokenItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.FilterLogs(opts, "Received", accountRule, tokenRule)
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultReceivedIterator{contract: _SuccinctFeeVault.contract, event: "Received", logs: logs, sub: sub}, nil
}

// WatchReceived is a free log subscription operation binding the contract event 0x8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb.
//
// Solidity: event Received(address indexed account, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) WatchReceived(opts *bind.WatchOpts, sink chan<- *SuccinctFeeVaultReceived, account []common.Address, token []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}
	var tokenRule []interface{}
	for _, tokenItem := range token {
		tokenRule = append(tokenRule, tokenItem)
	}

	logs, sub, err := _SuccinctFeeVault.contract.WatchLogs(opts, "Received", accountRule, tokenRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctFeeVaultReceived)
				if err := _SuccinctFeeVault.contract.UnpackLog(event, "Received", log); err != nil {
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

// ParseReceived is a log parse operation binding the contract event 0x8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb.
//
// Solidity: event Received(address indexed account, address indexed token, uint256 amount)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) ParseReceived(log types.Log) (*SuccinctFeeVaultReceived, error) {
	event := new(SuccinctFeeVaultReceived)
	if err := _SuccinctFeeVault.contract.UnpackLog(event, "Received", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

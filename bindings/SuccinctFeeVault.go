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
	ABI: "[{\"type\":\"function\",\"name\":\"addDeductor\",\"inputs\":[{\"name\":\"_deductor\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"allowedDeductors\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"balances\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"collect\",\"inputs\":[{\"name\":\"_to\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_token\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"collectNative\",\"inputs\":[{\"name\":\"_to\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deduct\",\"inputs\":[{\"name\":\"_account\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_token\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deductNative\",\"inputs\":[{\"name\":\"_account\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deposit\",\"inputs\":[{\"name\":\"_account\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_token\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"depositNative\",\"inputs\":[{\"name\":\"_account\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"removeDeductor\",\"inputs\":[{\"name\":\"_deductor\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Collected\",\"inputs\":[{\"name\":\"to\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"token\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Deducted\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"token\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Received\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"token\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"FailedToSendNative\",\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InsufficentAllowance\",\"inputs\":[{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InsufficientBalance\",\"inputs\":[{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"type\":\"error\",\"name\":\"InvalidAccount\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"type\":\"error\",\"name\":\"InvalidToken\",\"inputs\":[{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"address\"}]},{\"type\":\"error\",\"name\":\"OnlyDeductor\",\"inputs\":[{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"}]}]",
	Bin: "0x608060405234801561001057600080fd5b50611208806100206000396000f3fe6080604052600436106100dd5760003560e01c8063c23f001f1161007f578063cf6ee7a011610059578063cf6ee7a01461023f578063e5f8699d1461025f578063ee3985b31461027f578063f2fde38b146102bf57600080fd5b8063c23f001f146101b9578063c4d66de8146101ff578063c8fea2fb1461021f57600080fd5b80638340f549116100bb5780638340f5491461012c5780638da5cb5b1461014c57806391ab273414610179578063929c51791461019957600080fd5b806333bb7f91146100e257806339e0273c146100f7578063715018a614610117575b600080fd5b6100f56100f0366004611027565b6102df565b005b34801561010357600080fd5b506100f5610112366004611027565b6103a6565b34801561012357600080fd5b506100f56103d2565b34801561013857600080fd5b506100f5610147366004611049565b6103e6565b34801561015857600080fd5b506033546040516001600160a01b0390911681526020015b60405180910390f35b34801561018557600080fd5b506100f5610194366004611085565b610599565b3480156101a557600080fd5b506100f56101b4366004611049565b61068a565b3480156101c557600080fd5b506101f16101d43660046110af565b606560209081526000928352604080842090915290825290205481565b604051908152602001610170565b34801561020b57600080fd5b506100f561021a366004611027565b6107fb565b34801561022b57600080fd5b506100f561023a366004611049565b61090e565b34801561024b57600080fd5b506100f561025a366004611085565b610a9c565b34801561026b57600080fd5b506100f561027a366004611027565b610bf3565b34801561028b57600080fd5b506102af61029a366004611027565b60666020526000908152604090205460ff1681565b6040519015158152602001610170565b3480156102cb57600080fd5b506100f56102da366004611027565b610c1c565b6001600160a01b038116610316576040516325abcd9160e11b81526001600160a01b03821660048201526024015b60405180910390fd5b6001600160a01b03811660009081527fffdfc1249c027f9191656349feb0761381bb32c9f557e01f419fd08754bf5a1b60205260408120805434929061035d9084906110f8565b90915550506040513481526000906001600160a01b038316907f8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb9060200160405180910390a350565b6103ae610c95565b6001600160a01b03166000908152606660205260409020805460ff19166001179055565b6103da610c95565b6103e46000610cef565b565b6001600160a01b038316610418576040516325abcd9160e11b81526001600160a01b038416600482015260240161030d565b6001600160a01b03821661044a5760405163961c9a4f60e01b81526001600160a01b038316600482015260240161030d565b604051636eb1769f60e11b815233600482015230602482015282906000906001600160a01b0383169063dd62ed3e90604401602060405180830381865afa158015610499573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104bd9190611111565b9050828110156104f25760405163a20569a160e01b81526001600160a01b03851660048201526024810184905260440161030d565b6105076001600160a01b038316333086610d41565b6001600160a01b0380851660009081526065602090815260408083209389168352929052908120805485929061053e9084906110f8565b92505081905550836001600160a01b0316856001600160a01b03167f8cabf31d2b1b11ba52dbb302817a3c9c83e4b2a5194d35121ab1354d69f6a4cb8560405161058a91815260200190565b60405180910390a35050505050565b6105a1610c95565b804710156105cc57604051633db7aa8160e21b8152600060048201526024810182905260440161030d565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114610619576040519150601f19603f3d011682016040523d82523d6000602084013e61061e565b606091505b50509050806106435760405163d23dc39360e01b81526004810183905260240161030d565b6040518281526000906001600160a01b038516907f484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd906020015b60405180910390a3505050565b3360009081526066602052604090205460ff166106bc5760405163f9434e0160e01b815233600482015260240161030d565b6001600160a01b0383166106ee576040516325abcd9160e11b81526001600160a01b038416600482015260240161030d565b6001600160a01b0382166107205760405163961c9a4f60e01b81526001600160a01b038316600482015260240161030d565b6001600160a01b0380831660009081526065602090815260408083209387168352929052205481111561077857604051633db7aa8160e21b81526001600160a01b03831660048201526024810182905260440161030d565b6001600160a01b038083166000908152606560209081526040808320938716835292905290812080548392906107af90849061112a565b92505081905550816001600160a01b0316836001600160a01b03167f14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e458360405161067d91815260200190565b600054610100900460ff161580801561081b5750600054600160ff909116105b806108355750303b158015610835575060005460ff166001145b6108985760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161030d565b6000805460ff1916600117905580156108bb576000805461ff0019166101001790555b6108c482610cef565b801561090a576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050565b610916610c95565b6001600160a01b0382166109485760405163961c9a4f60e01b81526001600160a01b038316600482015260240161030d565b6040516370a0823160e01b815230600482015281906001600160a01b038416906370a0823190602401602060405180830381865afa15801561098e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109b29190611111565b10156109e357604051633db7aa8160e21b81526001600160a01b03831660048201526024810182905260440161030d565b60405163a9059cbb60e01b81526001600160a01b0384811660048301526024820183905283169063a9059cbb906044016020604051808303816000875af1158015610a32573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a56919061113d565b50816001600160a01b0316836001600160a01b03167f484decdc1e9549e1866295f6f86c889ded3f7de410e7488a7a415978589dc8fd8360405161067d91815260200190565b3360009081526066602052604090205460ff16610ace5760405163f9434e0160e01b815233600482015260240161030d565b6001600160a01b038216610b00576040516325abcd9160e11b81526001600160a01b038316600482015260240161030d565b6001600160a01b03821660009081527fffdfc1249c027f9191656349feb0761381bb32c9f557e01f419fd08754bf5a1b6020526040902054811115610b6257604051633db7aa8160e21b8152600060048201526024810182905260440161030d565b6001600160a01b03821660009081527fffdfc1249c027f9191656349feb0761381bb32c9f557e01f419fd08754bf5a1b602052604081208054839290610ba990849061112a565b90915550506040518181526000906001600160a01b038416907f14b6918429f4c7e863473d3806b010ffac61ad5a849aea129db946f44d6b9e459060200160405180910390a35050565b610bfb610c95565b6001600160a01b03166000908152606660205260409020805460ff19169055565b610c24610c95565b6001600160a01b038116610c895760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161030d565b610c9281610cef565b50565b6033546001600160a01b031633146103e45760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161030d565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b604080516001600160a01b0385811660248301528416604482015260648082018490528251808303909101815260849091019091526020810180516001600160e01b03166323b872dd60e01b179052610d9b908590610da1565b50505050565b6000610df6826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316610e7b9092919063ffffffff16565b9050805160001480610e17575080806020019051810190610e17919061113d565b610e765760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b606482015260840161030d565b505050565b6060610e8a8484600085610e92565b949350505050565b606082471015610ef35760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b606482015260840161030d565b600080866001600160a01b03168587604051610f0f9190611183565b60006040518083038185875af1925050503d8060008114610f4c576040519150601f19603f3d011682016040523d82523d6000602084013e610f51565b606091505b5091509150610f6287838387610f6d565b979650505050505050565b60608315610fdc578251600003610fd5576001600160a01b0385163b610fd55760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161030d565b5081610e8a565b610e8a8383815115610ff15781518083602001fd5b8060405162461bcd60e51b815260040161030d919061119f565b80356001600160a01b038116811461102257600080fd5b919050565b60006020828403121561103957600080fd5b6110428261100b565b9392505050565b60008060006060848603121561105e57600080fd5b6110678461100b565b92506110756020850161100b565b9150604084013590509250925092565b6000806040838503121561109857600080fd5b6110a18361100b565b946020939093013593505050565b600080604083850312156110c257600080fd5b6110cb8361100b565b91506110d96020840161100b565b90509250929050565b634e487b7160e01b600052601160045260246000fd5b8082018082111561110b5761110b6110e2565b92915050565b60006020828403121561112357600080fd5b5051919050565b8181038181111561110b5761110b6110e2565b60006020828403121561114f57600080fd5b8151801515811461104257600080fd5b60005b8381101561117a578181015183820152602001611162565b50506000910152565b6000825161119581846020870161115f565b9190910192915050565b60208152600082518060208401526111be81604085016020870161115f565b601f01601f1916919091016040019291505056fea264697066735822122079116901ddbc96a4c7e57ffc4e72c1d4a01662c96dec7dc79b77800fb2ad9feb64736f6c63430008100033",
}

// SuccinctFeeVaultABI is the input ABI used to generate the binding from.
// Deprecated: Use SuccinctFeeVaultMetaData.ABI instead.
var SuccinctFeeVaultABI = SuccinctFeeVaultMetaData.ABI

// SuccinctFeeVaultBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use SuccinctFeeVaultMetaData.Bin instead.
var SuccinctFeeVaultBin = SuccinctFeeVaultMetaData.Bin

// DeploySuccinctFeeVault deploys a new Ethereum contract, binding an instance of SuccinctFeeVault to it.
func DeploySuccinctFeeVault(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *SuccinctFeeVault, error) {
	parsed, err := SuccinctFeeVaultMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(SuccinctFeeVaultBin), backend)
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

// Initialize is a paid mutator transaction binding the contract method 0xc4d66de8.
//
// Solidity: function initialize(address _owner) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactor) Initialize(opts *bind.TransactOpts, _owner common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.contract.Transact(opts, "initialize", _owner)
}

// Initialize is a paid mutator transaction binding the contract method 0xc4d66de8.
//
// Solidity: function initialize(address _owner) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultSession) Initialize(_owner common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Initialize(&_SuccinctFeeVault.TransactOpts, _owner)
}

// Initialize is a paid mutator transaction binding the contract method 0xc4d66de8.
//
// Solidity: function initialize(address _owner) returns()
func (_SuccinctFeeVault *SuccinctFeeVaultTransactorSession) Initialize(_owner common.Address) (*types.Transaction, error) {
	return _SuccinctFeeVault.Contract.Initialize(&_SuccinctFeeVault.TransactOpts, _owner)
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

// SuccinctFeeVaultInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultInitializedIterator struct {
	Event *SuccinctFeeVaultInitialized // Event containing the contract specifics and raw log

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
func (it *SuccinctFeeVaultInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SuccinctFeeVaultInitialized)
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
		it.Event = new(SuccinctFeeVaultInitialized)
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
func (it *SuccinctFeeVaultInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SuccinctFeeVaultInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SuccinctFeeVaultInitialized represents a Initialized event raised by the SuccinctFeeVault contract.
type SuccinctFeeVaultInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) FilterInitialized(opts *bind.FilterOpts) (*SuccinctFeeVaultInitializedIterator, error) {

	logs, sub, err := _SuccinctFeeVault.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &SuccinctFeeVaultInitializedIterator{contract: _SuccinctFeeVault.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *SuccinctFeeVaultInitialized) (event.Subscription, error) {

	logs, sub, err := _SuccinctFeeVault.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SuccinctFeeVaultInitialized)
				if err := _SuccinctFeeVault.contract.UnpackLog(event, "Initialized", log); err != nil {
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
func (_SuccinctFeeVault *SuccinctFeeVaultFilterer) ParseInitialized(log types.Log) (*SuccinctFeeVaultInitialized, error) {
	event := new(SuccinctFeeVaultInitialized)
	if err := _SuccinctFeeVault.contract.UnpackLog(event, "Initialized", log); err != nil {
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

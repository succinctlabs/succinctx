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

// StorageOracleMetaData contains all meta data concerning the StorageOracle contract.
var StorageOracleMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"name\":\"InvalidL1BlockHash\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidL1BlockNumber\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"NotFromFunctionGateway\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"OnlyGuardian\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"OnlyTimelock\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"storedBlockNumber\",\"type\":\"uint256\"}],\"name\":\"OutdatedBlockNumber\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"AdminChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\"}],\"name\":\"BeaconUpgraded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\"}],\"name\":\"Initialized\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\"}],\"name\":\"RoleAdminChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleGranted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleRevoked\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\"}],\"name\":\"SlotRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"value\",\"type\":\"bytes32\"}],\"name\":\"SlotUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\"}],\"name\":\"Upgraded\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"GUARDIAN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"L1_BLOCK\",\"outputs\":[{\"internalType\":\"contractL1BlockPrecompile\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"TIMELOCK_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"functionId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"gateway\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"grantRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_output\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"_context\",\"type\":\"bytes\"}],\"name\":\"handleStorageSlot\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_gateway\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_functionId\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"_timelock\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_guardian\",\"type\":\"address\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"renounceRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\"}],\"name\":\"requestStorageSlot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"revokeRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"slots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"value\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"}],\"name\":\"upgradeTo\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"upgradeToAndCall\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"}]",
	Bin: "0x60a06040523060805234801561001457600080fd5b5061001d610022565b6100e1565b600054610100900460ff161561008e5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116146100df576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b608051611a546101186000396000818161051001528181610550015281816105ef0152818161062f01526106be0152611a546000f3fe60806040526004361061011f5760003560e01c8063752e8f4f116100a0578063b5f95d9b11610064578063b5f95d9b14610330578063d547741f14610384578063f288a2e2146103a4578063f84e198f146103d8578063ffa1ad74146103ee57600080fd5b8063752e8f4f146102a857806391d14854146102c857806396c9f17e146102e8578063a217fddf14610308578063a25aaecb1461031d57600080fd5b806336568abe116100e757806336568abe146102255780633659cfe61461024557806347718590146102655780634f1ef2861461028057806352d1902d1461029357600080fd5b806301ffc9a714610124578063116191b614610159578063248a9ca31461019157806324ea54f4146101cf5780632f2ff15d14610203575b600080fd5b34801561013057600080fd5b5061014461013f366004611460565b610422565b60405190151581526020015b60405180910390f35b34801561016557600080fd5b5060fb54610179906001600160a01b031681565b6040516001600160a01b039091168152602001610150565b34801561019d57600080fd5b506101c16101ac36600461148a565b600090815260c9602052604090206001015490565b604051908152602001610150565b3480156101db57600080fd5b506101c17f55435dd261a4b9b3364963f7738a7a662ad9c84396d64be3365284bb7f0a504181565b34801561020f57600080fd5b5061022361021e3660046114b8565b610459565b005b34801561023157600080fd5b506102236102403660046114b8565b610483565b34801561025157600080fd5b506102236102603660046114e8565b610506565b34801561027157600080fd5b506101796015602160991b0181565b61022361028e3660046115a8565b6105e5565b34801561029f57600080fd5b506101c16106b1565b3480156102b457600080fd5b506102236102c33660046115f8565b610764565b3480156102d457600080fd5b506101446102e33660046114b8565b6108c6565b3480156102f457600080fd5b50610223610303366004611652565b6108f1565b34801561031457600080fd5b506101c1600081565b6101c161032b3660046116a5565b610a28565b34801561033c57600080fd5b5061036f61034b3660046116a5565b60fd6020908152600092835260408084209091529082529020805460019091015482565b60408051928352602083019190915201610150565b34801561039057600080fd5b5061022361039f3660046114b8565b610c79565b3480156103b057600080fd5b506101c17ff66846415d2bf9eabda9e84793ff9c0ea96d87f50fc41e66aa16469c6a442f0581565b3480156103e457600080fd5b506101c160fc5481565b3480156103fa57600080fd5b5060408051808201825260058152640312e302e360dc1b602082015290516101509190611721565b60006001600160e01b03198216637965db0b60e01b148061045357506301ffc9a760e01b6001600160e01b03198316145b92915050565b600082815260c9602052604090206001015461047481610c9e565b61047e8383610ca8565b505050565b6001600160a01b03811633146104f85760405162461bcd60e51b815260206004820152602f60248201527f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e636560448201526e103937b632b9903337b91039b2b63360891b60648201526084015b60405180910390fd5b6105028282610d2e565b5050565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016300361054e5760405162461bcd60e51b81526004016104ef90611734565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166105976000805160206119d8833981519152546001600160a01b031690565b6001600160a01b0316146105bd5760405162461bcd60e51b81526004016104ef90611780565b6105c681610d95565b604080516000808252602082019092526105e291839190610dde565b50565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016300361062d5760405162461bcd60e51b81526004016104ef90611734565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166106766000805160206119d8833981519152546001600160a01b031690565b6001600160a01b03161461069c5760405162461bcd60e51b81526004016104ef90611780565b6106a582610d95565b61050282826001610dde565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146107515760405162461bcd60e51b815260206004820152603860248201527f555550535570677261646561626c653a206d757374206e6f742062652063616c60448201527f6c6564207468726f7567682064656c656761746563616c6c000000000000000060648201526084016104ef565b506000805160206119d883398151915290565b60fb546001600160a01b0316331461079157604051630b6d7fef60e01b81523360048201526024016104ef565b6000828060200190518101906107a791906117cc565b90506000806000848060200190518101906107c291906117e5565b6001600160a01b038216600090815260fd6020908152604080832084845290915290206001015492955090935091508311610843576001600160a01b038216600090815260fd602090815260408083208484529091529081902060010154905163145befab60e11b81526104ef918591600401918252602082015260400190565b60408051808201825285815260208082018681526001600160a01b038616600081815260fd845285812087825284528590209351845590516001909301929092558251848152908101879052909185917ff851bb3a9206b7e3f9232a2c0ef6cafb1f9610caa3a3d539d23cecf109b4e81b910160405180910390a3505050505050565b600091825260c9602090815260408084206001600160a01b0393909316845291905290205460ff1690565b600054610100900460ff16158080156109115750600054600160ff909116105b8061092b5750303b15801561092b575060005460ff166001145b61098e5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016104ef565b6000805460ff1916600117905580156109b1576000805461ff0019166101001790555b60fb80546001600160a01b0319166001600160a01b03871617905560fc8490556109db8383610f49565b8015610a21576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b6000806015602160991b016001600160a01b03166309bd5a606040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a70573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a9491906117cc565b905080610ab35760405162fa512960e01b815260040160405180910390fd5b60006015602160991b016001600160a01b0316638381f58a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610afa573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b1e919061181e565b67ffffffffffffffff16905080600003610b4b57604051630fd8993960e21b815260040160405180910390fd5b60408051602081018490526001600160a01b0387811682840181905260608084018990528451808503909101815260808401855260a0840186905260c084019190915260e08084018990528451808503909101815261010084019485905260fb5460fc5463e2362c3160e01b90965291949093919092169163e2362c31913491610be591879063752e8f4f60e01b90889061010401611848565b60206040518083038185885af1158015610c03573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190610c2891906117cc565b9450866001600160a01b031684847f334b3e7009523fcf490673304121f8969e6a6ca251c7469340cd5a432aa5751089604051610c6791815260200190565b60405180910390a45050505092915050565b600082815260c96020526040902060010154610c9481610c9e565b61047e8383610d2e565b6105e28133610fdf565b610cb282826108c6565b61050257600082815260c9602090815260408083206001600160a01b03851684529091529020805460ff19166001179055610cea3390565b6001600160a01b0316816001600160a01b0316837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45050565b610d3882826108c6565b1561050257600082815260c9602090815260408083206001600160a01b0385168085529252808320805460ff1916905551339285917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a45050565b610dbf7ff66846415d2bf9eabda9e84793ff9c0ea96d87f50fc41e66aa16469c6a442f05336108c6565b6105e257604051636744392960e11b81523360048201526024016104ef565b7f4910fdfa16fed3260ed0e7147f7cc6da11a60208b5b9406d12a635614ffd91435460ff1615610e115761047e83611038565b826001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015610e6b575060408051601f3d908101601f19168201909252610e68918101906117cc565b60015b610ece5760405162461bcd60e51b815260206004820152602e60248201527f45524331393637557067726164653a206e657720696d706c656d656e7461746960448201526d6f6e206973206e6f74205555505360901b60648201526084016104ef565b6000805160206119d88339815191528114610f3d5760405162461bcd60e51b815260206004820152602960248201527f45524331393637557067726164653a20756e737570706f727465642070726f786044820152681a58589b195555525160ba1b60648201526084016104ef565b5061047e8383836110d4565b600054610100900460ff16610f705760405162461bcd60e51b81526004016104ef9061188e565b610f786110ff565b610f806110ff565b610f8b600083610ca8565b610fb57ff66846415d2bf9eabda9e84793ff9c0ea96d87f50fc41e66aa16469c6a442f0583610ca8565b6105027f55435dd261a4b9b3364963f7738a7a662ad9c84396d64be3365284bb7f0a504182610ca8565b610fe982826108c6565b61050257610ff681611128565b61100183602061113a565b6040516020016110129291906118d9565b60408051601f198184030181529082905262461bcd60e51b82526104ef91600401611721565b6001600160a01b0381163b6110a55760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084016104ef565b6000805160206119d883398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b6110dd836112dd565b6000825111806110ea5750805b1561047e576110f9838361131d565b50505050565b600054610100900460ff166111265760405162461bcd60e51b81526004016104ef9061188e565b565b60606104536001600160a01b03831660145b60606000611149836002611964565b61115490600261197b565b67ffffffffffffffff81111561116c5761116c611505565b6040519080825280601f01601f191660200182016040528015611196576020820181803683370190505b509050600360fc1b816000815181106111b1576111b161198e565b60200101906001600160f81b031916908160001a905350600f60fb1b816001815181106111e0576111e061198e565b60200101906001600160f81b031916908160001a9053506000611204846002611964565b61120f90600161197b565b90505b6001811115611287576f181899199a1a9b1b9c1cb0b131b232b360811b85600f16601081106112435761124361198e565b1a60f81b8282815181106112595761125961198e565b60200101906001600160f81b031916908160001a90535060049490941c93611280816119a4565b9050611212565b5083156112d65760405162461bcd60e51b815260206004820181905260248201527f537472696e67733a20686578206c656e67746820696e73756666696369656e7460448201526064016104ef565b9392505050565b6112e681611038565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606112d683836040518060600160405280602781526020016119f8602791396060600080856001600160a01b03168560405161135a91906119bb565b600060405180830381855af49150503d8060008114611395576040519150601f19603f3d011682016040523d82523d6000602084013e61139a565b606091505b50915091506113ab868383876113b5565b9695505050505050565b6060831561142457825160000361141d576001600160a01b0385163b61141d5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016104ef565b508161142e565b61142e8383611436565b949350505050565b8151156114465781518083602001fd5b8060405162461bcd60e51b81526004016104ef9190611721565b60006020828403121561147257600080fd5b81356001600160e01b0319811681146112d657600080fd5b60006020828403121561149c57600080fd5b5035919050565b6001600160a01b03811681146105e257600080fd5b600080604083850312156114cb57600080fd5b8235915060208301356114dd816114a3565b809150509250929050565b6000602082840312156114fa57600080fd5b81356112d6816114a3565b634e487b7160e01b600052604160045260246000fd5b600082601f83011261152c57600080fd5b813567ffffffffffffffff8082111561154757611547611505565b604051601f8301601f19908116603f0116810190828211818310171561156f5761156f611505565b8160405283815286602085880101111561158857600080fd5b836020870160208301376000602085830101528094505050505092915050565b600080604083850312156115bb57600080fd5b82356115c6816114a3565b9150602083013567ffffffffffffffff8111156115e257600080fd5b6115ee8582860161151b565b9150509250929050565b6000806040838503121561160b57600080fd5b823567ffffffffffffffff8082111561162357600080fd5b61162f8683870161151b565b9350602085013591508082111561164557600080fd5b506115ee8582860161151b565b6000806000806080858703121561166857600080fd5b8435611673816114a3565b935060208501359250604085013561168a816114a3565b9150606085013561169a816114a3565b939692955090935050565b600080604083850312156116b857600080fd5b82356116c3816114a3565b946020939093013593505050565b60005b838110156116ec5781810151838201526020016116d4565b50506000910152565b6000815180845261170d8160208601602086016116d1565b601f01601f19169290920160200192915050565b6020815260006112d660208301846116f5565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b19195b1959d85d1958d85b1b60a21b606082015260800190565b6020808252602c908201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060408201526b6163746976652070726f787960a01b606082015260800190565b6000602082840312156117de57600080fd5b5051919050565b6000806000606084860312156117fa57600080fd5b83519250602084015161180c816114a3565b80925050604084015190509250925092565b60006020828403121561183057600080fd5b815167ffffffffffffffff811681146112d657600080fd5b84815260806020820152600061186160808301866116f5565b6001600160e01b031985166040840152828103606084015261188381856116f5565b979650505050505050565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b7f416363657373436f6e74726f6c3a206163636f756e74200000000000000000008152600083516119118160178501602088016116d1565b7001034b99036b4b9b9b4b733903937b6329607d1b60179184019182015283516119428160288401602088016116d1565b01602801949350505050565b634e487b7160e01b600052601160045260246000fd5b80820281158282048414176104535761045361194e565b808201808211156104535761045361194e565b634e487b7160e01b600052603260045260246000fd5b6000816119b3576119b361194e565b506000190190565b600082516119cd8184602087016116d1565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a2646970667358221220445d3acdc5e6aafb16a9953a111ea8c518e5e2965ae32b296edbb7783894407c64736f6c63430008140033",
}

// StorageOracleABI is the input ABI used to generate the binding from.
// Deprecated: Use StorageOracleMetaData.ABI instead.
var StorageOracleABI = StorageOracleMetaData.ABI

// StorageOracleBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use StorageOracleMetaData.Bin instead.
var StorageOracleBin = StorageOracleMetaData.Bin

// DeployStorageOracle deploys a new Ethereum contract, binding an instance of StorageOracle to it.
func DeployStorageOracle(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *StorageOracle, error) {
	parsed, err := StorageOracleMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(StorageOracleBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &StorageOracle{StorageOracleCaller: StorageOracleCaller{contract: contract}, StorageOracleTransactor: StorageOracleTransactor{contract: contract}, StorageOracleFilterer: StorageOracleFilterer{contract: contract}}, nil
}

// StorageOracle is an auto generated Go binding around an Ethereum contract.
type StorageOracle struct {
	StorageOracleCaller     // Read-only binding to the contract
	StorageOracleTransactor // Write-only binding to the contract
	StorageOracleFilterer   // Log filterer for contract events
}

// StorageOracleCaller is an auto generated read-only Go binding around an Ethereum contract.
type StorageOracleCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// StorageOracleTransactor is an auto generated write-only Go binding around an Ethereum contract.
type StorageOracleTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// StorageOracleFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type StorageOracleFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// StorageOracleSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type StorageOracleSession struct {
	Contract     *StorageOracle    // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// StorageOracleCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type StorageOracleCallerSession struct {
	Contract *StorageOracleCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts        // Call options to use throughout this session
}

// StorageOracleTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type StorageOracleTransactorSession struct {
	Contract     *StorageOracleTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts        // Transaction auth options to use throughout this session
}

// StorageOracleRaw is an auto generated low-level Go binding around an Ethereum contract.
type StorageOracleRaw struct {
	Contract *StorageOracle // Generic contract binding to access the raw methods on
}

// StorageOracleCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type StorageOracleCallerRaw struct {
	Contract *StorageOracleCaller // Generic read-only contract binding to access the raw methods on
}

// StorageOracleTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type StorageOracleTransactorRaw struct {
	Contract *StorageOracleTransactor // Generic write-only contract binding to access the raw methods on
}

// NewStorageOracle creates a new instance of StorageOracle, bound to a specific deployed contract.
func NewStorageOracle(address common.Address, backend bind.ContractBackend) (*StorageOracle, error) {
	contract, err := bindStorageOracle(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &StorageOracle{StorageOracleCaller: StorageOracleCaller{contract: contract}, StorageOracleTransactor: StorageOracleTransactor{contract: contract}, StorageOracleFilterer: StorageOracleFilterer{contract: contract}}, nil
}

// NewStorageOracleCaller creates a new read-only instance of StorageOracle, bound to a specific deployed contract.
func NewStorageOracleCaller(address common.Address, caller bind.ContractCaller) (*StorageOracleCaller, error) {
	contract, err := bindStorageOracle(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &StorageOracleCaller{contract: contract}, nil
}

// NewStorageOracleTransactor creates a new write-only instance of StorageOracle, bound to a specific deployed contract.
func NewStorageOracleTransactor(address common.Address, transactor bind.ContractTransactor) (*StorageOracleTransactor, error) {
	contract, err := bindStorageOracle(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &StorageOracleTransactor{contract: contract}, nil
}

// NewStorageOracleFilterer creates a new log filterer instance of StorageOracle, bound to a specific deployed contract.
func NewStorageOracleFilterer(address common.Address, filterer bind.ContractFilterer) (*StorageOracleFilterer, error) {
	contract, err := bindStorageOracle(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &StorageOracleFilterer{contract: contract}, nil
}

// bindStorageOracle binds a generic wrapper to an already deployed contract.
func bindStorageOracle(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := StorageOracleMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_StorageOracle *StorageOracleRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _StorageOracle.Contract.StorageOracleCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_StorageOracle *StorageOracleRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _StorageOracle.Contract.StorageOracleTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_StorageOracle *StorageOracleRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _StorageOracle.Contract.StorageOracleTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_StorageOracle *StorageOracleCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _StorageOracle.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_StorageOracle *StorageOracleTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _StorageOracle.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_StorageOracle *StorageOracleTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _StorageOracle.Contract.contract.Transact(opts, method, params...)
}

// DEFAULTADMINROLE is a free data retrieval call binding the contract method 0xa217fddf.
//
// Solidity: function DEFAULT_ADMIN_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleCaller) DEFAULTADMINROLE(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "DEFAULT_ADMIN_ROLE")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// DEFAULTADMINROLE is a free data retrieval call binding the contract method 0xa217fddf.
//
// Solidity: function DEFAULT_ADMIN_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleSession) DEFAULTADMINROLE() ([32]byte, error) {
	return _StorageOracle.Contract.DEFAULTADMINROLE(&_StorageOracle.CallOpts)
}

// DEFAULTADMINROLE is a free data retrieval call binding the contract method 0xa217fddf.
//
// Solidity: function DEFAULT_ADMIN_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleCallerSession) DEFAULTADMINROLE() ([32]byte, error) {
	return _StorageOracle.Contract.DEFAULTADMINROLE(&_StorageOracle.CallOpts)
}

// GUARDIANROLE is a free data retrieval call binding the contract method 0x24ea54f4.
//
// Solidity: function GUARDIAN_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleCaller) GUARDIANROLE(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "GUARDIAN_ROLE")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GUARDIANROLE is a free data retrieval call binding the contract method 0x24ea54f4.
//
// Solidity: function GUARDIAN_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleSession) GUARDIANROLE() ([32]byte, error) {
	return _StorageOracle.Contract.GUARDIANROLE(&_StorageOracle.CallOpts)
}

// GUARDIANROLE is a free data retrieval call binding the contract method 0x24ea54f4.
//
// Solidity: function GUARDIAN_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleCallerSession) GUARDIANROLE() ([32]byte, error) {
	return _StorageOracle.Contract.GUARDIANROLE(&_StorageOracle.CallOpts)
}

// L1BLOCK is a free data retrieval call binding the contract method 0x47718590.
//
// Solidity: function L1_BLOCK() view returns(address)
func (_StorageOracle *StorageOracleCaller) L1BLOCK(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "L1_BLOCK")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// L1BLOCK is a free data retrieval call binding the contract method 0x47718590.
//
// Solidity: function L1_BLOCK() view returns(address)
func (_StorageOracle *StorageOracleSession) L1BLOCK() (common.Address, error) {
	return _StorageOracle.Contract.L1BLOCK(&_StorageOracle.CallOpts)
}

// L1BLOCK is a free data retrieval call binding the contract method 0x47718590.
//
// Solidity: function L1_BLOCK() view returns(address)
func (_StorageOracle *StorageOracleCallerSession) L1BLOCK() (common.Address, error) {
	return _StorageOracle.Contract.L1BLOCK(&_StorageOracle.CallOpts)
}

// TIMELOCKROLE is a free data retrieval call binding the contract method 0xf288a2e2.
//
// Solidity: function TIMELOCK_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleCaller) TIMELOCKROLE(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "TIMELOCK_ROLE")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// TIMELOCKROLE is a free data retrieval call binding the contract method 0xf288a2e2.
//
// Solidity: function TIMELOCK_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleSession) TIMELOCKROLE() ([32]byte, error) {
	return _StorageOracle.Contract.TIMELOCKROLE(&_StorageOracle.CallOpts)
}

// TIMELOCKROLE is a free data retrieval call binding the contract method 0xf288a2e2.
//
// Solidity: function TIMELOCK_ROLE() view returns(bytes32)
func (_StorageOracle *StorageOracleCallerSession) TIMELOCKROLE() ([32]byte, error) {
	return _StorageOracle.Contract.TIMELOCKROLE(&_StorageOracle.CallOpts)
}

// VERSION is a free data retrieval call binding the contract method 0xffa1ad74.
//
// Solidity: function VERSION() pure returns(string)
func (_StorageOracle *StorageOracleCaller) VERSION(opts *bind.CallOpts) (string, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "VERSION")

	if err != nil {
		return *new(string), err
	}

	out0 := *abi.ConvertType(out[0], new(string)).(*string)

	return out0, err

}

// VERSION is a free data retrieval call binding the contract method 0xffa1ad74.
//
// Solidity: function VERSION() pure returns(string)
func (_StorageOracle *StorageOracleSession) VERSION() (string, error) {
	return _StorageOracle.Contract.VERSION(&_StorageOracle.CallOpts)
}

// VERSION is a free data retrieval call binding the contract method 0xffa1ad74.
//
// Solidity: function VERSION() pure returns(string)
func (_StorageOracle *StorageOracleCallerSession) VERSION() (string, error) {
	return _StorageOracle.Contract.VERSION(&_StorageOracle.CallOpts)
}

// FunctionId is a free data retrieval call binding the contract method 0xf84e198f.
//
// Solidity: function functionId() view returns(bytes32)
func (_StorageOracle *StorageOracleCaller) FunctionId(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "functionId")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// FunctionId is a free data retrieval call binding the contract method 0xf84e198f.
//
// Solidity: function functionId() view returns(bytes32)
func (_StorageOracle *StorageOracleSession) FunctionId() ([32]byte, error) {
	return _StorageOracle.Contract.FunctionId(&_StorageOracle.CallOpts)
}

// FunctionId is a free data retrieval call binding the contract method 0xf84e198f.
//
// Solidity: function functionId() view returns(bytes32)
func (_StorageOracle *StorageOracleCallerSession) FunctionId() ([32]byte, error) {
	return _StorageOracle.Contract.FunctionId(&_StorageOracle.CallOpts)
}

// Gateway is a free data retrieval call binding the contract method 0x116191b6.
//
// Solidity: function gateway() view returns(address)
func (_StorageOracle *StorageOracleCaller) Gateway(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "gateway")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Gateway is a free data retrieval call binding the contract method 0x116191b6.
//
// Solidity: function gateway() view returns(address)
func (_StorageOracle *StorageOracleSession) Gateway() (common.Address, error) {
	return _StorageOracle.Contract.Gateway(&_StorageOracle.CallOpts)
}

// Gateway is a free data retrieval call binding the contract method 0x116191b6.
//
// Solidity: function gateway() view returns(address)
func (_StorageOracle *StorageOracleCallerSession) Gateway() (common.Address, error) {
	return _StorageOracle.Contract.Gateway(&_StorageOracle.CallOpts)
}

// GetRoleAdmin is a free data retrieval call binding the contract method 0x248a9ca3.
//
// Solidity: function getRoleAdmin(bytes32 role) view returns(bytes32)
func (_StorageOracle *StorageOracleCaller) GetRoleAdmin(opts *bind.CallOpts, role [32]byte) ([32]byte, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "getRoleAdmin", role)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetRoleAdmin is a free data retrieval call binding the contract method 0x248a9ca3.
//
// Solidity: function getRoleAdmin(bytes32 role) view returns(bytes32)
func (_StorageOracle *StorageOracleSession) GetRoleAdmin(role [32]byte) ([32]byte, error) {
	return _StorageOracle.Contract.GetRoleAdmin(&_StorageOracle.CallOpts, role)
}

// GetRoleAdmin is a free data retrieval call binding the contract method 0x248a9ca3.
//
// Solidity: function getRoleAdmin(bytes32 role) view returns(bytes32)
func (_StorageOracle *StorageOracleCallerSession) GetRoleAdmin(role [32]byte) ([32]byte, error) {
	return _StorageOracle.Contract.GetRoleAdmin(&_StorageOracle.CallOpts, role)
}

// HasRole is a free data retrieval call binding the contract method 0x91d14854.
//
// Solidity: function hasRole(bytes32 role, address account) view returns(bool)
func (_StorageOracle *StorageOracleCaller) HasRole(opts *bind.CallOpts, role [32]byte, account common.Address) (bool, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "hasRole", role, account)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// HasRole is a free data retrieval call binding the contract method 0x91d14854.
//
// Solidity: function hasRole(bytes32 role, address account) view returns(bool)
func (_StorageOracle *StorageOracleSession) HasRole(role [32]byte, account common.Address) (bool, error) {
	return _StorageOracle.Contract.HasRole(&_StorageOracle.CallOpts, role, account)
}

// HasRole is a free data retrieval call binding the contract method 0x91d14854.
//
// Solidity: function hasRole(bytes32 role, address account) view returns(bool)
func (_StorageOracle *StorageOracleCallerSession) HasRole(role [32]byte, account common.Address) (bool, error) {
	return _StorageOracle.Contract.HasRole(&_StorageOracle.CallOpts, role, account)
}

// ProxiableUUID is a free data retrieval call binding the contract method 0x52d1902d.
//
// Solidity: function proxiableUUID() view returns(bytes32)
func (_StorageOracle *StorageOracleCaller) ProxiableUUID(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "proxiableUUID")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// ProxiableUUID is a free data retrieval call binding the contract method 0x52d1902d.
//
// Solidity: function proxiableUUID() view returns(bytes32)
func (_StorageOracle *StorageOracleSession) ProxiableUUID() ([32]byte, error) {
	return _StorageOracle.Contract.ProxiableUUID(&_StorageOracle.CallOpts)
}

// ProxiableUUID is a free data retrieval call binding the contract method 0x52d1902d.
//
// Solidity: function proxiableUUID() view returns(bytes32)
func (_StorageOracle *StorageOracleCallerSession) ProxiableUUID() ([32]byte, error) {
	return _StorageOracle.Contract.ProxiableUUID(&_StorageOracle.CallOpts)
}

// Slots is a free data retrieval call binding the contract method 0xb5f95d9b.
//
// Solidity: function slots(address , uint256 ) view returns(bytes32 value, uint256 blockNumber)
func (_StorageOracle *StorageOracleCaller) Slots(opts *bind.CallOpts, arg0 common.Address, arg1 *big.Int) (struct {
	Value       [32]byte
	BlockNumber *big.Int
}, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "slots", arg0, arg1)

	outstruct := new(struct {
		Value       [32]byte
		BlockNumber *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.Value = *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)
	outstruct.BlockNumber = *abi.ConvertType(out[1], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// Slots is a free data retrieval call binding the contract method 0xb5f95d9b.
//
// Solidity: function slots(address , uint256 ) view returns(bytes32 value, uint256 blockNumber)
func (_StorageOracle *StorageOracleSession) Slots(arg0 common.Address, arg1 *big.Int) (struct {
	Value       [32]byte
	BlockNumber *big.Int
}, error) {
	return _StorageOracle.Contract.Slots(&_StorageOracle.CallOpts, arg0, arg1)
}

// Slots is a free data retrieval call binding the contract method 0xb5f95d9b.
//
// Solidity: function slots(address , uint256 ) view returns(bytes32 value, uint256 blockNumber)
func (_StorageOracle *StorageOracleCallerSession) Slots(arg0 common.Address, arg1 *big.Int) (struct {
	Value       [32]byte
	BlockNumber *big.Int
}, error) {
	return _StorageOracle.Contract.Slots(&_StorageOracle.CallOpts, arg0, arg1)
}

// SupportsInterface is a free data retrieval call binding the contract method 0x01ffc9a7.
//
// Solidity: function supportsInterface(bytes4 interfaceId) view returns(bool)
func (_StorageOracle *StorageOracleCaller) SupportsInterface(opts *bind.CallOpts, interfaceId [4]byte) (bool, error) {
	var out []interface{}
	err := _StorageOracle.contract.Call(opts, &out, "supportsInterface", interfaceId)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// SupportsInterface is a free data retrieval call binding the contract method 0x01ffc9a7.
//
// Solidity: function supportsInterface(bytes4 interfaceId) view returns(bool)
func (_StorageOracle *StorageOracleSession) SupportsInterface(interfaceId [4]byte) (bool, error) {
	return _StorageOracle.Contract.SupportsInterface(&_StorageOracle.CallOpts, interfaceId)
}

// SupportsInterface is a free data retrieval call binding the contract method 0x01ffc9a7.
//
// Solidity: function supportsInterface(bytes4 interfaceId) view returns(bool)
func (_StorageOracle *StorageOracleCallerSession) SupportsInterface(interfaceId [4]byte) (bool, error) {
	return _StorageOracle.Contract.SupportsInterface(&_StorageOracle.CallOpts, interfaceId)
}

// GrantRole is a paid mutator transaction binding the contract method 0x2f2ff15d.
//
// Solidity: function grantRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleTransactor) GrantRole(opts *bind.TransactOpts, role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "grantRole", role, account)
}

// GrantRole is a paid mutator transaction binding the contract method 0x2f2ff15d.
//
// Solidity: function grantRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleSession) GrantRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.GrantRole(&_StorageOracle.TransactOpts, role, account)
}

// GrantRole is a paid mutator transaction binding the contract method 0x2f2ff15d.
//
// Solidity: function grantRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleTransactorSession) GrantRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.GrantRole(&_StorageOracle.TransactOpts, role, account)
}

// HandleStorageSlot is a paid mutator transaction binding the contract method 0x752e8f4f.
//
// Solidity: function handleStorageSlot(bytes _output, bytes _context) returns()
func (_StorageOracle *StorageOracleTransactor) HandleStorageSlot(opts *bind.TransactOpts, _output []byte, _context []byte) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "handleStorageSlot", _output, _context)
}

// HandleStorageSlot is a paid mutator transaction binding the contract method 0x752e8f4f.
//
// Solidity: function handleStorageSlot(bytes _output, bytes _context) returns()
func (_StorageOracle *StorageOracleSession) HandleStorageSlot(_output []byte, _context []byte) (*types.Transaction, error) {
	return _StorageOracle.Contract.HandleStorageSlot(&_StorageOracle.TransactOpts, _output, _context)
}

// HandleStorageSlot is a paid mutator transaction binding the contract method 0x752e8f4f.
//
// Solidity: function handleStorageSlot(bytes _output, bytes _context) returns()
func (_StorageOracle *StorageOracleTransactorSession) HandleStorageSlot(_output []byte, _context []byte) (*types.Transaction, error) {
	return _StorageOracle.Contract.HandleStorageSlot(&_StorageOracle.TransactOpts, _output, _context)
}

// Initialize is a paid mutator transaction binding the contract method 0x96c9f17e.
//
// Solidity: function initialize(address _gateway, bytes32 _functionId, address _timelock, address _guardian) returns()
func (_StorageOracle *StorageOracleTransactor) Initialize(opts *bind.TransactOpts, _gateway common.Address, _functionId [32]byte, _timelock common.Address, _guardian common.Address) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "initialize", _gateway, _functionId, _timelock, _guardian)
}

// Initialize is a paid mutator transaction binding the contract method 0x96c9f17e.
//
// Solidity: function initialize(address _gateway, bytes32 _functionId, address _timelock, address _guardian) returns()
func (_StorageOracle *StorageOracleSession) Initialize(_gateway common.Address, _functionId [32]byte, _timelock common.Address, _guardian common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.Initialize(&_StorageOracle.TransactOpts, _gateway, _functionId, _timelock, _guardian)
}

// Initialize is a paid mutator transaction binding the contract method 0x96c9f17e.
//
// Solidity: function initialize(address _gateway, bytes32 _functionId, address _timelock, address _guardian) returns()
func (_StorageOracle *StorageOracleTransactorSession) Initialize(_gateway common.Address, _functionId [32]byte, _timelock common.Address, _guardian common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.Initialize(&_StorageOracle.TransactOpts, _gateway, _functionId, _timelock, _guardian)
}

// RenounceRole is a paid mutator transaction binding the contract method 0x36568abe.
//
// Solidity: function renounceRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleTransactor) RenounceRole(opts *bind.TransactOpts, role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "renounceRole", role, account)
}

// RenounceRole is a paid mutator transaction binding the contract method 0x36568abe.
//
// Solidity: function renounceRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleSession) RenounceRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.RenounceRole(&_StorageOracle.TransactOpts, role, account)
}

// RenounceRole is a paid mutator transaction binding the contract method 0x36568abe.
//
// Solidity: function renounceRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleTransactorSession) RenounceRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.RenounceRole(&_StorageOracle.TransactOpts, role, account)
}

// RequestStorageSlot is a paid mutator transaction binding the contract method 0xa25aaecb.
//
// Solidity: function requestStorageSlot(address _account, uint256 _slot) payable returns(bytes32 requestId)
func (_StorageOracle *StorageOracleTransactor) RequestStorageSlot(opts *bind.TransactOpts, _account common.Address, _slot *big.Int) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "requestStorageSlot", _account, _slot)
}

// RequestStorageSlot is a paid mutator transaction binding the contract method 0xa25aaecb.
//
// Solidity: function requestStorageSlot(address _account, uint256 _slot) payable returns(bytes32 requestId)
func (_StorageOracle *StorageOracleSession) RequestStorageSlot(_account common.Address, _slot *big.Int) (*types.Transaction, error) {
	return _StorageOracle.Contract.RequestStorageSlot(&_StorageOracle.TransactOpts, _account, _slot)
}

// RequestStorageSlot is a paid mutator transaction binding the contract method 0xa25aaecb.
//
// Solidity: function requestStorageSlot(address _account, uint256 _slot) payable returns(bytes32 requestId)
func (_StorageOracle *StorageOracleTransactorSession) RequestStorageSlot(_account common.Address, _slot *big.Int) (*types.Transaction, error) {
	return _StorageOracle.Contract.RequestStorageSlot(&_StorageOracle.TransactOpts, _account, _slot)
}

// RevokeRole is a paid mutator transaction binding the contract method 0xd547741f.
//
// Solidity: function revokeRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleTransactor) RevokeRole(opts *bind.TransactOpts, role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "revokeRole", role, account)
}

// RevokeRole is a paid mutator transaction binding the contract method 0xd547741f.
//
// Solidity: function revokeRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleSession) RevokeRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.RevokeRole(&_StorageOracle.TransactOpts, role, account)
}

// RevokeRole is a paid mutator transaction binding the contract method 0xd547741f.
//
// Solidity: function revokeRole(bytes32 role, address account) returns()
func (_StorageOracle *StorageOracleTransactorSession) RevokeRole(role [32]byte, account common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.RevokeRole(&_StorageOracle.TransactOpts, role, account)
}

// UpgradeTo is a paid mutator transaction binding the contract method 0x3659cfe6.
//
// Solidity: function upgradeTo(address newImplementation) returns()
func (_StorageOracle *StorageOracleTransactor) UpgradeTo(opts *bind.TransactOpts, newImplementation common.Address) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "upgradeTo", newImplementation)
}

// UpgradeTo is a paid mutator transaction binding the contract method 0x3659cfe6.
//
// Solidity: function upgradeTo(address newImplementation) returns()
func (_StorageOracle *StorageOracleSession) UpgradeTo(newImplementation common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.UpgradeTo(&_StorageOracle.TransactOpts, newImplementation)
}

// UpgradeTo is a paid mutator transaction binding the contract method 0x3659cfe6.
//
// Solidity: function upgradeTo(address newImplementation) returns()
func (_StorageOracle *StorageOracleTransactorSession) UpgradeTo(newImplementation common.Address) (*types.Transaction, error) {
	return _StorageOracle.Contract.UpgradeTo(&_StorageOracle.TransactOpts, newImplementation)
}

// UpgradeToAndCall is a paid mutator transaction binding the contract method 0x4f1ef286.
//
// Solidity: function upgradeToAndCall(address newImplementation, bytes data) payable returns()
func (_StorageOracle *StorageOracleTransactor) UpgradeToAndCall(opts *bind.TransactOpts, newImplementation common.Address, data []byte) (*types.Transaction, error) {
	return _StorageOracle.contract.Transact(opts, "upgradeToAndCall", newImplementation, data)
}

// UpgradeToAndCall is a paid mutator transaction binding the contract method 0x4f1ef286.
//
// Solidity: function upgradeToAndCall(address newImplementation, bytes data) payable returns()
func (_StorageOracle *StorageOracleSession) UpgradeToAndCall(newImplementation common.Address, data []byte) (*types.Transaction, error) {
	return _StorageOracle.Contract.UpgradeToAndCall(&_StorageOracle.TransactOpts, newImplementation, data)
}

// UpgradeToAndCall is a paid mutator transaction binding the contract method 0x4f1ef286.
//
// Solidity: function upgradeToAndCall(address newImplementation, bytes data) payable returns()
func (_StorageOracle *StorageOracleTransactorSession) UpgradeToAndCall(newImplementation common.Address, data []byte) (*types.Transaction, error) {
	return _StorageOracle.Contract.UpgradeToAndCall(&_StorageOracle.TransactOpts, newImplementation, data)
}

// StorageOracleAdminChangedIterator is returned from FilterAdminChanged and is used to iterate over the raw logs and unpacked data for AdminChanged events raised by the StorageOracle contract.
type StorageOracleAdminChangedIterator struct {
	Event *StorageOracleAdminChanged // Event containing the contract specifics and raw log

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
func (it *StorageOracleAdminChangedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleAdminChanged)
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
		it.Event = new(StorageOracleAdminChanged)
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
func (it *StorageOracleAdminChangedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleAdminChangedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleAdminChanged represents a AdminChanged event raised by the StorageOracle contract.
type StorageOracleAdminChanged struct {
	PreviousAdmin common.Address
	NewAdmin      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterAdminChanged is a free log retrieval operation binding the contract event 0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f.
//
// Solidity: event AdminChanged(address previousAdmin, address newAdmin)
func (_StorageOracle *StorageOracleFilterer) FilterAdminChanged(opts *bind.FilterOpts) (*StorageOracleAdminChangedIterator, error) {

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "AdminChanged")
	if err != nil {
		return nil, err
	}
	return &StorageOracleAdminChangedIterator{contract: _StorageOracle.contract, event: "AdminChanged", logs: logs, sub: sub}, nil
}

// WatchAdminChanged is a free log subscription operation binding the contract event 0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f.
//
// Solidity: event AdminChanged(address previousAdmin, address newAdmin)
func (_StorageOracle *StorageOracleFilterer) WatchAdminChanged(opts *bind.WatchOpts, sink chan<- *StorageOracleAdminChanged) (event.Subscription, error) {

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "AdminChanged")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleAdminChanged)
				if err := _StorageOracle.contract.UnpackLog(event, "AdminChanged", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseAdminChanged(log types.Log) (*StorageOracleAdminChanged, error) {
	event := new(StorageOracleAdminChanged)
	if err := _StorageOracle.contract.UnpackLog(event, "AdminChanged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleBeaconUpgradedIterator is returned from FilterBeaconUpgraded and is used to iterate over the raw logs and unpacked data for BeaconUpgraded events raised by the StorageOracle contract.
type StorageOracleBeaconUpgradedIterator struct {
	Event *StorageOracleBeaconUpgraded // Event containing the contract specifics and raw log

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
func (it *StorageOracleBeaconUpgradedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleBeaconUpgraded)
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
		it.Event = new(StorageOracleBeaconUpgraded)
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
func (it *StorageOracleBeaconUpgradedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleBeaconUpgradedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleBeaconUpgraded represents a BeaconUpgraded event raised by the StorageOracle contract.
type StorageOracleBeaconUpgraded struct {
	Beacon common.Address
	Raw    types.Log // Blockchain specific contextual infos
}

// FilterBeaconUpgraded is a free log retrieval operation binding the contract event 0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e.
//
// Solidity: event BeaconUpgraded(address indexed beacon)
func (_StorageOracle *StorageOracleFilterer) FilterBeaconUpgraded(opts *bind.FilterOpts, beacon []common.Address) (*StorageOracleBeaconUpgradedIterator, error) {

	var beaconRule []interface{}
	for _, beaconItem := range beacon {
		beaconRule = append(beaconRule, beaconItem)
	}

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "BeaconUpgraded", beaconRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleBeaconUpgradedIterator{contract: _StorageOracle.contract, event: "BeaconUpgraded", logs: logs, sub: sub}, nil
}

// WatchBeaconUpgraded is a free log subscription operation binding the contract event 0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e.
//
// Solidity: event BeaconUpgraded(address indexed beacon)
func (_StorageOracle *StorageOracleFilterer) WatchBeaconUpgraded(opts *bind.WatchOpts, sink chan<- *StorageOracleBeaconUpgraded, beacon []common.Address) (event.Subscription, error) {

	var beaconRule []interface{}
	for _, beaconItem := range beacon {
		beaconRule = append(beaconRule, beaconItem)
	}

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "BeaconUpgraded", beaconRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleBeaconUpgraded)
				if err := _StorageOracle.contract.UnpackLog(event, "BeaconUpgraded", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseBeaconUpgraded(log types.Log) (*StorageOracleBeaconUpgraded, error) {
	event := new(StorageOracleBeaconUpgraded)
	if err := _StorageOracle.contract.UnpackLog(event, "BeaconUpgraded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the StorageOracle contract.
type StorageOracleInitializedIterator struct {
	Event *StorageOracleInitialized // Event containing the contract specifics and raw log

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
func (it *StorageOracleInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleInitialized)
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
		it.Event = new(StorageOracleInitialized)
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
func (it *StorageOracleInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleInitialized represents a Initialized event raised by the StorageOracle contract.
type StorageOracleInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_StorageOracle *StorageOracleFilterer) FilterInitialized(opts *bind.FilterOpts) (*StorageOracleInitializedIterator, error) {

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &StorageOracleInitializedIterator{contract: _StorageOracle.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_StorageOracle *StorageOracleFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *StorageOracleInitialized) (event.Subscription, error) {

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleInitialized)
				if err := _StorageOracle.contract.UnpackLog(event, "Initialized", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseInitialized(log types.Log) (*StorageOracleInitialized, error) {
	event := new(StorageOracleInitialized)
	if err := _StorageOracle.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleRoleAdminChangedIterator is returned from FilterRoleAdminChanged and is used to iterate over the raw logs and unpacked data for RoleAdminChanged events raised by the StorageOracle contract.
type StorageOracleRoleAdminChangedIterator struct {
	Event *StorageOracleRoleAdminChanged // Event containing the contract specifics and raw log

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
func (it *StorageOracleRoleAdminChangedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleRoleAdminChanged)
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
		it.Event = new(StorageOracleRoleAdminChanged)
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
func (it *StorageOracleRoleAdminChangedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleRoleAdminChangedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleRoleAdminChanged represents a RoleAdminChanged event raised by the StorageOracle contract.
type StorageOracleRoleAdminChanged struct {
	Role              [32]byte
	PreviousAdminRole [32]byte
	NewAdminRole      [32]byte
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterRoleAdminChanged is a free log retrieval operation binding the contract event 0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff.
//
// Solidity: event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole)
func (_StorageOracle *StorageOracleFilterer) FilterRoleAdminChanged(opts *bind.FilterOpts, role [][32]byte, previousAdminRole [][32]byte, newAdminRole [][32]byte) (*StorageOracleRoleAdminChangedIterator, error) {

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

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "RoleAdminChanged", roleRule, previousAdminRoleRule, newAdminRoleRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleRoleAdminChangedIterator{contract: _StorageOracle.contract, event: "RoleAdminChanged", logs: logs, sub: sub}, nil
}

// WatchRoleAdminChanged is a free log subscription operation binding the contract event 0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff.
//
// Solidity: event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole)
func (_StorageOracle *StorageOracleFilterer) WatchRoleAdminChanged(opts *bind.WatchOpts, sink chan<- *StorageOracleRoleAdminChanged, role [][32]byte, previousAdminRole [][32]byte, newAdminRole [][32]byte) (event.Subscription, error) {

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

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "RoleAdminChanged", roleRule, previousAdminRoleRule, newAdminRoleRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleRoleAdminChanged)
				if err := _StorageOracle.contract.UnpackLog(event, "RoleAdminChanged", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseRoleAdminChanged(log types.Log) (*StorageOracleRoleAdminChanged, error) {
	event := new(StorageOracleRoleAdminChanged)
	if err := _StorageOracle.contract.UnpackLog(event, "RoleAdminChanged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleRoleGrantedIterator is returned from FilterRoleGranted and is used to iterate over the raw logs and unpacked data for RoleGranted events raised by the StorageOracle contract.
type StorageOracleRoleGrantedIterator struct {
	Event *StorageOracleRoleGranted // Event containing the contract specifics and raw log

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
func (it *StorageOracleRoleGrantedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleRoleGranted)
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
		it.Event = new(StorageOracleRoleGranted)
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
func (it *StorageOracleRoleGrantedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleRoleGrantedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleRoleGranted represents a RoleGranted event raised by the StorageOracle contract.
type StorageOracleRoleGranted struct {
	Role    [32]byte
	Account common.Address
	Sender  common.Address
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterRoleGranted is a free log retrieval operation binding the contract event 0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d.
//
// Solidity: event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)
func (_StorageOracle *StorageOracleFilterer) FilterRoleGranted(opts *bind.FilterOpts, role [][32]byte, account []common.Address, sender []common.Address) (*StorageOracleRoleGrantedIterator, error) {

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

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "RoleGranted", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleRoleGrantedIterator{contract: _StorageOracle.contract, event: "RoleGranted", logs: logs, sub: sub}, nil
}

// WatchRoleGranted is a free log subscription operation binding the contract event 0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d.
//
// Solidity: event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender)
func (_StorageOracle *StorageOracleFilterer) WatchRoleGranted(opts *bind.WatchOpts, sink chan<- *StorageOracleRoleGranted, role [][32]byte, account []common.Address, sender []common.Address) (event.Subscription, error) {

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

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "RoleGranted", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleRoleGranted)
				if err := _StorageOracle.contract.UnpackLog(event, "RoleGranted", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseRoleGranted(log types.Log) (*StorageOracleRoleGranted, error) {
	event := new(StorageOracleRoleGranted)
	if err := _StorageOracle.contract.UnpackLog(event, "RoleGranted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleRoleRevokedIterator is returned from FilterRoleRevoked and is used to iterate over the raw logs and unpacked data for RoleRevoked events raised by the StorageOracle contract.
type StorageOracleRoleRevokedIterator struct {
	Event *StorageOracleRoleRevoked // Event containing the contract specifics and raw log

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
func (it *StorageOracleRoleRevokedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleRoleRevoked)
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
		it.Event = new(StorageOracleRoleRevoked)
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
func (it *StorageOracleRoleRevokedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleRoleRevokedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleRoleRevoked represents a RoleRevoked event raised by the StorageOracle contract.
type StorageOracleRoleRevoked struct {
	Role    [32]byte
	Account common.Address
	Sender  common.Address
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterRoleRevoked is a free log retrieval operation binding the contract event 0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b.
//
// Solidity: event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)
func (_StorageOracle *StorageOracleFilterer) FilterRoleRevoked(opts *bind.FilterOpts, role [][32]byte, account []common.Address, sender []common.Address) (*StorageOracleRoleRevokedIterator, error) {

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

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "RoleRevoked", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleRoleRevokedIterator{contract: _StorageOracle.contract, event: "RoleRevoked", logs: logs, sub: sub}, nil
}

// WatchRoleRevoked is a free log subscription operation binding the contract event 0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b.
//
// Solidity: event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender)
func (_StorageOracle *StorageOracleFilterer) WatchRoleRevoked(opts *bind.WatchOpts, sink chan<- *StorageOracleRoleRevoked, role [][32]byte, account []common.Address, sender []common.Address) (event.Subscription, error) {

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

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "RoleRevoked", roleRule, accountRule, senderRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleRoleRevoked)
				if err := _StorageOracle.contract.UnpackLog(event, "RoleRevoked", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseRoleRevoked(log types.Log) (*StorageOracleRoleRevoked, error) {
	event := new(StorageOracleRoleRevoked)
	if err := _StorageOracle.contract.UnpackLog(event, "RoleRevoked", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleSlotRequestedIterator is returned from FilterSlotRequested and is used to iterate over the raw logs and unpacked data for SlotRequested events raised by the StorageOracle contract.
type StorageOracleSlotRequestedIterator struct {
	Event *StorageOracleSlotRequested // Event containing the contract specifics and raw log

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
func (it *StorageOracleSlotRequestedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleSlotRequested)
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
		it.Event = new(StorageOracleSlotRequested)
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
func (it *StorageOracleSlotRequestedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleSlotRequestedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleSlotRequested represents a SlotRequested event raised by the StorageOracle contract.
type StorageOracleSlotRequested struct {
	BlockNumber *big.Int
	BlockHash   [32]byte
	Account     common.Address
	Slot        *big.Int
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterSlotRequested is a free log retrieval operation binding the contract event 0x334b3e7009523fcf490673304121f8969e6a6ca251c7469340cd5a432aa57510.
//
// Solidity: event SlotRequested(uint256 indexed blockNumber, bytes32 indexed blockHash, address indexed account, uint256 slot)
func (_StorageOracle *StorageOracleFilterer) FilterSlotRequested(opts *bind.FilterOpts, blockNumber []*big.Int, blockHash [][32]byte, account []common.Address) (*StorageOracleSlotRequestedIterator, error) {

	var blockNumberRule []interface{}
	for _, blockNumberItem := range blockNumber {
		blockNumberRule = append(blockNumberRule, blockNumberItem)
	}
	var blockHashRule []interface{}
	for _, blockHashItem := range blockHash {
		blockHashRule = append(blockHashRule, blockHashItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "SlotRequested", blockNumberRule, blockHashRule, accountRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleSlotRequestedIterator{contract: _StorageOracle.contract, event: "SlotRequested", logs: logs, sub: sub}, nil
}

// WatchSlotRequested is a free log subscription operation binding the contract event 0x334b3e7009523fcf490673304121f8969e6a6ca251c7469340cd5a432aa57510.
//
// Solidity: event SlotRequested(uint256 indexed blockNumber, bytes32 indexed blockHash, address indexed account, uint256 slot)
func (_StorageOracle *StorageOracleFilterer) WatchSlotRequested(opts *bind.WatchOpts, sink chan<- *StorageOracleSlotRequested, blockNumber []*big.Int, blockHash [][32]byte, account []common.Address) (event.Subscription, error) {

	var blockNumberRule []interface{}
	for _, blockNumberItem := range blockNumber {
		blockNumberRule = append(blockNumberRule, blockNumberItem)
	}
	var blockHashRule []interface{}
	for _, blockHashItem := range blockHash {
		blockHashRule = append(blockHashRule, blockHashItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "SlotRequested", blockNumberRule, blockHashRule, accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleSlotRequested)
				if err := _StorageOracle.contract.UnpackLog(event, "SlotRequested", log); err != nil {
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

// ParseSlotRequested is a log parse operation binding the contract event 0x334b3e7009523fcf490673304121f8969e6a6ca251c7469340cd5a432aa57510.
//
// Solidity: event SlotRequested(uint256 indexed blockNumber, bytes32 indexed blockHash, address indexed account, uint256 slot)
func (_StorageOracle *StorageOracleFilterer) ParseSlotRequested(log types.Log) (*StorageOracleSlotRequested, error) {
	event := new(StorageOracleSlotRequested)
	if err := _StorageOracle.contract.UnpackLog(event, "SlotRequested", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleSlotUpdatedIterator is returned from FilterSlotUpdated and is used to iterate over the raw logs and unpacked data for SlotUpdated events raised by the StorageOracle contract.
type StorageOracleSlotUpdatedIterator struct {
	Event *StorageOracleSlotUpdated // Event containing the contract specifics and raw log

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
func (it *StorageOracleSlotUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleSlotUpdated)
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
		it.Event = new(StorageOracleSlotUpdated)
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
func (it *StorageOracleSlotUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleSlotUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleSlotUpdated represents a SlotUpdated event raised by the StorageOracle contract.
type StorageOracleSlotUpdated struct {
	BlockNumber *big.Int
	Account     common.Address
	Slot        *big.Int
	Value       [32]byte
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterSlotUpdated is a free log retrieval operation binding the contract event 0xf851bb3a9206b7e3f9232a2c0ef6cafb1f9610caa3a3d539d23cecf109b4e81b.
//
// Solidity: event SlotUpdated(uint256 indexed blockNumber, address indexed account, uint256 slot, bytes32 value)
func (_StorageOracle *StorageOracleFilterer) FilterSlotUpdated(opts *bind.FilterOpts, blockNumber []*big.Int, account []common.Address) (*StorageOracleSlotUpdatedIterator, error) {

	var blockNumberRule []interface{}
	for _, blockNumberItem := range blockNumber {
		blockNumberRule = append(blockNumberRule, blockNumberItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "SlotUpdated", blockNumberRule, accountRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleSlotUpdatedIterator{contract: _StorageOracle.contract, event: "SlotUpdated", logs: logs, sub: sub}, nil
}

// WatchSlotUpdated is a free log subscription operation binding the contract event 0xf851bb3a9206b7e3f9232a2c0ef6cafb1f9610caa3a3d539d23cecf109b4e81b.
//
// Solidity: event SlotUpdated(uint256 indexed blockNumber, address indexed account, uint256 slot, bytes32 value)
func (_StorageOracle *StorageOracleFilterer) WatchSlotUpdated(opts *bind.WatchOpts, sink chan<- *StorageOracleSlotUpdated, blockNumber []*big.Int, account []common.Address) (event.Subscription, error) {

	var blockNumberRule []interface{}
	for _, blockNumberItem := range blockNumber {
		blockNumberRule = append(blockNumberRule, blockNumberItem)
	}
	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "SlotUpdated", blockNumberRule, accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleSlotUpdated)
				if err := _StorageOracle.contract.UnpackLog(event, "SlotUpdated", log); err != nil {
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

// ParseSlotUpdated is a log parse operation binding the contract event 0xf851bb3a9206b7e3f9232a2c0ef6cafb1f9610caa3a3d539d23cecf109b4e81b.
//
// Solidity: event SlotUpdated(uint256 indexed blockNumber, address indexed account, uint256 slot, bytes32 value)
func (_StorageOracle *StorageOracleFilterer) ParseSlotUpdated(log types.Log) (*StorageOracleSlotUpdated, error) {
	event := new(StorageOracleSlotUpdated)
	if err := _StorageOracle.contract.UnpackLog(event, "SlotUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// StorageOracleUpgradedIterator is returned from FilterUpgraded and is used to iterate over the raw logs and unpacked data for Upgraded events raised by the StorageOracle contract.
type StorageOracleUpgradedIterator struct {
	Event *StorageOracleUpgraded // Event containing the contract specifics and raw log

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
func (it *StorageOracleUpgradedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(StorageOracleUpgraded)
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
		it.Event = new(StorageOracleUpgraded)
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
func (it *StorageOracleUpgradedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *StorageOracleUpgradedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// StorageOracleUpgraded represents a Upgraded event raised by the StorageOracle contract.
type StorageOracleUpgraded struct {
	Implementation common.Address
	Raw            types.Log // Blockchain specific contextual infos
}

// FilterUpgraded is a free log retrieval operation binding the contract event 0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b.
//
// Solidity: event Upgraded(address indexed implementation)
func (_StorageOracle *StorageOracleFilterer) FilterUpgraded(opts *bind.FilterOpts, implementation []common.Address) (*StorageOracleUpgradedIterator, error) {

	var implementationRule []interface{}
	for _, implementationItem := range implementation {
		implementationRule = append(implementationRule, implementationItem)
	}

	logs, sub, err := _StorageOracle.contract.FilterLogs(opts, "Upgraded", implementationRule)
	if err != nil {
		return nil, err
	}
	return &StorageOracleUpgradedIterator{contract: _StorageOracle.contract, event: "Upgraded", logs: logs, sub: sub}, nil
}

// WatchUpgraded is a free log subscription operation binding the contract event 0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b.
//
// Solidity: event Upgraded(address indexed implementation)
func (_StorageOracle *StorageOracleFilterer) WatchUpgraded(opts *bind.WatchOpts, sink chan<- *StorageOracleUpgraded, implementation []common.Address) (event.Subscription, error) {

	var implementationRule []interface{}
	for _, implementationItem := range implementation {
		implementationRule = append(implementationRule, implementationItem)
	}

	logs, sub, err := _StorageOracle.contract.WatchLogs(opts, "Upgraded", implementationRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(StorageOracleUpgraded)
				if err := _StorageOracle.contract.UnpackLog(event, "Upgraded", log); err != nil {
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
func (_StorageOracle *StorageOracleFilterer) ParseUpgraded(log types.Log) (*StorageOracleUpgraded, error) {
	event := new(StorageOracleUpgraded)
	if err := _StorageOracle.contract.UnpackLog(event, "Upgraded", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

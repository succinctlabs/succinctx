package system

type ProvingSystem interface {
	Compile() error
	Prove() error
	Verify() error
	Export() error
}

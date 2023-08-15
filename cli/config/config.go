package config

type PresetType string

const (
	PresetTypeGnark   PresetType = "gnark"
	PresetTypePlonky2 PresetType = "plonky2"
	PresetTypeCircom  PresetType = "circom"
	PresetTypeHalo2   PresetType = "halo2"
)

var AllPresets = []PresetType{
	PresetTypeGnark,
	// PresetTypePlonky2,
	// PresetTypeCircom,
	// PresetTypeHalo2,
}

type SuccinctConfig struct {
	Preset       PresetType `json:"preset"`
	BuildCommand string     `json:"build_command"`
	ProveCommand string     `json:"prove_command"`
}

var GnarkConfig = SuccinctConfig{
	Preset:       PresetTypeGnark,
	BuildCommand: "go build -o build/main ./circuit/ && ./build/main",
	ProveCommand: "./build/main -prove -input $INPUT",
}

// TODO: make default config for each preset
var DefaultConfigs = map[PresetType]SuccinctConfig{
	PresetTypeGnark: GnarkConfig,
}

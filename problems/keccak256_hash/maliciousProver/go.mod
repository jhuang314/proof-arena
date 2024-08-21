module github.com/PolyhedraZK/proof-arena/problems/keccak/maliciousProver

go 1.22.5

require (
	github.com/consensys/gnark v0.10.0
	github.com/consensys/gnark-crypto v0.12.2-0.20240215234832-d72fcb379d3e
	golang.org/x/crypto v0.17.0
)

require github.com/PolyhedraZK/proof-arena/SPJ/IPCUtils v0.0.0

replace github.com/PolyhedraZK/proof-arena/SPJ/IPCUtils => ../../../SPJ/IPCUtils

require (
	github.com/bits-and-blooms/bitset v1.8.0 // indirect
	github.com/blang/semver/v4 v4.0.0 // indirect
	github.com/consensys/bavard v0.1.13 // indirect
	github.com/davecgh/go-spew v1.1.1 // indirect
	github.com/fxamacker/cbor/v2 v2.5.0 // indirect
	github.com/google/pprof v0.0.0-20230817174616-7a8ec2ada47b // indirect
	github.com/ingonyama-zk/icicle v0.0.0-20230928131117-97f0079e5c71 // indirect
	github.com/ingonyama-zk/iciclegnark v0.1.0 // indirect
	github.com/mattn/go-colorable v0.1.13 // indirect
	github.com/mattn/go-isatty v0.0.19 // indirect
	github.com/mmcloughlin/addchain v0.4.0 // indirect
	github.com/pmezard/go-difflib v1.0.0 // indirect
	github.com/rs/zerolog v1.30.0 // indirect
	github.com/stretchr/testify v1.8.4 // indirect
	github.com/x448/float16 v0.8.4 // indirect
	golang.org/x/sys v0.15.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
	rsc.io/tmplfunc v0.0.3 // indirect
)

# Changelog

## [1.8.0](https://www.github.com/orthanq/orthanq/compare/v1.7.9...v1.8.0) (2025-01-17)


### Features

* use lp selected haplotypes in model and extend resulting table by 0 distance haplotypes ([#39](https://www.github.com/orthanq/orthanq/issues/39)) ([b7f0561](https://www.github.com/orthanq/orthanq/commit/b7f0561fac85f48982b527841694f2353a38aff0))

### [1.7.9](https://www.github.com/orthanq/orthanq/compare/v1.7.8...v1.7.9) (2024-10-07)


### Bug Fixes

* remove if-else block from the linear interpolation func ([#35](https://www.github.com/orthanq/orthanq/issues/35)) ([d0b4562](https://www.github.com/orthanq/orthanq/commit/d0b4562049ecc1458b0e75f5914d239572610a70))

### [1.7.8](https://www.github.com/orthanq/orthanq/compare/v1.7.7...v1.7.8) (2024-08-24)


### Bug Fixes

* handle allele name parsing from xml with and without HLA prefix ([#33](https://www.github.com/orthanq/orthanq/issues/33)) ([9bcf1f9](https://www.github.com/orthanq/orthanq/commit/9bcf1f9d271ff6cbc151b51163081d30b31b434a))

### [1.7.7](https://www.github.com/orthanq/orthanq/compare/v1.7.6...v1.7.7) (2024-08-23)


### Bug Fixes

* fix threshold for considered variants to long argument and lower threshold ([#31](https://www.github.com/orthanq/orthanq/issues/31)) ([6062f8a](https://www.github.com/orthanq/orthanq/commit/6062f8a58b7e9ccaab1e1c9ff76c9a36147e6eed))

### [1.7.6](https://www.github.com/orthanq/orthanq/compare/v1.7.5...v1.7.6) (2024-08-22)


### Bug Fixes

* fix equivalence class generation for hla and virus case separately; further, implement a flag for extension of haplotype list after linear program computation ([#29](https://www.github.com/orthanq/orthanq/issues/29)) ([cc7a92d](https://www.github.com/orthanq/orthanq/commit/cc7a92df98e6f04cb63d5bf6ac68914644792772))

### [1.7.5](https://www.github.com/orthanq/orthanq/compare/v1.7.4...v1.7.5) (2024-08-19)


### Bug Fixes

* implement user defined parameters threshold for equivalence classes and extending haplotype list; also properly adjust path of equivalence class graph ([#27](https://www.github.com/orthanq/orthanq/issues/27)) ([b9f8255](https://www.github.com/orthanq/orthanq/commit/b9f8255818360b16c76abaa41bc20e326a01f2c6))

### [1.7.4](https://www.github.com/orthanq/orthanq/compare/v1.7.3...v1.7.4) (2024-08-15)


### Bug Fixes

* fixate good_lp version and do not use wildcard constraint in cargo.toml ([68afd30](https://www.github.com/orthanq/orthanq/commit/68afd305236f85893fe68cec135c79d2ef19e983))
* fix scenario path for hla preprocessing

### [1.7.3](https://www.github.com/orthanq/orthanq/compare/v1.7.2...v1.7.3) (2024-08-14)


### Bug Fixes

* add description and license info to cargo.toml ([#23](https://www.github.com/orthanq/orthanq/issues/23)) ([a3036e9](https://www.github.com/orthanq/orthanq/commit/a3036e9b265c164fccc3ef1abd672de3feefe2e2))
* fix no such file or directory error by reading scenario to string and writing it to a temp.

### [1.7.2](https://www.github.com/orthanq/orthanq/compare/v1.7.1...v1.7.2) (2024-08-14)


### Bug Fixes

* perform interpolation of allele frequency distribution on the probability scale instead of PHRED scale; further, this changes AF distribution display to a bubble chart ([#21](https://www.github.com/orthanq/orthanq/issues/21)) ([309d33a](https://www.github.com/orthanq/orthanq/commit/309d33a9f2517b9e6362e693b666dedb667e4407))

### [1.7.1](https://www.github.com/orthanq/orthanq/compare/v1.7.0...v1.7.1) (2024-08-05)


### Bug Fixes

* fix preprocess hla outputs ([#19](https://www.github.com/orthanq/orthanq/issues/19)) ([34c78ae](https://www.github.com/orthanq/orthanq/commit/34c78aebfcdd7b77b2eb9d1a051ee45cb51e2dcd))

## [1.7.0](https://www.github.com/orthanq/orthanq/compare/v1.6.0...v1.7.0) (2024-07-17)


### Features

* implement generic virus application ([#17](https://www.github.com/orthanq/orthanq/issues/17)) ([05bfe4e](https://www.github.com/orthanq/orthanq/commit/05bfe4e1f1efc90247da2db1d94e9e5b60326925))

## [1.6.0](https://www.github.com/orthanq/orthanq/compare/v1.5.0...v1.6.0) (2024-07-08)


### Features

* set a configurable threshold that data has sufficient observation ([#15](https://www.github.com/orthanq/orthanq/issues/15)) ([09bea42](https://www.github.com/orthanq/orthanq/commit/09bea426a813e1231380a030115710b318f7a048))

## [1.5.0](https://www.github.com/orthanq/orthanq/compare/v1.4.0...v1.5.0) (2024-06-25)


### Features

* optimize model by pruning search space ([#8](https://www.github.com/orthanq/orthanq/issues/8)) ([d16cbe8](https://www.github.com/orthanq/orthanq/commit/d16cbe853f61e1ec9e517632a7453aae42f5ab97))

## [1.4.0](https://www.github.com/orthanq/orthanq/compare/v1.3.0...v1.4.0) (2024-06-20)


### Features

* download nextstrain resources and generate candidate variants from nuc conversions ([#12](https://www.github.com/orthanq/orthanq/issues/12)) ([f1ffda9](https://www.github.com/orthanq/orthanq/commit/f1ffda92f328d4078746b0b374b9e9dc93f8037e))

## [1.3.0](https://www.github.com/orthanq/orthanq/compare/v1.2.0...v1.3.0) (2024-04-12)


### Features

* breaking change: orthanq virus candidates: use the oldest submitted genome to represent each nextstrain in candidate haplotypes avoiding  irrelevant pango lineages ([96f397b](https://www.github.com/orthanq/orthanq/commit/96f397b0ba99f0f695b93b75303da3ed0d269b52))

## [1.2.0](https://www.github.com/orthanq/orthanq/compare/v1.1.0...v1.2.0) (2024-02-05)


### Features

* retrieve metadata and download data packages belonging to sarscov2 and use in candidate variant generation ([#6](https://www.github.com/orthanq/orthanq/issues/6)) ([dfce2a5](https://www.github.com/orthanq/orthanq/commit/dfce2a57d257e075765230b16cbaac1cd6d00ae7))

## [1.1.0](https://www.github.com/orthanq/orthanq/compare/v1.0.1...v1.1.0) (2024-01-05)


### Features

* enable viral quantification  ([#4](https://www.github.com/orthanq/orthanq/issues/4)) ([b945239](https://www.github.com/orthanq/orthanq/commit/b945239b81293f6520ad7cb0def6cbb235dca517))

### [1.0.1](https://www.github.com/orthanq/orthanq/compare/v1.0.0...v1.0.1) (2023-12-21)


### Miscellaneous Chores

* release 1.0.1 ([af15015](https://www.github.com/orthanq/orthanq/commit/af15015df8b83c89c74ab571d2dd61582355c980))

## 1.0.0 (2023-12-21)


### Features

* output 2-field info by summing up densitites of events that have identical first two fields ([dbd192c](https://www.github.com/orthanq/orthanq/commit/dbd192c3349deda41938d346028b3cfd617f8d40))


### Miscellaneous Chores

* release 1.0.0 ([827179b](https://www.github.com/orthanq/orthanq/commit/827179b814ce0246e4d72c2eb47da3fadbd402ee))

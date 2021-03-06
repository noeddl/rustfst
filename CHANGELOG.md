# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Add `FstIterator` and `FstIteratorMut` to iterate over states and arcs in a given FST without referencing the FST.
- Implement `FstIterator` and `FstIteratorMut` for ConstFst and VectorFst.

### Changed
- Make `KDELTA` public outside of the crate

## [0.4.0] - 2019-11-12

### Added
- Add `dfs` to the public interface in order to perform Depth First Search.
- Add `find_strongly_connected_components`  which is the implementation of the Tarjan's algoritm to find the strongly connect components in a directed graph.
- Add the `FstProperties` bitflags struct and a function to commpute the property flags for an `Fst`.
- Implement `topsort` and `statesort`.
- Add `BinaryDeserializer` trait. Should be used to parse an Fst in binary format.
- Implement `BinaryDeserializer` for VectorFst i.e VectorFst binary deserialization is now supported.
- Add `BinarySerializer` trait. Should bbe user to serialize an Fst in binary format compatible with OpenFST.
- Implement `BinarySerializer` for VectorFst i.e VectorFst binary serialization is now supported.
- Implement `Minimize` algorithm.
- Implement every queue types supported by OpenFST following the `Queue` trait :
    - `TrivialQueue`
    - `FifoQueue`
    - `LifoQueue`
    - `ShortestFirstQueue`
    - `TopOrderQueue`
    - `StateOrderQueue`
    - `SccQueue`
    - `AutoQueue`
    - `OtherQueue`
- Implement `ShortestDistance` algorithm.
- Implememt `ShortestPaths` algorithm.
- Add associated type `ReverseWeight` in the `Semiring` trait denoting the type of the weight reversed.
- Added the crate `rustfst-cli` as a CLI for the lib. It supports the subcommands:
    - `minimize` for the minimization.
    - `connect` for the connect algorithm.
    - `arcsort` for the arcsort algorithm.
    - `invert` for the invert algorithm.
    - `project` for the project algorithm.
    - `topsort` for the topsort algorithm.
    - `map` for the Map algorithm with several mappers.
    - `reverse` for the Reverse algorithm.
    - `shortestpath` for ShortestPath algorithm.
    - `rmfinalepsilon` for RmFinaleEpsilon algorithm.
    - `push` for Weights/Labels pushing algorithm.
- Added a prelude to `rustfst` to reduce the number of imports necessary when using the lib.
- Added a bench keyword the the rustfst-cli to be able to benchmark the running time of the different algorithms across multiple runs.
- Add a python package `rustfst-python-bench` to perform bench at the CLI level and at the functions level.
- Added lots of unchecked (thus unsafe) functions to `Fst` and `MutableFst` traits.
- Add shortespath to OpenFST benchmark.
- Implemented `push` function which supports weights and / or labels pushing.
- Added `take_value` method to the Semiring trait to extract a value from a Weight and take the ownership.
- Added `fst_convert` to the public API to convert one Fst object from one type to another type.
- Added `divide_assign` to `WeaklyDivisibleSemiring` semiring to perform in-place division.
- Added support for `ConstFst`. Also added a converter from `VectorFst` to `ConstFst` through the `From` trait.
- Added `final_weight_unnchecked_mut` to `MutableFst` trait.
- Added Code Coverage tests.
- Added support for binary format of `ConstFst`.

### Changed
- Before test cases were generated with pynini (python wrapper around openfst). Now they are directly generated with OpenFST (c++). Allows to test operations that are not wrapped.
- Test cases are now generated directly in the CI by buiding and running openfst which allows to avoid pushing data on the repo.
- Fix issue in FactorWeight algorithm when `factor_arc_weights` was toggle on.
- `reverse` function has now the same API as OpenFST returning an Fst of `ReverseWeight`s.
- `reverse` methods for every Semiring now returns `ReverseWeight`.
- Fix `reverse` of `GallicWeight`.
- Stopped using `arc_map` in invert's and project's implementation to run faster.
- Use `BufWriter` in the `BinarySerializer` of the `VectorFst` which improves a lot the serialization speed.
- Optimize `arcsort` function. Can no longer fail.
- Optimize `arcsum` function. Can no longer fail. Don't need to use an ArcMapper anymore.
- Optimize `invert` with unchecked functions.
- Optimize `project` with unchecked functions.
- Optimize `reverse`.
- Change implementation of `connect` algorithm which is now on par with OpenFST's one. Also works now for big fsts, no longer crashes.
- Move panic from `unwind` to `abort`.
- Change implementation of `TopSort` algorithm and `TopSortQueue`. Now uses `TopSortVisitor`.
- Use `SccVisitor` in `AutoQueue` implementation.
- Use `SccVisitor` in `rm_final_epsilon` implementation.
- Use `SccVisitor` in `compute_fst_properties` implementation.
- `value` methods of Semiring trait now returns a reference to the underlying weight.
- `final_weight` method of `Fst` trait now returns a reference to the final weight instead of a copy.
- `final_weight` method of `Fst` trait now returns a `Fallible`. Fail only when the state doesn't exist.
- `final_weight_mut` method of `Fst` trait now returns a `Fallible`. Fail only when the state doesn't exist.
- `FinalStateIterator` now returns reference to final weights instead of copy.
- Migrated to nom `5.0`.
### Removed
- Crate `rustfst-tests-openfst` has been removed and moved to the `rustfst` as unit tests. 
- Remove `state_map` and all `StateMappers`. Now use directly the functions `arc_sum` and `arc_unique`.

## [0.3.0] - 2019-04-03

### Added
- Add `arc_map` function to perform modifications that don't modify the number of arcs (and `ArcMapper` to specify how to modify the arcs).
- Implemented the following arc mappers to be used with `arc_map`:
   - `IdentityMapper`: Mapper that returns its input.
   - `InputEpsilonMapper`: Mapper that converts all input symbols to epsilon.
   - `InvertWeightMapper`: Mapper to inverse all non-Zero() weights.
   - `OutputEpsilonMapper`: Mapper that converts all output symbols to epsilon.
   - `PlusMapper`: Mapper to add a constant to all weights.
   - `QuantizeMapper`: Mapper to quantize all weights.
   - `RmWeightMapper`: Mapper to map all non-Zero() weights to One().
   - `TimesMapper`: Mapper to (right) multiply a constant to all weights.
- Add `final_weight_mut` to the `MutableFst` trait to retrieve a mutable reference on the final weight and implemented for `VectorFst`.
- `arc_map` method has been added to the MutableFst trait as a provided method.
- Add `num-traits` and `ordered-float` as dependency because hashing float is needed (yeah this is bad).
- Add struct `FinalArc` to correctly handle final weight when performing arc mapping.
- Add enum `MapFinalAction` to handle the creation of super final states when performing arc mapping.
- Implement `encode` and `decode` functions to allow the representation of a weighted transducer as a weighted automaton, an unweighted transducer or an unweighted automaton.
- Implement `rm_final_epsilon` which removes final states that have epsilon-only input arcs.
- Add `delete_final_weight` and `delete_arcs` to the MutableFst trait and implement them for `VectorFst`.
- Add `ArcFilter` trait and implement it for `AnyArcFilter`, `EpsilonArcFilter`, `InputEpsilonArcFilter` and `OutputEpsilonArcFilter`.
- Add `state_map` function to perform state modifications and `StateMapper` to specify the modifications.
- Implemented the following state mappers :
    - `ArcSumMapper`: Mapper that Plus-sum the arcs with same nextstate, ilabel and olabel.
    - `ArcUniqueMapper`: Remove duplicate arcs.
    - `IdentityStateMapper`: Return its input.
- Add `pop_arcs`, `reserve_arcs` and `reserve_state` to the `MutableFst` API.
- `state_map` method has been added to the MutableFst trait as a provided method.
- Implement `weight_convert` and `WeightConverter` to changed the Semiring of an FST.
- Implement WeightConverter trait for `IdentityArcMapper`, `InputEpsilonMapper`, `InvertWeightMapper`, `OutputEpsilonMapper`, `PlusMapper`, `QuantizeMapper`, `RmWeightMapper`, `TimesMapper`, 
- Add `arcsort` to sort the outgoing arcs of an FST.
- Add `ilabel_compare`, `olabel_compare` and `arc_compare` to compare two arcs. Can be used with `arcsort`.
- Add `GallicWeight` and `StringWeight`.
- Add `ProductWeight` : Weight W1 x W2
- Add `PowerWeight`: Weoght W ^ N
- Add `UnionWeight`
- Add `state_map` and `StateMapper`. Implement `StateMapper` for `ArcSumMapper` and `AddUniqueMapper`.
- Implement `WeightConverter` for `FromGallicMapper` and `ToGallicMapper`.
- Add `is_acceptor` to the FST trait to detect whether an FST is an acceptor.
- Implement `determinize` for both acceptors and transducers.

### Changed
- In the `Semiring` trait, `ONE` and `ZERO` associated constants have been removed in favor of `one` and `zero` functions.
- In the Semiring, `plus` and `times` methods now accept a `AsrRef<Self>` parameter.
- Added `inverse_assign`, `quantize_assign`, `plus_assign` and `times_assign` to the Semiring trait to perform in place operations.
- Changed `ArcMapper` trait to use `FinalArc` and `MapFinalAction`.
- Implement `invert` and `project` using `arc_map`.
- Change internal representation of float weights to use the `ordered-float` crate and derive `Hash` and `Eq` for each Semiring.
- `ArcSumMapper` is now called inside `rm_epsilon` function to conform with OpenFST.
- `plus` and `times` in Semiring now returns Fallible.
- `ArcMapper` returns Fallible.

### Removed
- Removed `Add`, `AddAssign`, `Mul`, `MulAssign` and `Copy` trait bound for `Semiring`.
- Removed custom `Result` type and replace it with `Fallible` from failure. Underneath, it is the same type.

## [0.2.0] - 2019-01-07
### Added
- `write_text` function to serialize an `ExpandedFst` into text format (compatible with OpenFST).
- `draw` to generate a file representing an `ExpandedFst` in dot format that can be displayed with GraphViz.
- Implement `num_input_epsilons`, `num_output_expilons` and `relabel_pairs`.
- `closure_plus` and `closure_star` are now also available as provided methods on a `MutabeFst`.
- Add `is_one` and `is_zero` to `Semiring` API.
- Add `is_start` to `Fst` API.
- Add `text` method to write the text representation of an fst into a string.
- Add `read_text` and `from_text_string` methods to parse a text representation of an FST.
- Implement weight pushing algorithms to either start state or final states (function `push_weights`).
- Implement `reverse` operation of an FST.
- Implement `reweight` operation to modify the weights of an FST according to some potentials.
- Implement `isomorphic` operation to compare two FSTs without depending on the ordering of the states or the arcs.
- Implement a `SymbolTable` data structure to handle the mapping symbol (string) / label (int) in a FST.
- Add `symt!` macro to quickly create a `SymbolTable` from a list of strings.
- Add `fst!` macro to quickly create an acceptor or a transducer from a list of labels.
- Migrate to edition 2018 of Rust.
- Add integration tests to compare the output of this crate directly with OpenFST (by using the pynini python wrapper).
- Add weight quantization for f32 Semiring and use it in the PartialEq trait.
- Add `fst_path!` macro to easily create a FstPath object.

### Changed
- `new` method now present in the `Semiring` trait.
- In all the APIs, `StateId` are now passed by value instead of reference as it is more optimized.
- `acceptor`, `transducer`, `inverse`, `project`, `closure_plus`, `closure_star` functions can't fail anymore (no Result in API).
- Fix multiple issues when parsing an FST in text format.
- The `num_arcs` function in the `Fst` trait now computes the number of arcs leaving a specific state instead of the number of arcs in the graph.
- `acceptor` and `transducer` methods now take slices and weight instead of only iterator of labels.
- Rename `Path` object to `FstPath` to avoid conflicts with the one from the standard library.

### Removed
- `determinize` no longer public as the implementation is not satisfactory.
- In the `Semiring` trait, `one` and `zero` functions have been removed in favor of `ONE` and `ZERO` which are defined as associated constants.
- Removed `project_input` and `project_output` functions in favor of `project` with a type parameters.

## [0.1.7] - 2018-10-23
### Added
- First released version of rustfst

[Unreleased]: https://github.com/garvys/rustfst/compare/0.4.0...HEAD
[0.4.0]: https://github.com/garvys/rustfst/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/garvys/rustfst/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/garvys/rustfst/compare/0.1.7...0.2.0
[0.1.7]: https://github.com/garvys/rustfst/compare/0.1.2...0.1.7

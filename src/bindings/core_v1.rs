pub use core_v1::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod core_v1 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_manager"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addScenario"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addScenario"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("scenario"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Scenario"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index_id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkActionRegister"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkActionRegister",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSourceRegister"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkSourceRegister",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeScenario"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeScenario"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("scriptIndex"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCounter"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCounter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getExecutionPausedState"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getExecutionPausedState",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getScenario"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getScenario"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Scenario"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getScenariosByOwner"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getScenariosByOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("m_scenarios"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                                ::std::boxed::Box::new(
                                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                                        ::std::vec![
                                                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                                            ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                                                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                                                        ],
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                                                ::std::boxed::Box::new(
                                                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                                                        ::std::vec![
                                                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                                                                        ],
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ScenarioWrapper[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getScenariosIdsByOwner"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getScenariosIdsByOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("m_ids"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeScenario"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeScenario"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateExecutionPauseState"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateExecutionPauseState",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateRegister"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateRegister"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum RegisterType"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionStateUpdated"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionStateUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_old"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_new"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RegisterUpdated"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RegisterUpdated"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("t"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ScenarioAdded"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ScenarioAdded"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ScenarioExecuted"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ScenarioExecuted"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ScenarioRemoved"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ScenarioRemoved"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionExecutionError"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ActionExecutionError",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionPaused"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionPaused"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidActionExecutor"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidActionExecutor",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidActorAddress"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidActorAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInputAmount"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidInputAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInputTokenAddress"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInputTokenAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidScenarioExecutor"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidScenarioExecutor",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidScenarioFinalOutput"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidScenarioFinalOutput",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidScenarioId"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidScenarioId"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSource"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSource"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoValidSources"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoValidSources"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SourceValidationError"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SourceValidationError",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COREV1_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    pub struct CoreV1<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CoreV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CoreV1<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CoreV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CoreV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CoreV1)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> CoreV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    COREV1_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addScenario` (0x3b823a58) function
        pub fn add_scenario(
            &self,
            scenario: Scenario,
            index_id: u64,
        ) -> ::ethers_contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([59, 130, 58, 88], (scenario, index_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkActionRegister` (0x857efc1c) function
        pub fn check_action_register(
            &self,
            addr: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([133, 126, 252, 28], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSourceRegister` (0xb477ecb6) function
        pub fn check_source_register(
            &self,
            addr: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([180, 119, 236, 182], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeScenario` (0x255ca0e4) function
        pub fn execute_scenario(
            &self,
            owner: ::ethers_core::types::Address,
            id: u64,
            script_index: u8,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 92, 160, 228], (owner, id, script_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCounter` (0x8ada066e) function
        pub fn get_counter(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([138, 218, 6, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getExecutionPausedState` (0x464a586c) function
        pub fn get_execution_paused_state(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 74, 88, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getScenario` (0xc65e3dd6) function
        pub fn get_scenario(
            &self,
            id: u64,
        ) -> ::ethers_contract::builders::ContractCall<M, (bool, Scenario)> {
            self.0
                .method_hash([198, 94, 61, 214], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getScenariosByOwner` (0x2c85f2f2) function
        pub fn get_scenarios_by_owner(
            &self,
            owner: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ScenarioWrapper>,
        > {
            self.0
                .method_hash([44, 133, 242, 242], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getScenariosIdsByOwner` (0x88e1d433) function
        pub fn get_scenarios_ids_by_owner(
            &self,
            owner: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::vec::Vec<u64>> {
            self.0
                .method_hash([136, 225, 212, 51], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeScenario` (0x587cd8b2) function
        pub fn remove_scenario(
            &self,
            id: u64,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 124, 216, 178], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateExecutionPauseState` (0x1e12ba71) function
        pub fn update_execution_pause_state(
            &self,
            value: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 18, 186, 113], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateRegister` (0x52cc948f) function
        pub fn update_register(
            &self,
            t: u8,
            addr: ::ethers_core::types::Address,
            value: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 204, 148, 143], (t, addr, value))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ExecutionStateUpdated` event
        pub fn execution_state_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionStateUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RegisterUpdated` event
        pub fn register_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegisterUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ScenarioAdded` event
        pub fn scenario_added_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ScenarioAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ScenarioExecuted` event
        pub fn scenario_executed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ScenarioExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ScenarioRemoved` event
        pub fn scenario_removed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ScenarioRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, CoreV1Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for CoreV1<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActionExecutionError` with signature `ActionExecutionError(address)` and selector `0x8af3df02`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ActionExecutionError", abi = "ActionExecutionError(address)")]
    pub struct ActionExecutionError {
        pub value: ::ethers_core::types::Address,
    }
    ///Custom Error type `ExecutionPaused` with signature `ExecutionPaused()` and selector `0x692da15f`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ExecutionPaused", abi = "ExecutionPaused()")]
    pub struct ExecutionPaused;
    ///Custom Error type `InvalidActionExecutor` with signature `InvalidActionExecutor(address)` and selector `0x3750c75d`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidActionExecutor", abi = "InvalidActionExecutor(address)")]
    pub struct InvalidActionExecutor {
        pub value: ::ethers_core::types::Address,
    }
    ///Custom Error type `InvalidActorAddress` with signature `InvalidActorAddress()` and selector `0x70e45109`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidActorAddress", abi = "InvalidActorAddress()")]
    pub struct InvalidActorAddress;
    ///Custom Error type `InvalidInputAmount` with signature `InvalidInputAmount()` and selector `0x340dabef`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidInputAmount", abi = "InvalidInputAmount()")]
    pub struct InvalidInputAmount;
    ///Custom Error type `InvalidInputTokenAddress` with signature `InvalidInputTokenAddress()` and selector `0x42f1b2fc`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidInputTokenAddress", abi = "InvalidInputTokenAddress()")]
    pub struct InvalidInputTokenAddress;
    ///Custom Error type `InvalidScenarioExecutor` with signature `InvalidScenarioExecutor()` and selector `0x82362886`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidScenarioExecutor", abi = "InvalidScenarioExecutor()")]
    pub struct InvalidScenarioExecutor;
    ///Custom Error type `InvalidScenarioFinalOutput` with signature `InvalidScenarioFinalOutput()` and selector `0xfc310d37`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidScenarioFinalOutput",
        abi = "InvalidScenarioFinalOutput()"
    )]
    pub struct InvalidScenarioFinalOutput;
    ///Custom Error type `InvalidScenarioId` with signature `InvalidScenarioId()` and selector `0x06f63221`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidScenarioId", abi = "InvalidScenarioId()")]
    pub struct InvalidScenarioId;
    ///Custom Error type `InvalidSource` with signature `InvalidSource(address)` and selector `0xfccbaf55`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSource", abi = "InvalidSource(address)")]
    pub struct InvalidSource {
        pub value: ::ethers_core::types::Address,
    }
    ///Custom Error type `NoValidSources` with signature `NoValidSources()` and selector `0xc4a4e9a6`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoValidSources", abi = "NoValidSources()")]
    pub struct NoValidSources;
    ///Custom Error type `SourceValidationError` with signature `SourceValidationError(address)` and selector `0xc4fec1c9`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SourceValidationError", abi = "SourceValidationError(address)")]
    pub struct SourceValidationError {
        pub value: ::ethers_core::types::Address,
    }
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CoreV1Errors {
        ActionExecutionError(ActionExecutionError),
        ExecutionPaused(ExecutionPaused),
        InvalidActionExecutor(InvalidActionExecutor),
        InvalidActorAddress(InvalidActorAddress),
        InvalidInputAmount(InvalidInputAmount),
        InvalidInputTokenAddress(InvalidInputTokenAddress),
        InvalidScenarioExecutor(InvalidScenarioExecutor),
        InvalidScenarioFinalOutput(InvalidScenarioFinalOutput),
        InvalidScenarioId(InvalidScenarioId),
        InvalidSource(InvalidSource),
        NoValidSources(NoValidSources),
        SourceValidationError(SourceValidationError),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for CoreV1Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ActionExecutionError as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActionExecutionError(decoded));
            }
            if let Ok(decoded) = <ExecutionPaused as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecutionPaused(decoded));
            }
            if let Ok(decoded) = <InvalidActionExecutor as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidActionExecutor(decoded));
            }
            if let Ok(decoded) = <InvalidActorAddress as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidActorAddress(decoded));
            }
            if let Ok(decoded) = <InvalidInputAmount as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInputAmount(decoded));
            }
            if let Ok(decoded) = <InvalidInputTokenAddress as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInputTokenAddress(decoded));
            }
            if let Ok(decoded) = <InvalidScenarioExecutor as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidScenarioExecutor(decoded));
            }
            if let Ok(decoded) = <InvalidScenarioFinalOutput as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidScenarioFinalOutput(decoded));
            }
            if let Ok(decoded) = <InvalidScenarioId as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidScenarioId(decoded));
            }
            if let Ok(decoded) = <InvalidSource as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSource(decoded));
            }
            if let Ok(decoded) = <NoValidSources as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoValidSources(decoded));
            }
            if let Ok(decoded) = <SourceValidationError as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SourceValidationError(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for CoreV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActionExecutionError(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExecutionPaused(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidActionExecutor(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidActorAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInputAmount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInputTokenAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidScenarioExecutor(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidScenarioFinalOutput(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidScenarioId(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSource(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::NoValidSources(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SourceValidationError(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for CoreV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActionExecutionError as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExecutionPaused as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidActionExecutor as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidActorAddress as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInputAmount as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInputTokenAddress as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidScenarioExecutor as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidScenarioFinalOutput as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidScenarioId as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSource as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <NoValidSources as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SourceValidationError as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CoreV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActionExecutionError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidActionExecutor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidActorAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInputAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInputTokenAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidScenarioExecutor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidScenarioFinalOutput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidScenarioId(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSource(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoValidSources(element) => ::core::fmt::Display::fmt(element, f),
                Self::SourceValidationError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CoreV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActionExecutionError> for CoreV1Errors {
        fn from(value: ActionExecutionError) -> Self {
            Self::ActionExecutionError(value)
        }
    }
    impl ::core::convert::From<ExecutionPaused> for CoreV1Errors {
        fn from(value: ExecutionPaused) -> Self {
            Self::ExecutionPaused(value)
        }
    }
    impl ::core::convert::From<InvalidActionExecutor> for CoreV1Errors {
        fn from(value: InvalidActionExecutor) -> Self {
            Self::InvalidActionExecutor(value)
        }
    }
    impl ::core::convert::From<InvalidActorAddress> for CoreV1Errors {
        fn from(value: InvalidActorAddress) -> Self {
            Self::InvalidActorAddress(value)
        }
    }
    impl ::core::convert::From<InvalidInputAmount> for CoreV1Errors {
        fn from(value: InvalidInputAmount) -> Self {
            Self::InvalidInputAmount(value)
        }
    }
    impl ::core::convert::From<InvalidInputTokenAddress> for CoreV1Errors {
        fn from(value: InvalidInputTokenAddress) -> Self {
            Self::InvalidInputTokenAddress(value)
        }
    }
    impl ::core::convert::From<InvalidScenarioExecutor> for CoreV1Errors {
        fn from(value: InvalidScenarioExecutor) -> Self {
            Self::InvalidScenarioExecutor(value)
        }
    }
    impl ::core::convert::From<InvalidScenarioFinalOutput> for CoreV1Errors {
        fn from(value: InvalidScenarioFinalOutput) -> Self {
            Self::InvalidScenarioFinalOutput(value)
        }
    }
    impl ::core::convert::From<InvalidScenarioId> for CoreV1Errors {
        fn from(value: InvalidScenarioId) -> Self {
            Self::InvalidScenarioId(value)
        }
    }
    impl ::core::convert::From<InvalidSource> for CoreV1Errors {
        fn from(value: InvalidSource) -> Self {
            Self::InvalidSource(value)
        }
    }
    impl ::core::convert::From<NoValidSources> for CoreV1Errors {
        fn from(value: NoValidSources) -> Self {
            Self::NoValidSources(value)
        }
    }
    impl ::core::convert::From<SourceValidationError> for CoreV1Errors {
        fn from(value: SourceValidationError) -> Self {
            Self::SourceValidationError(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for CoreV1Errors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ExecutionStateUpdated", abi = "ExecutionStateUpdated(bool,bool)")]
    pub struct ExecutionStateUpdatedFilter {
        pub old: bool,
        #[ethevent(indexed)]
        pub new: bool,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RegisterUpdated", abi = "RegisterUpdated(uint8,address,bool)")]
    pub struct RegisterUpdatedFilter {
        #[ethevent(indexed)]
        pub t: u8,
        #[ethevent(indexed)]
        pub addr: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub value: bool,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ScenarioAdded",
        abi = "ScenarioAdded(address,uint64,uint256,uint64)"
    )]
    pub struct ScenarioAddedFilter(
        #[ethevent(indexed)]
        pub ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub u64,
        #[ethevent(indexed)]
        pub ::ethers_core::types::U256,
        pub u64,
    );
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ScenarioExecuted", abi = "ScenarioExecuted(address,uint64)")]
    pub struct ScenarioExecutedFilter(
        #[ethevent(indexed)]
        pub ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub u64,
    );
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ScenarioRemoved", abi = "ScenarioRemoved(address,uint64)")]
    pub struct ScenarioRemovedFilter(
        #[ethevent(indexed)]
        pub ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub u64,
    );
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CoreV1Events {
        ExecutionStateUpdatedFilter(ExecutionStateUpdatedFilter),
        RegisterUpdatedFilter(RegisterUpdatedFilter),
        ScenarioAddedFilter(ScenarioAddedFilter),
        ScenarioExecutedFilter(ScenarioExecutedFilter),
        ScenarioRemovedFilter(ScenarioRemovedFilter),
    }
    impl ::ethers_contract::EthLogDecode for CoreV1Events {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = ExecutionStateUpdatedFilter::decode_log(log) {
                return Ok(CoreV1Events::ExecutionStateUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RegisterUpdatedFilter::decode_log(log) {
                return Ok(CoreV1Events::RegisterUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ScenarioAddedFilter::decode_log(log) {
                return Ok(CoreV1Events::ScenarioAddedFilter(decoded));
            }
            if let Ok(decoded) = ScenarioExecutedFilter::decode_log(log) {
                return Ok(CoreV1Events::ScenarioExecutedFilter(decoded));
            }
            if let Ok(decoded) = ScenarioRemovedFilter::decode_log(log) {
                return Ok(CoreV1Events::ScenarioRemovedFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CoreV1Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecutionStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ScenarioAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ScenarioExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ScenarioRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ExecutionStateUpdatedFilter> for CoreV1Events {
        fn from(value: ExecutionStateUpdatedFilter) -> Self {
            Self::ExecutionStateUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RegisterUpdatedFilter> for CoreV1Events {
        fn from(value: RegisterUpdatedFilter) -> Self {
            Self::RegisterUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ScenarioAddedFilter> for CoreV1Events {
        fn from(value: ScenarioAddedFilter) -> Self {
            Self::ScenarioAddedFilter(value)
        }
    }
    impl ::core::convert::From<ScenarioExecutedFilter> for CoreV1Events {
        fn from(value: ScenarioExecutedFilter) -> Self {
            Self::ScenarioExecutedFilter(value)
        }
    }
    impl ::core::convert::From<ScenarioRemovedFilter> for CoreV1Events {
        fn from(value: ScenarioRemovedFilter) -> Self {
            Self::ScenarioRemovedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addScenario` function with signature `addScenario((address,address,address,uint256,(uint8,(address,uint8,uint8,bytes)[],(address,bytes)[])[]),uint64)` and selector `0x3b823a58`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "addScenario",
        abi = "addScenario((address,address,address,uint256,(uint8,(address,uint8,uint8,bytes)[],(address,bytes)[])[]),uint64)"
    )]
    pub struct AddScenarioCall {
        pub scenario: Scenario,
        pub index_id: u64,
    }
    ///Container type for all input parameters for the `checkActionRegister` function with signature `checkActionRegister(address)` and selector `0x857efc1c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "checkActionRegister", abi = "checkActionRegister(address)")]
    pub struct CheckActionRegisterCall {
        pub addr: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `checkSourceRegister` function with signature `checkSourceRegister(address)` and selector `0xb477ecb6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "checkSourceRegister", abi = "checkSourceRegister(address)")]
    pub struct CheckSourceRegisterCall {
        pub addr: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `executeScenario` function with signature `executeScenario(address,uint64,uint8)` and selector `0x255ca0e4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeScenario", abi = "executeScenario(address,uint64,uint8)")]
    pub struct ExecuteScenarioCall {
        pub owner: ::ethers_core::types::Address,
        pub id: u64,
        pub script_index: u8,
    }
    ///Container type for all input parameters for the `getCounter` function with signature `getCounter()` and selector `0x8ada066e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCounter", abi = "getCounter()")]
    pub struct GetCounterCall;
    ///Container type for all input parameters for the `getExecutionPausedState` function with signature `getExecutionPausedState()` and selector `0x464a586c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getExecutionPausedState", abi = "getExecutionPausedState()")]
    pub struct GetExecutionPausedStateCall;
    ///Container type for all input parameters for the `getScenario` function with signature `getScenario(uint64)` and selector `0xc65e3dd6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getScenario", abi = "getScenario(uint64)")]
    pub struct GetScenarioCall {
        pub id: u64,
    }
    ///Container type for all input parameters for the `getScenariosByOwner` function with signature `getScenariosByOwner(address)` and selector `0x2c85f2f2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getScenariosByOwner", abi = "getScenariosByOwner(address)")]
    pub struct GetScenariosByOwnerCall {
        pub owner: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `getScenariosIdsByOwner` function with signature `getScenariosIdsByOwner(address)` and selector `0x88e1d433`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getScenariosIdsByOwner", abi = "getScenariosIdsByOwner(address)")]
    pub struct GetScenariosIdsByOwnerCall {
        pub owner: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `removeScenario` function with signature `removeScenario(uint64)` and selector `0x587cd8b2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeScenario", abi = "removeScenario(uint64)")]
    pub struct RemoveScenarioCall {
        pub id: u64,
    }
    ///Container type for all input parameters for the `updateExecutionPauseState` function with signature `updateExecutionPauseState(bool)` and selector `0x1e12ba71`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updateExecutionPauseState",
        abi = "updateExecutionPauseState(bool)"
    )]
    pub struct UpdateExecutionPauseStateCall {
        pub value: bool,
    }
    ///Container type for all input parameters for the `updateRegister` function with signature `updateRegister(uint8,address,bool)` and selector `0x52cc948f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateRegister", abi = "updateRegister(uint8,address,bool)")]
    pub struct UpdateRegisterCall {
        pub t: u8,
        pub addr: ::ethers_core::types::Address,
        pub value: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CoreV1Calls {
        AddScenario(AddScenarioCall),
        CheckActionRegister(CheckActionRegisterCall),
        CheckSourceRegister(CheckSourceRegisterCall),
        ExecuteScenario(ExecuteScenarioCall),
        GetCounter(GetCounterCall),
        GetExecutionPausedState(GetExecutionPausedStateCall),
        GetScenario(GetScenarioCall),
        GetScenariosByOwner(GetScenariosByOwnerCall),
        GetScenariosIdsByOwner(GetScenariosIdsByOwnerCall),
        RemoveScenario(RemoveScenarioCall),
        UpdateExecutionPauseState(UpdateExecutionPauseStateCall),
        UpdateRegister(UpdateRegisterCall),
    }
    impl ::ethers_core::abi::AbiDecode for CoreV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddScenarioCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddScenario(decoded));
            }
            if let Ok(decoded) = <CheckActionRegisterCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckActionRegister(decoded));
            }
            if let Ok(decoded) = <CheckSourceRegisterCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSourceRegister(decoded));
            }
            if let Ok(decoded) = <ExecuteScenarioCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteScenario(decoded));
            }
            if let Ok(decoded) = <GetCounterCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCounter(decoded));
            }
            if let Ok(decoded) = <GetExecutionPausedStateCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetExecutionPausedState(decoded));
            }
            if let Ok(decoded) = <GetScenarioCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetScenario(decoded));
            }
            if let Ok(decoded) = <GetScenariosByOwnerCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetScenariosByOwner(decoded));
            }
            if let Ok(decoded) = <GetScenariosIdsByOwnerCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetScenariosIdsByOwner(decoded));
            }
            if let Ok(decoded) = <RemoveScenarioCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveScenario(decoded));
            }
            if let Ok(decoded) = <UpdateExecutionPauseStateCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateExecutionPauseState(decoded));
            }
            if let Ok(decoded) = <UpdateRegisterCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateRegister(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for CoreV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddScenario(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CheckActionRegister(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CheckSourceRegister(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteScenario(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetCounter(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetExecutionPausedState(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetScenario(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetScenariosByOwner(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetScenariosIdsByOwner(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RemoveScenario(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UpdateExecutionPauseState(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UpdateRegister(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CoreV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddScenario(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckActionRegister(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckSourceRegister(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteScenario(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetExecutionPausedState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetScenario(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetScenariosByOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetScenariosIdsByOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveScenario(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateExecutionPauseState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateRegister(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddScenarioCall> for CoreV1Calls {
        fn from(value: AddScenarioCall) -> Self {
            Self::AddScenario(value)
        }
    }
    impl ::core::convert::From<CheckActionRegisterCall> for CoreV1Calls {
        fn from(value: CheckActionRegisterCall) -> Self {
            Self::CheckActionRegister(value)
        }
    }
    impl ::core::convert::From<CheckSourceRegisterCall> for CoreV1Calls {
        fn from(value: CheckSourceRegisterCall) -> Self {
            Self::CheckSourceRegister(value)
        }
    }
    impl ::core::convert::From<ExecuteScenarioCall> for CoreV1Calls {
        fn from(value: ExecuteScenarioCall) -> Self {
            Self::ExecuteScenario(value)
        }
    }
    impl ::core::convert::From<GetCounterCall> for CoreV1Calls {
        fn from(value: GetCounterCall) -> Self {
            Self::GetCounter(value)
        }
    }
    impl ::core::convert::From<GetExecutionPausedStateCall> for CoreV1Calls {
        fn from(value: GetExecutionPausedStateCall) -> Self {
            Self::GetExecutionPausedState(value)
        }
    }
    impl ::core::convert::From<GetScenarioCall> for CoreV1Calls {
        fn from(value: GetScenarioCall) -> Self {
            Self::GetScenario(value)
        }
    }
    impl ::core::convert::From<GetScenariosByOwnerCall> for CoreV1Calls {
        fn from(value: GetScenariosByOwnerCall) -> Self {
            Self::GetScenariosByOwner(value)
        }
    }
    impl ::core::convert::From<GetScenariosIdsByOwnerCall> for CoreV1Calls {
        fn from(value: GetScenariosIdsByOwnerCall) -> Self {
            Self::GetScenariosIdsByOwner(value)
        }
    }
    impl ::core::convert::From<RemoveScenarioCall> for CoreV1Calls {
        fn from(value: RemoveScenarioCall) -> Self {
            Self::RemoveScenario(value)
        }
    }
    impl ::core::convert::From<UpdateExecutionPauseStateCall> for CoreV1Calls {
        fn from(value: UpdateExecutionPauseStateCall) -> Self {
            Self::UpdateExecutionPauseState(value)
        }
    }
    impl ::core::convert::From<UpdateRegisterCall> for CoreV1Calls {
        fn from(value: UpdateRegisterCall) -> Self {
            Self::UpdateRegister(value)
        }
    }
    ///Container type for all return fields from the `addScenario` function with signature `addScenario((address,address,address,uint256,(uint8,(address,uint8,uint8,bytes)[],(address,bytes)[])[]),uint64)` and selector `0x3b823a58`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AddScenarioReturn {
        pub id: u64,
    }
    ///Container type for all return fields from the `checkActionRegister` function with signature `checkActionRegister(address)` and selector `0x857efc1c`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckActionRegisterReturn(pub bool);
    ///Container type for all return fields from the `checkSourceRegister` function with signature `checkSourceRegister(address)` and selector `0xb477ecb6`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckSourceRegisterReturn(pub bool);
    ///Container type for all return fields from the `getCounter` function with signature `getCounter()` and selector `0x8ada066e`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetCounterReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getExecutionPausedState` function with signature `getExecutionPausedState()` and selector `0x464a586c`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetExecutionPausedStateReturn(pub bool);
    ///Container type for all return fields from the `getScenario` function with signature `getScenario(uint64)` and selector `0xc65e3dd6`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetScenarioReturn(pub bool, pub Scenario);
    ///Container type for all return fields from the `getScenariosByOwner` function with signature `getScenariosByOwner(address)` and selector `0x2c85f2f2`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetScenariosByOwnerReturn {
        pub m_scenarios: ::std::vec::Vec<ScenarioWrapper>,
    }
    ///Container type for all return fields from the `getScenariosIdsByOwner` function with signature `getScenariosIdsByOwner(address)` and selector `0x88e1d433`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetScenariosIdsByOwnerReturn {
        pub m_ids: ::std::vec::Vec<u64>,
    }
    ///`ActionData(address,bytes)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ActionData {
        pub executor: ::ethers_core::types::Address,
        pub input: ::ethers_core::types::Bytes,
    }
    ///`Scenario(address,address,address,uint256,(uint8,(address,uint8,uint8,bytes)[],(address,bytes)[])[])`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Scenario {
        pub owner: ::ethers_core::types::Address,
        pub actor: ::ethers_core::types::Address,
        pub input_token: ::ethers_core::types::Address,
        pub input_amount: ::ethers_core::types::U256,
        pub scripts: ::std::vec::Vec<Script>,
    }
    ///`ScenarioWrapper(uint64,(address,address,address,uint256,(uint8,(address,uint8,uint8,bytes)[],(address,bytes)[])[]))`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ScenarioWrapper {
        pub id: u64,
        pub scenario: Scenario,
    }
    ///`Script(uint8,(address,uint8,uint8,bytes)[],(address,bytes)[])`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Script {
        pub trigger_type: u8,
        pub sources_to_verify: ::std::vec::Vec<SourceData>,
        pub actions_chain: ::std::vec::Vec<ActionData>,
    }
    ///`SourceData(address,uint8,uint8,bytes)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SourceData {
        pub addr: ::ethers_core::types::Address,
        pub kind: u8,
        pub condition: u8,
        pub input: ::ethers_core::types::Bytes,
    }
}
